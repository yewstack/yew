//! This module contains the bundle version of a supsense [BSuspense]

use gloo::utils::document;
use web_sys::Element;

#[cfg(feature = "hydration")]
use super::Fragment;
use super::{BNode, BSubtree, Reconcilable, ReconcileTarget};
use crate::html::AnyScope;
use crate::virtual_dom::{Key, VSuspense};
use crate::NodeRef;

#[derive(Debug)]
enum Fallback {
    /// Suspense Fallback with fallback being rendered as placeholder.
    Bundle(BNode),
    /// Suspense Fallback with Hydration Fragment being rendered as placeholder.
    #[cfg(feature = "hydration")]
    Fragment(Fragment),
}

/// The bundle implementation to [VSuspense]
#[derive(Debug)]
pub(super) struct BSuspense {
    children_bundle: BNode,
    /// The supsense is suspended if fallback contains [Some] bundle
    fallback: Option<Fallback>,
    detached_parent: Element,
    key: Option<Key>,
}

impl BSuspense {
    /// Get the key of the underlying suspense
    pub fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

impl ReconcileTarget for BSuspense {
    fn detach(self, root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        match self.fallback {
            Some(m) => {
                match m {
                    Fallback::Bundle(bundle) => {
                        bundle.detach(root, parent, parent_to_detach);
                    }

                    #[cfg(feature = "hydration")]
                    Fallback::Fragment(fragment) => {
                        fragment.detach(root, parent, parent_to_detach);
                    }
                }

                self.children_bundle
                    .detach(root, &self.detached_parent, false);
            }
            None => {
                self.children_bundle.detach(root, parent, parent_to_detach);
            }
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) -> NodeRef {
        match self.fallback.as_ref() {
            Some(Fallback::Bundle(bundle)) => bundle.shift(next_parent, next_sibling),
            #[cfg(feature = "hydration")]
            Some(Fallback::Fragment(fragment)) => fragment.shift(next_parent, next_sibling),
            None => self.children_bundle.shift(next_parent, next_sibling),
        }
    }
}

impl Reconcilable for VSuspense {
    type Bundle = BSuspense;

    fn attach(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let VSuspense {
            children,
            fallback,
            suspended,
            key,
        } = self;
        let detached_parent = document()
            .create_element("div")
            .expect("failed to create detached element");

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        if suspended {
            let (_child_ref, children_bundle) =
                children.attach(root, parent_scope, &detached_parent, NodeRef::default());
            let (fallback_ref, fallback) =
                fallback.attach(root, parent_scope, parent, next_sibling);
            (
                fallback_ref,
                BSuspense {
                    children_bundle,
                    fallback: Some(Fallback::Bundle(fallback)),
                    detached_parent,
                    key,
                },
            )
        } else {
            let (child_ref, children_bundle) =
                children.attach(root, parent_scope, parent, next_sibling);
            (
                child_ref,
                BSuspense {
                    children_bundle,
                    fallback: None,
                    detached_parent,
                    key,
                },
            )
        }
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        match bundle {
            // We only preserve the child state if they are the same suspense.
            BNode::Suspense(m) if m.key == self.key => {
                self.reconcile(root, parent_scope, parent, next_sibling, m)
            }
            _ => self.replace(root, parent_scope, parent, next_sibling, bundle),
        }
    }

    fn reconcile(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        suspense: &mut Self::Bundle,
    ) -> NodeRef {
        let VSuspense {
            children,
            fallback: vfallback,
            suspended,
            key: _,
        } = self;

        let children_bundle = &mut suspense.children_bundle;
        // no need to update key & detached_parent

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (suspended, &mut suspense.fallback) {
            // Both suspended, reconcile children into detached_parent, fallback into the DOM
            (true, Some(fallback)) => {
                children.reconcile_node(
                    root,
                    parent_scope,
                    &suspense.detached_parent,
                    NodeRef::default(),
                    children_bundle,
                );

                match fallback {
                    Fallback::Bundle(bundle) => {
                        vfallback.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
                    }
                    #[cfg(feature = "hydration")]
                    Fallback::Fragment(fragment) => match fragment.front().cloned() {
                        Some(m) => NodeRef::new(m),
                        None => next_sibling,
                    },
                }
            }
            // Not suspended, just reconcile the children into the DOM
            (false, None) => {
                children.reconcile_node(root, parent_scope, parent, next_sibling, children_bundle)
            }
            // Freshly suspended. Shift children into the detached parent, then add fallback to the
            // DOM
            (true, None) => {
                children_bundle.shift(&suspense.detached_parent, NodeRef::default());

                children.reconcile_node(
                    root,
                    parent_scope,
                    &suspense.detached_parent,
                    NodeRef::default(),
                    children_bundle,
                );
                // first render of fallback

                let (fallback_ref, fallback) =
                    vfallback.attach(root, parent_scope, parent, next_sibling);
                suspense.fallback = Some(Fallback::Bundle(fallback));
                fallback_ref
            }
            // Freshly unsuspended. Detach fallback from the DOM, then shift children into it.
            (false, Some(_)) => {
                match suspense.fallback.take() {
                    Some(Fallback::Bundle(bundle)) => {
                        bundle.detach(root, parent, false);
                    }
                    #[cfg(feature = "hydration")]
                    Some(Fallback::Fragment(fragment)) => {
                        fragment.detach(root, parent, false);
                    }
                    None => {
                        unreachable!("None condition has been checked before.")
                    }
                };

                children_bundle.shift(parent, next_sibling.clone());
                children.reconcile_node(root, parent_scope, parent, next_sibling, children_bundle)
            }
        }
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::{Fragment, Hydratable};
    use crate::virtual_dom::Collectable;

    impl Hydratable for VSuspense {
        fn hydrate(
            self,
            root: &BSubtree,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> (NodeRef, Self::Bundle) {
            let detached_parent = document()
                .create_element("div")
                .expect("failed to create detached element");

            let collectable = Collectable::Suspense;
            let fallback_fragment = Fragment::collect_between(fragment, &collectable, parent);

            let mut nodes = fallback_fragment.deep_clone();

            for node in nodes.iter() {
                detached_parent.append_child(node).unwrap();
            }

            // Even if initially suspended, these children correspond to the first non-suspended
            // content Refer to VSuspense::render_to_string
            let (_, children_bundle) =
                self.children
                    .hydrate(root, parent_scope, &detached_parent, &mut nodes);

            // We trim all leading text nodes before checking as it's likely these are whitespaces.
            nodes.trim_start_text_nodes(&detached_parent);

            assert!(nodes.is_empty(), "expected end of suspense, found node.");

            let node_ref = fallback_fragment
                .front()
                .cloned()
                .map(NodeRef::new)
                .unwrap_or_default();

            (
                node_ref,
                BSuspense {
                    children_bundle,
                    detached_parent,
                    key: self.key,

                    // We start hydration with the BSuspense being suspended.
                    // A subsequent render will resume the BSuspense if not needed to be suspended.
                    fallback: Some(Fallback::Fragment(fallback_fragment)),
                },
            )
        }
    }
}
