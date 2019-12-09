//! This module contains the implementation of a virtual text node `VText`.

use super::{Reform, VDiff, VNode};
use log::warn;
use std::cmp::PartialEq;
use std::fmt;
use stdweb::web::{document, Element, INode, Node, TextNode};

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
pub struct VText {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
        }
    }
}

impl VDiff for VText {
    /// Remove VText from parent.
    fn detach(&mut self, parent: &Element) -> Option<Node> {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");
        let next_sibling = node.next_sibling();
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VText");
        }
        next_sibling
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text had changed.
    fn apply(
        &mut self,
        parent: &Element,
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        assert!(
            self.reference.is_none(),
            "reference is ignored so must not be set"
        );
        let reform = {
            match ancestor {
                // If element matched this type
                Some(VNode::VText(mut vtext)) => {
                    self.reference = vtext.reference.take();
                    if self.text != vtext.text {
                        if let Some(ref element) = self.reference {
                            element.set_node_value(Some(&self.text));
                        }
                    }
                    Reform::Keep
                }
                Some(mut vnode) => Reform::Before(vnode.detach(parent)),
                None => Reform::Before(None),
            }
        };
        match reform {
            Reform::Keep => {}
            Reform::Before(next_sibling) => {
                let element = document().create_text_node(&self.text);
                if let Some(next_sibling) = next_sibling {
                    parent
                        .insert_before(&element, &next_sibling)
                        .expect("can't insert text before the next sibling");
                } else if let Some(next_sibling) = previous_sibling.and_then(|p| p.next_sibling()) {
                    parent
                        .insert_before(&element, &next_sibling)
                        .expect("can't insert text before next_sibling");
                } else {
                    parent.append_child(&element);
                }
                self.reference = Some(element);
            }
        }
        self.reference.as_ref().map(|t| t.as_node().to_owned())
    }
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

impl PartialEq for VText {
    fn eq(&self, other: &VText) -> bool {
        self.text == other.text
    }
}
