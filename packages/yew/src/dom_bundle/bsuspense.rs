//! This module contains the bundle version of a supsense [BSuspense]

use super::{BNode, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::virtual_dom::{Key, VSuspense};
use crate::NodeRef;
use web_sys::Element;

/// The bundle implementation to [VSuspense]
#[derive(Debug)]
pub struct BSuspense {
    children: BNode,
    /// The supsense is suspended if fallback contains [Some] bundle
    fallback: Option<BNode>,
    detached_parent: Element,
    key: Option<Key>,
}

impl BSuspense {
    /// Get the key of the underlying suspense
    pub(crate) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
    /// Get the bundle node that actually shows up in the dom
    fn active_node(&self) -> &BNode {
        self.fallback.as_ref().unwrap_or(&self.children)
    }
}

impl DomBundle for BSuspense {
    fn detach(self, parent: &Element) {
        if let Some(fallback) = self.fallback {
            fallback.detach(parent);
            self.children.detach(&self.detached_parent);
        } else {
            self.children.detach(parent);
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        self.active_node().shift(next_parent, next_sibling)
    }
}

impl Reconcilable for VSuspense {
    type Bundle = BSuspense;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let VSuspense {
            children,
            fallback,
            detached_parent,
            suspended,
            key,
        } = self;
        let detached_parent = detached_parent.expect("no detached parent?");

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        if suspended {
            let (_child_ref, children) =
                children.attach(parent_scope, &detached_parent, NodeRef::default());
            let (fallback_ref, fallback) = fallback.attach(parent_scope, parent, next_sibling);
            (
                fallback_ref,
                BSuspense {
                    children,
                    fallback: Some(fallback),
                    detached_parent,
                    key,
                },
            )
        } else {
            let (child_ref, children) = children.attach(parent_scope, parent, next_sibling);
            (
                child_ref,
                BSuspense {
                    children,
                    fallback: None,
                    detached_parent,
                    key,
                },
            )
        }
    }

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        let suspense = match bundle {
            // We only preserve the child state if they are the same suspense.
            BNode::BSuspense(m)
                if m.key == self.key
                    && self.detached_parent.as_ref() == Some(&m.detached_parent) =>
            {
                m
            }
            _ => {
                let (self_ref, self_) = self.attach(parent_scope, parent, next_sibling);
                bundle.replace(parent, self_.into());
                return self_ref;
            }
        };
        let VSuspense {
            children,
            fallback,
            detached_parent,
            suspended,
            key: _,
        } = self;
        let detached_parent = detached_parent.expect("no detached parent?");

        let children_bundle = &mut suspense.children;
        // no need to update key & detached_parent

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (suspended, &mut suspense.fallback) {
            (true, Some(fallback_bundle)) => {
                children.reconcile(
                    parent_scope,
                    &detached_parent,
                    NodeRef::default(),
                    children_bundle,
                );

                fallback.reconcile(parent_scope, parent, next_sibling, fallback_bundle)
            }

            (false, None) => {
                children.reconcile(parent_scope, parent, next_sibling, children_bundle)
            }

            (true, None) => {
                children_bundle.shift(&detached_parent, NodeRef::default());

                children.reconcile(
                    parent_scope,
                    &detached_parent,
                    NodeRef::default(),
                    children_bundle,
                );
                // first render of fallback
                let (fallback_ref, fallback) = fallback.attach(parent_scope, parent, next_sibling);
                suspense.fallback = Some(fallback);
                fallback_ref
            }

            (false, Some(_)) => {
                suspense.fallback.take().unwrap().detach(parent);

                children_bundle.shift(parent, next_sibling.clone());
                children.reconcile(parent_scope, parent, next_sibling, children_bundle)
            }
        }
    }
}
