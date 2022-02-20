//! This module contains the bundle version of a supsense [BSuspense]

use super::{BNode, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::virtual_dom::{Key, VSuspense};
use crate::NodeRef;
use web_sys::Element;

/// The bundle implementation to [VSuspense]
#[derive(Debug)]
pub struct BSuspense {
    children_bundle: BNode,
    /// The supsense is suspended if fallback contains [Some] bundle
    fallback_bundle: Option<BNode>,
    detached_parent: Element,
    key: Option<Key>,
}

impl BSuspense {
    /// Get the key of the underlying suspense
    pub(super) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
    /// Get the bundle node that actually shows up in the dom
    fn active_node(&self) -> &BNode {
        self.fallback_bundle
            .as_ref()
            .unwrap_or(&self.children_bundle)
    }
}

impl DomBundle for BSuspense {
    fn detach(self, parent: &Element, parent_to_detach: bool) {
        if let Some(fallback) = self.fallback_bundle {
            fallback.detach(parent, parent_to_detach);
            self.children_bundle.detach(&self.detached_parent, false);
        } else {
            self.children_bundle.detach(parent, parent_to_detach);
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
            let (_child_ref, children_bundle) =
                children.attach(parent_scope, &detached_parent, NodeRef::default());
            let (fallback_ref, fallback) = fallback.attach(parent_scope, parent, next_sibling);
            (
                fallback_ref,
                BSuspense {
                    children_bundle,
                    fallback_bundle: Some(fallback),
                    detached_parent,
                    key,
                },
            )
        } else {
            let (child_ref, children_bundle) = children.attach(parent_scope, parent, next_sibling);
            (
                child_ref,
                BSuspense {
                    children_bundle,
                    fallback_bundle: None,
                    detached_parent,
                    key,
                },
            )
        }
    }

    fn reconcile_node(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        match bundle {
            // We only preserve the child state if they are the same suspense.
            BNode::BSuspense(m)
                if m.key == self.key
                    && self.detached_parent.as_ref() == Some(&m.detached_parent) =>
            {
                self.reconcile(parent_scope, parent, next_sibling, m)
            }
            _ => self.replace(parent_scope, parent, next_sibling, bundle),
        }
    }

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        suspense: &mut Self::Bundle,
    ) -> NodeRef {
        let VSuspense {
            children,
            fallback,
            detached_parent,
            suspended,
            key: _,
        } = self;
        let detached_parent = detached_parent.expect("no detached parent?");

        let children_bundle = &mut suspense.children_bundle;
        // no need to update key & detached_parent

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (suspended, &mut suspense.fallback_bundle) {
            // Both suspended, reconcile children into detached_parent, fallback into the DOM
            (true, Some(fallback_bundle)) => {
                children.reconcile_node(
                    parent_scope,
                    &detached_parent,
                    NodeRef::default(),
                    children_bundle,
                );

                fallback.reconcile_node(parent_scope, parent, next_sibling, fallback_bundle)
            }
            // Not suspended, just reconcile the children into the DOM
            (false, None) => {
                children.reconcile_node(parent_scope, parent, next_sibling, children_bundle)
            }
            // Freshly suspended. Shift children into the detached parent, then add fallback to the DOM
            (true, None) => {
                children_bundle.shift(&detached_parent, NodeRef::default());

                children.reconcile_node(
                    parent_scope,
                    &detached_parent,
                    NodeRef::default(),
                    children_bundle,
                );
                // first render of fallback
                let (fallback_ref, fallback) = fallback.attach(parent_scope, parent, next_sibling);
                suspense.fallback_bundle = Some(fallback);
                fallback_ref
            }
            // Freshly unsuspended. Detach fallback from the DOM, then shift children into it.
            (false, Some(_)) => {
                suspense
                    .fallback_bundle
                    .take()
                    .unwrap() // We just matched Some(_)
                    .detach(parent, false);

                children_bundle.shift(parent, next_sibling.clone());
                children.reconcile_node(parent_scope, parent, next_sibling, children_bundle)
            }
        }
    }
}
