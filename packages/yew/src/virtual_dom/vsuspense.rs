use super::{Key, VNode};
use crate::{
    dom_bundle::{DomBundle, VDiff},
    html::{AnyScope, NodeRef},
};
use std::borrow::BorrowMut;
use web_sys::Element;

/// This struct represents a suspendable DOM fragment.
#[derive(Clone, Debug, PartialEq)]
pub struct VSuspense {
    /// Child nodes.
    children: Box<VNode>,
    /// Fallback nodes when suspended.
    fallback: Box<VNode>,
    /// The element to attach to when children is not attached to DOM
    detached_parent: Element,
    /// Whether the current status is suspended.
    suspended: bool,
    /// The Key.
    pub(crate) key: Option<Key>,
}

impl VSuspense {
    pub(crate) fn new(
        children: VNode,
        fallback: VNode,
        detached_parent: Element,
        suspended: bool,
        key: Option<Key>,
    ) -> Self {
        Self {
            children: children.into(),
            fallback: fallback.into(),
            detached_parent,
            suspended,
            key,
        }
    }

    pub(crate) fn active_node(&self) -> &VNode {
        if self.suspended {
            &self.fallback
        } else {
            &self.children
        }
    }
}

impl DomBundle for VSuspense {
    fn detach(self, parent: &Element) {
        if self.suspended {
            self.fallback.detach(parent);
            self.children.detach(&self.detached_parent);
        } else {
            self.children.detach(parent);
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        self.active_node().shift(next_parent, next_sibling)
    }
}

impl VDiff for VSuspense {
    type Bundle = VSuspense;

    fn attach(
        mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        let node_ref = if self.suspended {
            let (_child_ref, children) =
                self.children
                    .attach(parent_scope, &self.detached_parent, NodeRef::default());
            self.children = children.into();
            let (fallback_ref, fallback) = self.fallback.attach(parent_scope, parent, next_sibling);
            self.fallback = fallback.into();
            fallback_ref
        } else {
            let (child_ref, children) = self.children.attach(parent_scope, parent, next_sibling);
            self.children = children.into();
            child_ref
        };
        (node_ref, self)
    }

    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut VNode,
    ) -> NodeRef {
        let suspense = match ancestor {
            // We only preserve the child state if they are the same suspense.
            VNode::VSuspense(m)
                if m.key == self.key && self.detached_parent == m.detached_parent =>
            {
                m
            }
            _ => {
                let (self_ref, self_) = self.attach(parent_scope, parent, next_sibling);
                ancestor.replace(parent, self_.into());
                return self_ref;
            }
        };
        let was_suspended = suspense.suspended;
        let children_ancestor = suspense.children.borrow_mut();
        let fallback_ancestor = suspense.fallback.borrow_mut();

        suspense.suspended = self.suspended;
        // no need to update key & detached_parent

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (self.suspended, was_suspended) {
            (true, true) => {
                self.children.apply(
                    parent_scope,
                    &self.detached_parent,
                    NodeRef::default(),
                    children_ancestor,
                );

                self.fallback
                    .apply(parent_scope, parent, next_sibling, fallback_ancestor)
            }

            (false, false) => {
                self.children
                    .apply(parent_scope, parent, next_sibling, children_ancestor)
            }

            (true, false) => {
                children_ancestor.shift(&self.detached_parent, NodeRef::default());

                self.children.apply(
                    parent_scope,
                    &self.detached_parent,
                    NodeRef::default(),
                    children_ancestor,
                );
                // first render of fallback
                let (fallback_ref, fallback) =
                    self.fallback.attach(parent_scope, parent, next_sibling);
                *fallback_ancestor = fallback;
                fallback_ref
            }

            (false, true) => {
                fallback_ancestor.replace(parent, VNode::default());

                children_ancestor.shift(parent, next_sibling.clone());
                self.children
                    .apply(parent_scope, parent, next_sibling, children_ancestor)
            }
        }
    }
}
