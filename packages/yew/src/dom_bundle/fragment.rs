use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use super::{BSubtree, DomSlot};
use crate::virtual_dom::Collectable;

/// A Hydration Fragment
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Fragment(VecDeque<Node>, Option<Node>);

impl Deref for Fragment {
    type Target = VecDeque<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fragment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Fragment {
    /// Collects child nodes of an element into a VecDeque.
    pub fn collect_children(parent: &Element) -> Self {
        let mut fragment = VecDeque::with_capacity(parent.child_nodes().length() as usize);

        let mut current_node = parent.first_child();

        // This is easier than iterating child nodes at the moment
        // as we don't have to downcast iterator values.
        while let Some(m) = current_node {
            current_node = m.next_sibling();
            fragment.push_back(m);
        }

        Self(fragment, None)
    }

    /// Collects nodes for a Component Bundle or a BSuspense.
    pub fn collect_between(
        collect_from: &mut Fragment,
        collect_for: &Collectable,
        parent: &Element,
    ) -> Self {
        let is_open_tag = |node: &Node| {
            let comment_text = node.text_content().unwrap_or_default();

            comment_text.starts_with(collect_for.open_start_mark())
                && comment_text.ends_with(collect_for.end_mark())
        };

        let is_close_tag = |node: &Node| {
            let comment_text = node.text_content().unwrap_or_default();

            comment_text.starts_with(collect_for.close_start_mark())
                && comment_text.ends_with(collect_for.end_mark())
        };

        // We trim all leading text nodes as it's likely these are whitespaces.
        collect_from.trim_start_text_nodes();

        let first_node = collect_from
            .pop_front()
            .unwrap_or_else(|| panic!("expected {} opening tag, found EOF", collect_for.name()));

        assert_eq!(
            first_node.node_type(),
            Node::COMMENT_NODE,
            // TODO: improve error message with human readable node type name.
            "expected {} start, found node type {}",
            collect_for.name(),
            first_node.node_type()
        );

        let mut nodes = VecDeque::new();

        if !is_open_tag(&first_node) {
            panic!(
                "expected {} opening tag, found comment node",
                collect_for.name()
            );
        }

        // We remove the opening tag.
        parent.remove_child(&first_node).unwrap();

        let mut nested_layers = 1;

        loop {
            let current_node = collect_from.pop_front().unwrap_or_else(|| {
                panic!("expected {} closing tag, found EOF", collect_for.name())
            });

            if current_node.node_type() == Node::COMMENT_NODE {
                if is_open_tag(&current_node) {
                    // We found another opening tag, we need to increase component counter.
                    nested_layers += 1;
                } else if is_close_tag(&current_node) {
                    // We found a closing tag, minus component counter.
                    nested_layers -= 1;
                    if nested_layers == 0 {
                        // We have found the end of the current tag we are collecting, breaking
                        // the loop.

                        // We remove the closing tag.
                        parent.remove_child(&current_node).unwrap();
                        break;
                    }
                }
            }

            nodes.push_back(current_node);
        }

        let next_child = collect_from.0.front().cloned();
        Self(nodes, next_child)
    }

    /// Remove child nodes until first non-text node.
    pub fn trim_start_text_nodes(&mut self) {
        while let Some(ref m) = self.front().cloned() {
            if m.node_type() == Node::TEXT_NODE {
                self.pop_front();

                m.unchecked_ref::<web_sys::Text>().remove();
            } else {
                break;
            }
        }
    }

    /// Deeply clones all nodes.
    pub fn deep_clone(&self) -> Self {
        let nodes = self
            .iter()
            .map(|m| m.clone_node_with_deep(true).expect("failed to clone node."))
            .collect::<VecDeque<_>>();

        // the cloned nodes are disconnected from the real dom, so next_child is `None`
        Self(nodes, None)
    }

    // detaches current fragment.
    pub fn detach(self, _root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        if !parent_to_detach {
            for node in self.iter() {
                parent
                    .remove_child(node)
                    .expect("failed to remove child element");
            }
        }
    }

    /// Shift current Fragment into a different position in the dom.
    pub fn shift(&self, next_parent: &Element, slot: DomSlot) -> DomSlot {
        for node in self.iter() {
            slot.insert(next_parent, node);
        }

        self.front().cloned().map(DomSlot::at).unwrap_or(slot)
    }

    /// Return the node that comes after all the nodes in this fragment
    pub fn sibling_at_end(&self) -> Option<&Node> {
        self.1.as_ref()
    }
}
