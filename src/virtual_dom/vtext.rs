//! This module contains the implementation of a virtual text node `VText`.

use super::{Reform, VDiff, VNode};
use html::{Component, ScopeEnv};
use std::cmp::PartialEq;
use std::fmt;
use std::marker::PhantomData;
use stdweb::web::{document, INode, Node, TextNode};

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// represenation.
pub struct VText<CTX, COMP: Component<CTX>> {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
    _ctx: PhantomData<CTX>,
    _comp: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VText<CTX, COMP> {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
            _ctx: PhantomData,
            _comp: PhantomData,
        }
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VText<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Remove VTag from parent.
    fn remove(self, parent: &Node) -> Option<Node> {
        let node = self.reference
            .expect("tried to remove not rendered VText from DOM");
        let sibling = node.next_sibling();
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VText");
        }
        sibling
    }

    /// Renders virtual node over existent `TextNode`, but
    /// only if value of text had changed.
    /// Parameter `precursor` is necesssary for `VTag` and `VList` which
    /// has children and renders them.
    fn apply(
        &mut self,
        parent: &Node,
        _: Option<&Node>,
        opposite: Option<VNode<Self::Context, Self::Component>>,
        _: ScopeEnv<Self::Context, Self::Component>,
    ) -> Option<Node> {
        let reform = {
            match opposite {
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
                Some(vnode) => {
                    let node = vnode.remove(parent);
                    Reform::Before(node)
                }
                None => Reform::Before(None),
            }
        };
        match reform {
            Reform::Keep => {}
            Reform::Before(node) => {
                let element = document().create_text_node(&self.text);
                if let Some(sibling) = node {
                    parent
                        .insert_before(&element, &sibling)
                        .expect("can't insert text before sibling");
                } else {
                    parent.append_child(&element);
                }
                self.reference = Some(element);
            }
        }
        self.reference.as_ref().map(|t| t.as_node().to_owned())
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VText<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VText<CTX, COMP> {
    fn eq(&self, other: &VText<CTX, COMP>) -> bool {
        self.text == other.text
    }
}
