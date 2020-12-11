//! This module contains the implementation of a virtual element node `VTag`.

use super::{
    Attributes, Key, Listener, Listeners, Patch, PositionalAttr, Transformer, VDiff, VList, VNode,
};
use crate::html::{AnyScope, NodeRef};
use log::warn;
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::rc::Rc;

use std::ops::Deref;

/// SVG namespace string used for creating svg elements
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// Default namespace for html elements
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Used to improve performance of runtime element checks
#[derive(Clone, Copy, Debug, PartialEq)]
enum ElementType {
    Input,
    Textarea,
    Button,
    Other,
}

impl ElementType {
    fn from_tag(tag: &str) -> Self {
        match tag.to_ascii_lowercase().as_str() {
            "input" => Self::Input,
            "textarea" => Self::Textarea,
            "button" => Self::Button,
            _ => Self::Other,
        }
    }
}

/// A type for a virtual
/// [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// representation.
#[derive(Debug)]
pub struct VTag {
    /// A tag of the element.
    pub(crate) tag: Cow<'static, str>,
    /// Type of element.
    element_type: ElementType,
    /// A reference to the DOM `Element`.
    pub reference: Option<Element>,
    /// List of attached listeners.
    pub listeners: Listeners,
    /// List of attributes.
    pub attributes: Attributes,
    /// List of children nodes
    pub children: VList,
    /// Contains a value of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    pub value: Option<String>,
    /// Contains
    /// [kind](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#Form_%3Cinput%3E_types)
    /// value of an `InputElement`.
    pub kind: Option<Cow<'static, str>>,
    /// Represents `checked` attribute of
    /// [input](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-checked).
    /// It exists to override standard behavior of `checked` attribute, because
    /// in original HTML it sets `defaultChecked` value of `InputElement`, but for reactive
    /// frameworks it's more useful to control `checked` value of an `InputElement`.
    pub checked: bool,
    /// A node reference used for DOM access in Component lifecycle methods
    pub node_ref: NodeRef,
    /// Keeps handler for attached listeners to have an opportunity to drop them later.
    captured: Vec<EventListener>,

    pub key: Option<Key>,
}

impl Clone for VTag {
    fn clone(&self) -> Self {
        VTag {
            tag: self.tag.clone(),
            element_type: self.element_type,
            reference: None,
            listeners: self.listeners.clone(),
            attributes: self.attributes.clone(),
            children: self.children.clone(),
            value: self.value.clone(),
            kind: self.kind.clone(),
            checked: self.checked,
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
            captured: Vec::new(),
        }
    }
}

