//! This module contains the bundle implementation of a tag [BTag]

use super::attributes::{InputFields, Value};
use super::listeners::ListenerRegistration;
use super::Apply;
use crate::dom_bundle::{insert_node, BNode, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::virtual_dom::{vtag::VTagInner, vtag::SVG_NAMESPACE, Attributes, Key, VTag};
use crate::NodeRef;
use gloo::console;
use gloo_utils::document;
use std::ops::DerefMut;
use std::{borrow::Cow, hint::unreachable_unchecked};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlTextAreaElement as TextAreaElement};

/// [BTag] fields that are specific to different [BTag] kinds.
/// Decreases the memory footprint of [BTag] by avoiding impossible field and value combinations.
#[derive(Debug)]
enum BTagInner {
    /// Fields specific to
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    Input(InputFields),
    /// Fields specific to
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    Textarea {
        /// Contains a value of an
        /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
        value: Value<TextAreaElement>,
    },
    /// Fields for all other kinds of [VTag]s
    Other {
        /// A tag of the element.
        tag: Cow<'static, str>,
        /// List of child nodes
        child_bundle: BNode,
    },
}

/// The bundle implementation to [VTag]
#[derive(Debug)]
pub struct BTag {
    /// [BTag] fields that are specific to different [BTag] kinds.
    inner: BTagInner,
    listeners: ListenerRegistration,
    /// A reference to the DOM [`Element`].
    reference: Element,
    /// A node reference used for DOM access in Component lifecycle methods
    node_ref: NodeRef,
    attributes: Attributes,
    key: Option<Key>,
}

impl DomBundle for BTag {
    fn detach(self, parent: &Element) {
        self.listeners.unregister();

        let node = self.reference;
        // recursively remove its children
        if let BTagInner::Other { child_bundle, .. } = self.inner {
            child_bundle.detach(&node);
        }
        if parent.remove_child(&node).is_err() {
            console::warn!("Node not found to remove VTag");
        }
        // It could be that the ref was already reused when rendering another element.
        // Only unset the ref it still belongs to our node
        if self.node_ref.get().as_ref() == Some(&node) {
            self.node_ref.set(None);
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        next_parent
            .insert_before(&self.reference, next_sibling.get().as_ref())
            .unwrap();
    }
}

impl Reconcilable for VTag {
    type Bundle = BTag;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let el = self.create_element(parent);
        let Self {
            listeners,
            attributes,
            node_ref,
            key,
            ..
        } = self;
        insert_node(&el, parent, next_sibling.get().as_ref());

        let attributes = attributes.apply(&el);
        let listeners = listeners.apply(&el);

        let inner = match self.inner {
            VTagInner::Input(f) => {
                let f = f.apply(el.unchecked_ref());
                BTagInner::Input(f)
            }
            VTagInner::Textarea { value } => {
                let value = value.apply(el.unchecked_ref());
                BTagInner::Textarea { value }
            }
            VTagInner::Other { children, tag } => {
                let (_, child_bundle) = children.attach(parent_scope, &el, NodeRef::default());
                BTagInner::Other {
                    child_bundle: child_bundle.into(),
                    tag,
                }
            }
        };
        node_ref.set(Some(el.clone().into()));
        (
            node_ref.clone(),
            BTag {
                inner,
                listeners,
                reference: el,
                attributes,
                key,
                node_ref,
            },
        )
    }

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        // This kind of branching patching routine reduces branch predictor misses and the need to
        // unpack the enums (including `Option`s) all the time, resulting in a more streamlined
        // patching flow
        let is_matching_tag = match bundle {
            BNode::BTag(ex) if self.key == ex.key => match (&self.inner, &ex.inner) {
                (VTagInner::Input(_), BTagInner::Input(_)) => true,
                (VTagInner::Textarea { .. }, BTagInner::Textarea { .. }) => true,
                (VTagInner::Other { tag: l, .. }, BTagInner::Other { tag: r, .. }) if l == r => {
                    true
                }
                _ => false,
            },
            _ => false,
        };
        // If the ancestor is a tag of the same type, don't recreate, keep the
        // old tag and update its attributes and children.
        let tag = if is_matching_tag {
            match bundle {
                BNode::BTag(a) => {
                    // Preserve the reference that already exists
                    a.deref_mut()
                }
                _ => unsafe { unreachable_unchecked() },
            }
        } else {
            return self.replace(parent_scope, parent, next_sibling, bundle);
        };

        let el = &tag.reference;
        self.attributes.apply_diff(el, &mut tag.attributes);
        self.listeners.apply_diff(el, &mut tag.listeners);

        match (self.inner, &mut tag.inner) {
            (VTagInner::Input(new), BTagInner::Input(old)) => {
                new.apply_diff(el.unchecked_ref(), old);
            }
            (VTagInner::Textarea { value: new }, BTagInner::Textarea { value: old }) => {
                new.apply_diff(el.unchecked_ref(), old);
            }
            (
                VTagInner::Other { children: new, .. },
                BTagInner::Other {
                    child_bundle: old, ..
                },
            ) => {
                new.reconcile(parent_scope, el, NodeRef::default(), old);
            }
            // Can not happen, because we checked for tag equability above
            _ => unsafe { unreachable_unchecked() },
        }

        tag.key = self.key;

        if self.node_ref != tag.node_ref && tag.node_ref.get().as_ref() == Some(el) {
            tag.node_ref.set(None);
        }
        if self.node_ref != tag.node_ref {
            tag.node_ref = self.node_ref;
            tag.node_ref.set(Some(el.clone().into()));
        }

        tag.node_ref.clone()
    }
}

impl VTag {
    fn create_element(&self, parent: &Element) -> Element {
        let tag = self.tag();
        if tag == "svg"
            || parent
                .namespace_uri()
                .map_or(false, |ns| ns == SVG_NAMESPACE)
        {
            let namespace = Some(SVG_NAMESPACE);
            document()
                .create_element_ns(namespace, tag)
                .expect("can't create namespaced element for vtag")
        } else {
            document()
                .create_element(tag)
                .expect("can't create element for vtag")
        }
    }
}

impl BTag {
    /// Get the key of the underlying tag
    pub(in crate::dom_bundle) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }

    #[cfg(test)]
    pub(super) fn reference(&self) -> &Element {
        &self.reference
    }

    #[cfg(test)]
    pub(super) fn children(&self) -> &[BNode] {
        match &self.inner {
            BTagInner::Other { child_bundle, .. } => match child_bundle {
                BNode::BList(blist) => blist,
                _ => unreachable!("should be blist"),
            },
            _ => &[],
        }
    }

    #[cfg(test)]
    pub(super) fn tag(&self) -> &str {
        match &self.inner {
            BTagInner::Input { .. } => "input",
            BTagInner::Textarea { .. } => "textarea",
            BTagInner::Other { tag, .. } => tag.as_ref(),
        }
    }
}
