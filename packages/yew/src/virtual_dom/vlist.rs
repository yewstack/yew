//! This module contains fragments implementation.
use super::{Key, VDiff, VNode, VText};
use crate::html::{AnyScope, NodeRef};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct VList {
    /// The list of children nodes.
    pub children: Vec<VNode>,
    pub key: Option<Key>,
}

impl Deref for VList {
    type Target = Vec<VNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for VList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

impl VList {
    /// Creates a new empty `VList` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `VList` instance with children.
    pub fn new_with_children(children: Vec<VNode>, key: Option<Key>) -> Self {
        VList { children, key }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.push(child);
    }

    /// Add multiple `VNode` children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        self.children.extend(children);
    }

    fn children_keys(&self, warn: bool) -> HashSet<Key> {
        let mut hash_set = HashSet::with_capacity(self.children.len());
        for l in self.children.iter() {
            if let Some(k) = l.key() {
                if !hash_set.insert(k.clone()) && warn {
                    log::warn!("Key '{}' is not unique in list but must be.", k);
                }
            }
        }
        hash_set
    }
}

impl VDiff for VList {
    fn detach(&mut self, parent: &Element) {
        for mut child in self.children.drain(..) {
            child.detach(parent);
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        // Here, we will try to diff the previous list elements with the new
        // ones we want to insert. For that, we will use two lists:
        //  - lefts: new elements to render in the DOM
        //  - rights: previously rendered elements.
        //
        // The left items are known since we want to insert them
        // (self.children). For the right ones, we will look at the ancestor,
        // i.e. the current DOM list element that we want to replace with self.

        if self.children.is_empty() {
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("");
            self.children.push(placeholder.into());
        }

        let left_keys = self.children_keys(true);
        let lefts_keyed = left_keys.len() == self.children.len();

        let right_keys = if let Some(VNode::VList(vlist)) = &ancestor {
            vlist.children_keys(false)
        } else {
            HashSet::new()
        };

        let mut right_children = match ancestor {
            // If the ancestor is also a VList, then the "right" list is the
            // previously rendered items.
            Some(VNode::VList(vlist)) => vlist.children,
            // If the ancestor was not a VList, then the "right" list is a single node
            Some(vnode) => vec![vnode],
            None => vec![],
        };
        let rights_keyed = right_keys.len() == right_children.len();

        // If the existing list and the new list are both properly keyed,
        // then move existing list nodes into the new list's order before diffing
        if lefts_keyed && rights_keyed {
            // Find the intersection of keys to determine which right nodes can be reused
            let matched_keys: HashSet<_> = left_keys.intersection(&right_keys).collect();

            // Detach any right nodes that were not matched with a left node
            right_children = right_children
                .into_iter()
                .filter_map(|mut right| {
                    if matched_keys.contains(right.key().as_ref().unwrap()) {
                        Some(right)
                    } else {
                        right.detach(parent);
                        None
                    }
                })
                .collect();

            // Determine which rights are already in correct order and which
            // rights need to be moved in the DOM before being reused
            let mut rights_to_move = HashMap::with_capacity(right_children.len());
            let mut matched_lefts = self
                .children
                .iter()
                .filter(|left| matched_keys.contains(left.key().as_ref().unwrap()))
                .peekable();
            let mut left = matched_lefts.next();

            // Note: `filter_map` is used to move rights into `rights_to_move`
            #[allow(clippy::unnecessary_filter_map)]
            let rights_in_place: Vec<_> = right_children
                .into_iter()
                .filter_map(|right| {
                    if right.key() == left.and_then(|l| l.key()) {
                        left = matched_lefts.next();
                        return Some(right);
                    } else if right.key() == matched_lefts.peek().and_then(|l| l.key()) {
                        matched_lefts.next();
                        left = matched_lefts.next();
                        return Some(right);
                    }

                    rights_to_move.insert(right.key().unwrap(), right);
                    None
                })
                .collect();

            // Move rights into correct order and build `right_children`
            right_children = Vec::with_capacity(matched_keys.len());
            let mut matched_lefts = self
                .children
                .iter()
                .filter(|left| matched_keys.contains(left.key().as_ref().unwrap()));

            for right in rights_in_place.into_iter() {
                let mut left = matched_lefts.next().unwrap();
                while right.key() != left.key() {
                    let right_to_move = rights_to_move.remove(&left.key().unwrap()).unwrap();
                    right_to_move.move_before(parent, Some(right.first_node()));
                    right_children.push(right_to_move);
                    left = matched_lefts.next().unwrap();
                }
                right_children.push(right);
            }

            for left in matched_lefts {
                let right_to_move = rights_to_move.remove(&left.key().unwrap()).unwrap();
                right_to_move.move_before(parent, next_sibling.get());
                right_children.push(right_to_move);
            }

            assert!(rights_to_move.is_empty())
        }

        let mut rights = right_children.into_iter().peekable();
        let mut last_next_sibling = NodeRef::default();
        let mut nodes: Vec<NodeRef> = self
            .children
            .iter_mut()
            .map(|left| {
                let ancestor = rights.next();

                // Create a new `next_sibling` reference which points to the next `right` or
                // the outer list's `next_sibling` if there are no more `rights`.
                let new_next_sibling = NodeRef::default();
                if let Some(next_right) = rights.peek() {
                    new_next_sibling.set(Some(next_right.first_node()));
                } else {
                    new_next_sibling.link(next_sibling.clone());
                }

                // Update the next list item and then link the previous left's `next_sibling` to the
                // returned `node` reference so that the previous left has an up-to-date `next_sibling`.
                // This is important for rendering a `VComp` because each `VComp` keeps track of its
                // `next_sibling` to properly render its children.
                let node = left.apply(parent_scope, parent, new_next_sibling.clone(), ancestor);
                last_next_sibling.link(node.clone());
                last_next_sibling = new_next_sibling;
                node
            })
            .collect();

        // If there are more `rights` than `lefts`, we need to make sure to link the last left's `next_sibling`
        // to the outer list's `next_sibling` so that it doesn't point at a `right` that is detached.
        last_next_sibling.link(next_sibling);

        // Detach all extra rights
        for mut right in rights {
            right.detach(parent);
        }

        assert!(!nodes.is_empty(), "VList should have at least one child");
        nodes.swap_remove(0)
    }
}