impl VTag {
    /// Creates a new `VTag` instance with `tag` name (cannot be changed later in DOM).
    pub fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        let tag: Cow<'static, str> = tag.into();
        let element_type = ElementType::from_tag(&tag);
        VTag {
            tag,
            element_type,
            reference: None,
            attributes: Attributes::new(),
            listeners: Vec::new(),
            captured: Vec::new(),
            children: VList::new(),
            node_ref: NodeRef::default(),
            key: None,
            value: None,
            kind: None,
            // In HTML node `checked` attribute sets `defaultChecked` parameter,
            // but we use own field to control real `checked` parameter
            checked: false,
        }
    }

    /// Returns tag of an `Element`. In HTML tags are always uppercase.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.add_child(child);
    }

    /// Add multiple `VNode` children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        self.children.add_children(children);
    }

    /// Sets `value` for an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    pub fn set_value<T: ToString>(&mut self, value: &T) {
        self.value = Some(value.to_string());
    }

    /// Sets `kind` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// Same as set `type` attribute.
    pub fn set_kind(&mut self, value: impl Into<Cow<'static, str>>) {
        self.kind = Some(value.into());
    }

    /// Sets `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Not a value of node's attribute).
    pub fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }

    /// Pushes a key-value pair to the attributes without ensuring uniqueness.
    ///
    /// Adding multiple attributes with the same key will cause unexpected behaviour
    /// if the variant is `Attributes::Vec`.
    #[doc(hidden)]
    pub fn __macro_push_attribute(&mut self, key: &'static str, value: Cow<'static, str>) {
        match &mut self.attributes {
            Attributes::Vec(v) => v.push(PositionalAttr::new(key, value)),
            Attributes::IndexMap(m) => {
                m.insert(key, value);
            }
        }
    }

    /// Pushes a placeholder to the attributes to preserve alignment.
    /// This is only required for the `Attributes::Vec` variant.
    #[doc(hidden)]
    pub fn __macro_push_attribute_placeholder(&mut self, key: &'static str) {
        // only the `Vec` variant needs placeholders
        if let Attributes::Vec(v) = &mut self.attributes {
            v.push(PositionalAttr::new_placeholder(key));
        }
    }

    /// Adds a key-value pair to attributes
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `type/kind`, `value` and `checked`.
    pub fn add_attribute(&mut self, key: &'static str, value: impl Into<Cow<'static, str>>) {
        self.attributes
            .get_mut_index_map()
            .insert(key, value.into());
    }

    /// Sets attributes to a virtual node.
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `type/kind`, `value` and `checked`.
    pub fn set_attributes(&mut self, attrs: impl Into<Attributes>) {
        self.attributes = attrs.into();
    }

    /// Adds new listener to the node.
    /// It's boxed because we want to keep it in a single list.
    /// Later `Listener::attach` will attach an actual listener to a DOM node.
    pub fn add_listener(&mut self, listener: Rc<dyn Listener>) {
        self.listeners.push(listener);
    }

    /// Adds new listeners to the node.
    /// They are boxed because we want to keep them in a single list.
    /// Later `Listener::attach` will attach an actual listener to a DOM node.
    pub fn add_listeners(&mut self, listeners: Vec<Rc<dyn Listener>>) {
        self.listeners.extend(listeners);
    }

    /// Every render it removes all listeners and attach it back later
    /// TODO(#943): Compare references of handler to do listeners update better
    fn recreate_listeners(&mut self, ancestor: &mut Option<Box<Self>>) {
        if let Some(ancestor) = ancestor.as_mut() {
            ancestor.captured.clear();
        }

        let element = self.reference.clone().expect("element expected");

        for listener in self.listeners.drain(..) {
            let handle = listener.attach(&element);
            self.captured.push(handle);
        }
    }

    fn refresh_value(&mut self) {
        // Don't refresh value if the element is not controlled
        if self.value.is_none() {
            return;
        }

        if let Some(element) = self.reference.as_ref() {
            // if self.element_type == ElementType::Input {
            //     let input_el = InputElement::try_from(element.clone()).ok();
            //     // let input_el = cfg_match! {
            //     //     feature = "std_web" => InputElement::try_from(element.clone()).ok(),
            //     //     feature = "web_sys" => element.dyn_ref::<InputElement>(),
            //     // };
            //     if let Some(input) = input_el {
            //         // let current_value = cfg_match! {
            //         //     feature = "std_web" => input.raw_value(),
            //         //     feature = "web_sys" => input.value(),
            //         // };
            //         self.set_value(&current_value)
            //     }
            // } else if self.element_type == ElementType::Textarea {
            //     // let textarea_el = cfg_match! {
            //     //     feature = "std_web" => TextAreaElement::try_from(element.clone()).ok(),
            //     //     feature = "web_sys" => element.dyn_ref::<TextAreaElement>(),
            //     // };
            //     if let Some(tae) = textarea_el {
            //         let current_value = &tae.value();
            //         self.set_value(&current_value)
            //     }
            // }
        }
    }

    /// Compares new kind with ancestor and produces a patch to apply, if any
    fn diff_kind<'a>(&'a self, ancestor: &'a Option<Box<Self>>) -> Option<Patch<&'a str, ()>> {
        match (
            self.kind.as_ref(),
            ancestor.as_ref().and_then(|anc| anc.kind.as_ref()),
        ) {
            (Some(ref left), Some(ref right)) => {
                if left != right {
                    Some(Patch::Replace(&**left, ()))
                } else {
                    None
                }
            }
            (Some(ref left), None) => Some(Patch::Add(&**left, ())),
            (None, Some(right)) => Some(Patch::Remove(&**right)),
            (None, None) => None,
        }
    }

    /// Compares new value with ancestor and produces a patch to apply, if any
    fn diff_value<'a>(&'a self, ancestor: &'a Option<Box<Self>>) -> Option<Patch<&'a str, ()>> {
        match (
            self.value.as_ref(),
            ancestor.as_ref().and_then(|anc| anc.value.as_ref()),
        ) {
            (Some(ref left), Some(ref right)) => {
                if left != right {
                    Some(Patch::Replace(&**left, ()))
                } else {
                    None
                }
            }
            (Some(ref left), None) => Some(Patch::Add(&**left, ())),
            (None, Some(right)) => Some(Patch::Remove(&**right)),
            (None, None) => None,
        }
    }

    fn apply_diffs(&mut self, ancestor: &mut Option<Box<Self>>) {
        let changes = if let Some(old_attributes) = ancestor.as_mut().map(|a| &mut a.attributes) {
            Attributes::diff(&self.attributes, old_attributes)
        } else {
            self.attributes
                .iter()
                .map(|(k, v)| Patch::Add(k, v))
                .collect()
        };

        let element = self.reference.as_ref().expect("element expected");

        for change in changes {
            match change {
                Patch::Add(key, value) | Patch::Replace(key, value) => {
                    element
                        .set_attribute(&key, &value)
                        .expect("invalid attribute key");
                }
                Patch::Remove(key) => element
                    .remove_attribute(&key)
                    .expect("could not remove attribute"),
            }
        }

        // TODO: add std_web after https://github.com/koute/stdweb/issues/395 will be approved
        // Check this out: https://github.com/yewstack/yew/pull/1033/commits/4b4e958bb1ccac0524eb20f63f06ae394c20553d
        // #[cfg(feature = "web_sys")]
        // {
        if self.element_type == ElementType::Button {
            if let Some(button) = element.dyn_ref::<ButtonElement>() {
                if let Some(change) = self.diff_kind(ancestor) {
                    let kind = match change {
                        Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                        Patch::Remove(_) => "",
                    };
                    button.set_type(kind);
                }
            }
        }
        // }

        // `input` element has extra parameters to control
        // I override behavior of attributes to make it more clear
        // and useful in templates. For example I interpret `checked`
        // attribute as `checked` parameter, not `defaultChecked` as browsers do
        if self.element_type == ElementType::Input {
            if let Some(input) = element.dyn_ref::<InputElement>() {
                if let Some(change) = self.diff_kind(ancestor) {
                    let kind = match change {
                        Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                        Patch::Remove(_) => "",
                    };
                    input.set_type(kind)
                    // cfg_match! {
                    //     feature = "std_web" => ({
                    //         //https://github.com/koute/stdweb/commit/3b85c941db00b8e3c942624afd50c5929085fb08
                    //         //input.set_kind(&kind);
                    //         let input = &input;
                    //         js! { @(no_return)
                    //             @{input}.type = @{kind};
                    //         }
                    //     }),
                    //     feature = "web_sys" => input.set_type(kind),
                    //     feature = "static_render" => input.set_type(kind),
                    // }
                }

                if let Some(change) = self.diff_value(ancestor) {
                    let raw_value = match change {
                        Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                        Patch::Remove(_) => "",
                    };
                    input.set_value(raw_value);
                    // cfg_match! {
                    //     feature = "std_web" => input.set_raw_value(raw_value),
                    //     feature = "web_sys" => input.set_value(raw_value),
                    //     feature = "static_render" => input.set_type(raw_value),
                    // };
                }

                // IMPORTANT! This parameter has to be set every time
                // to prevent strange behaviour in the browser when the DOM changes
                set_checked(&input, self.checked);
            }
        } else if self.element_type == ElementType::Textarea {
            if let Some(tae) = {
                element.dyn_ref::<TextAreaElement>()
                // cfg_match! {
                //     feature = "std_web" => TextAreaElement::try_from(element.clone()).ok(),
                //     feature = "web_sys" => element.dyn_ref::<TextAreaElement>(),
                //     feature = "static_render" => element.dyn_ref::<TextAreaElement>(),
                // }
            } {
                if let Some(change) = self.diff_value(ancestor) {
                    let value = match change {
                        Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                        Patch::Remove(_) => "",
                    };
                    tae.set_value(value);
                }
            }
        }
    }

    fn create_element(&self, parent: &Element) -> Element {
        let tag = self.tag();
        if tag == "svg"
            || parent
                .namespace_uri()
                .map_or(false, |ns| ns == SVG_NAMESPACE)
        {
            let namespace = Some(SVG_NAMESPACE);

            get_document()
                .create_element_ns(namespace, tag)
                .expect("can't create namespaced element for vtag")
        } else {
            get_document()
                .create_element(tag)
                .expect("can't create element for vtag")
        }
    }
}

