use super::{Key, VDiff, VNode};
use crate::html::{AnyScope, NodeRef};
use web_sys::{Element, Node};

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

    pub(crate) fn first_node(&self) -> Option<Node> {
        if self.suspended {
            self.fallback.first_node()
        } else {
            self.children.first_node()
        }
    }
}

impl VDiff for VSuspense {
    fn detach(&mut self, parent: &Element) {
        if self.suspended {
            self.fallback.detach(parent);
            self.children.detach(&self.detached_parent);
        } else {
            self.children.detach(parent);
        }
    }

    fn shift(&self, previous_parent: &Element, next_parent: &Element, next_sibling: NodeRef) {
        if self.suspended {
            self.fallback
                .shift(previous_parent, next_parent, next_sibling);
        } else {
            self.children
                .shift(previous_parent, next_parent, next_sibling);
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        let (already_suspended, children_ancestor, fallback_ancestor) = match ancestor {
            Some(VNode::VSuspense(mut m)) => {
                // We only preserve the child state if they are the same suspense.
                if m.key != self.key || self.detached_parent != m.detached_parent {
                    m.detach(parent);

                    (false, None, None)
                } else {
                    (m.suspended, Some(*m.children), Some(*m.fallback))
                }
            }
            Some(mut m) => {
                m.detach(parent);
                (false, None, None)
            }
            None => (false, None, None),
        };

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (self.suspended, already_suspended) {
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
                children_ancestor.as_ref().unwrap().shift(
                    parent,
                    &self.detached_parent,
                    NodeRef::default(),
                );

                self.children.apply(
                    parent_scope,
                    &self.detached_parent,
                    NodeRef::default(),
                    children_ancestor,
                );

                // first render of fallback, ancestor needs to be None.
                self.fallback
                    .apply(parent_scope, parent, next_sibling, None)
            }

            (false, true) => {
                fallback_ancestor.unwrap().detach(parent);

                children_ancestor.as_ref().unwrap().shift(
                    &self.detached_parent,
                    parent,
                    next_sibling.clone(),
                );
                self.children
                    .apply(parent_scope, parent, next_sibling, children_ancestor)
            }
        }
    }
}
