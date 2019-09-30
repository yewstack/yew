//! This module contains fragments implementation.
use super::{VDiff, VNode, VText};
use crate::html::{Component, Scope};
use stdweb::web::{Element, Node};

/// This struct represents a fragment of the Virtual DOM tree.
pub struct VList<COMP: Component> {
    /// The list of children nodes. Which also could have their own children.
    pub childs: Vec<VNode<COMP>>,
}

impl<COMP: Component> Default for VList<COMP> {
    fn default() -> Self {
        VList::new()
    }
}

impl<COMP: Component> VList<COMP> {
    /// Creates a new empty `VList` instance.
    pub fn new() -> Self {
        VList { childs: Vec::new() }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode<COMP>) {
        self.childs.push(child);
    }
}

impl<COMP: Component> VDiff for VList<COMP> {
    type Component = COMP;

    fn detach(&mut self, parent: &Element) -> Option<Node> {
        let mut last_sibling = None;
        for mut child in self.childs.drain(..) {
            last_sibling = child.detach(parent);
        }
        last_sibling
    }

    fn apply(
        &mut self,
        parent: &Element,
        precursor: Option<&Node>,
        ancestor: Option<VNode<Self::Component>>,
        env: &Scope<Self::Component>,
    ) -> Option<Node> {
        // Reuse precursor, because fragment reuse parent
        let mut precursor = precursor.map(|node| node.to_owned());
        let mut rights = {
            match ancestor {
                // If element matched this type
                Some(VNode::VList(vlist)) => {
                    // Previously rendered items
                    vlist.childs
                }
                Some(vnode) => {
                    // Use the current node as a single fragment list
                    // and let the `apply` of `VNode` to handle it.
                    vec![vnode]
                }
                None => Vec::new(),
            }
        };

        if self.childs.is_empty() {
            // Fixes: https://github.com/yewstack/yew/issues/294
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("".into());
            self.childs.push(placeholder.into());
        }

        // Process children
        let mut lefts = self.childs.iter_mut();
        let mut rights = rights.drain(..);
        loop {
            match (lefts.next(), rights.next()) {
                (Some(left), Some(right)) => {
                    precursor = left.apply(parent, precursor.as_ref(), Some(right), &env);
                }
                (Some(left), None) => {
                    precursor = left.apply(parent, precursor.as_ref(), None, &env);
                }
                (None, Some(ref mut right)) => {
                    right.detach(parent);
                }
                (None, None) => break,
            }
        }
        precursor
    }
}