mod diff {
    use super::*;

    impl VDiff for VTag {
        /// Remove VTag from parent.
        fn detach(&mut self, parent: &Element) {
            let node = self
                .reference
                .take()
                .expect("tried to remove not rendered VTag from DOM");

            // recursively remove its children
            self.children.detach(&node);
            if parent.remove_child(&node.into()).is_err() {
                warn!("Node not found to remove VTag");
            }
            self.node_ref.set(None);
        }

        /// Renders virtual tag over DOM `Element`, but it also compares this with an ancestor `VTag`
        /// to compute what to patch in the actual DOM nodes.
        fn apply(
            &mut self,
            parent_scope: &AnyScope,
            parent: &Element,
            next_sibling: NodeRef,
            ancestor: Option<VNode>,
        ) -> NodeRef {
            let mut ancestor_tag = ancestor.and_then(|mut ancestor| {
                match ancestor {
                    // If the ancestor is a tag of the same type, don't recreate, keep the
                    // old tag and update its attributes and children.
                    VNode::VTag(vtag) if self.tag() == vtag.tag() && self.key == vtag.key => {
                        Some(vtag)
                    }
                    _ => {
                        let element = self.create_element(parent);
                        super::super::insert_node(
                            (&element).into(),
                            parent,
                            Some(ancestor.first_node()),
                        );
                        self.reference = Some(element);
                        ancestor.detach(parent);
                        None
                    }
                }
            });

            if let Some(ref mut ancestor_tag) = &mut ancestor_tag {
                // Refresh the current value to later compare it against the desired value
                // since it may have been changed since we last set it.
                ancestor_tag.refresh_value();
                // Preserve the reference that already exists.
                self.reference = ancestor_tag.reference.take();
            } else if self.reference.is_none() {
                let element = self.create_element(parent);
                super::super::insert_node((&element).into(), parent, next_sibling.get());
                self.reference = Some(element);
            }

            self.apply_diffs(&mut ancestor_tag);
            self.recreate_listeners(&mut ancestor_tag);

            // Process children
            let element = self.reference.as_ref().expect("Reference should be set");
            if !self.children.is_empty() {
                self.children.apply(
                    parent_scope,
                    element,
                    NodeRef::default(),
                    ancestor_tag.map(|a| a.children.into()),
                );
            } else if let Some(mut ancestor_tag) = ancestor_tag {
                ancestor_tag.children.detach(element);
            }

            let node = element.as_node();

            // let node = cfg_match! {
            //     feature = "std_web" => element.as_node(),
            //     feature = "web_sys" => element.deref(),
            //     feature = "static_render" => element.deref(),
            // };
            self.node_ref.set(Some(node.clone().into()));
            self.node_ref.clone()
        }
    }
}

/// Set `checked` value for the `InputElement`.
fn set_checked(input: &InputElement, value: bool) {
    input.set_checked(value)
}

impl PartialEq for VTag {
    fn eq(&self, other: &VTag) -> bool {
        self.tag == other.tag
            && self.value == other.value
            && self.kind == other.kind
            && self.checked == other.checked
            && self.listeners.len() == other.listeners.len()
            && self
                .listeners
                .iter()
                .map(|l| l.kind())
                .eq(other.listeners.iter().map(|l| l.kind()))
            && self.attributes == other.attributes
            && self.children == other.children
    }
}
