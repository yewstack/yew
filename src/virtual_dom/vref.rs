//! This module contains the implementation of a "generic" virtual node `VRef`.

use super::{Reform, VDiff, VNode};
use html::{Component, ScopeEnv};
use std::cmp::PartialEq;
use std::fmt;
use std::marker::PhantomData;
use stdweb::web::{document, INode, Node, TextNode};

/// A type for an arbitrary [Node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
/// represenation.
pub struct VRef<CTX, COMP: Component<CTX>> {
    /// A reference to the `Node`.
    pub reference: Option<Node>,
    _ctx: PhantomData<CTX>,
    _comp: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VRef<CTX, COMP> {
    /// Creates new virtual text node with a content.
    pub fn new() -> Self {
        VRef {
            reference: None,
            _ctx: PhantomData,
            _comp: PhantomData,
        }
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VRef<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Remove VTag from parent.
    fn remove(self, parent: &Node) -> Option<Node> {
        let node = self.reference
            .expect("tried to remove not rendered VRef from DOM");
        let sibling = node.next_sibling();
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VRef");
        }
        sibling
    }

    /// Renders virtual node over existent `Node`.
    ///
    /// TODO: this _always_ succeeds, and thus is not currently performant.
    fn apply(
        &mut self,
        parent: &Node,
        _precursor: Option<&Node>,
        opposite: Option<VNode<Self::Context, Self::Component>>,
        _scope: ScopeEnv<Self::Context, Self::Component>,
    ) -> Option<Node> {
        let reform = {
            match opposite {
                // // If element matched this type
                // Some(VNode::VText(mut vtext)) => {
                //     self.reference = vtext.reference.take();
                //     if self.text != vtext.text {
                //         if let Some(ref element) = self.reference {
                //             element.set_node_value(Some(&self.text));
                //         }
                //     }
                //     Reform::Keep
                // }
                Some(vnode) => {
                    let node = vnode.remove(parent);
                    Reform::Before(node)
                }
                None => Reform::Before(None),
            }
        };
        // note: https://developer.mozilla.org/en-US/docs/Web/API/SVGElement
        // SVG element is a part of "Element"
        match reform {
            Reform::Keep => { unreachable!() }
            Reform::Before(node) => {
                let element = document().create_element("");
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

impl<CTX, COMP: Component<CTX>> fmt::Debug for VRef<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VRef {{ node: {:?} }}", self.reference)
    }
}


// impl<CTX, COMP: Component<CTX>> PartialEq for VRef<CTX, COMP> {
//     fn eq(&self, other: &VRef<CTX, COMP>) -> bool {
//         return self.text == other.text;
//     }
// }
//
impl <CTX, COMP: Component<CTX>> From<Node> for VRef<CTX, COMP> {
    fn from(reference: Node) -> VRef<CTX, COMP> {
        VRef {
            reference: Some(reference),
            _ctx: PhantomData,
            _comp: PhantomData,
        }
    }
}
