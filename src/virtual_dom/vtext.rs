//! This module contains the implementation of a virtual text node `VText`.

use super::{Reform, VDiff, VNode};
use crate::html::{Component, Scope};
use log::warn;
use std::cmp::PartialEq;
use std::fmt;
use std::marker::PhantomData;
use stdweb::web::{document, Element, INode, Node, TextNode};

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
pub struct VText<COMP: Component> {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
    _comp: PhantomData<COMP>,
}

impl<COMP: Component> VText<COMP> {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
            _comp: PhantomData,
        }
    }
}

impl<COMP: Component> VDiff for VText<COMP> {
    type Component = COMP;

    /// Remove VText from parent.
    fn detach(&mut self, parent: &Element) -> Option<Node> {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");
        let sibling = node.next_sibling();
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VText");
        }
        sibling
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text had changed.
    fn apply(
        &mut self,
        parent: &Element,
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode<Self::Component>>,
        _: &Scope<Self::Component>,
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
                Some(mut vnode) => {
                    let node = vnode.detach(parent);
                    Reform::Before(node)
                }
                None => Reform::Before(None),
            }
        };
        match reform {
            Reform::Keep => {}
            Reform::Before(ancestor) => {
                let element = document().create_text_node(&self.text);
                if let Some(ancestor) = ancestor {
                    parent
                        .insert_before(&element, &ancestor)
                        .expect("can't insert text before ancestor");
                } else if let Some(next_sibling) =
                    previous_sibling.and_then(|previous_sibling| previous_sibling.next_sibling())
                {
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

impl<COMP: Component> fmt::Debug for VText<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

impl<COMP: Component> PartialEq for VText<COMP> {
    fn eq(&self, other: &VText<COMP>) -> bool {
        self.text == other.text
    }
}
