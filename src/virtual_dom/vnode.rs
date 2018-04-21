//! This module contains the implementation of abstract virtual node.

use super::{VComp, VDiff, VList, VTag, VText};
use html::{Component, Renderable, Env};
use std::cmp::PartialEq;
use std::fmt;
use stdweb::web::{INode, Node};

/// Bind virtual element to a DOM reference.
pub enum VNode<CTX, COMP: Component<CTX>> {
    /// A bind between `VTag` and `Element`.
    VTag(VTag<CTX, COMP>),
    /// A bind between `VText` and `TextNode`.
    VText(VText<CTX, COMP>),
    /// A bind between `VComp` and `Element`.
    VComp(VComp<CTX, COMP>),
    /// A holder for a list of other nodes.
    VList(VList<CTX, COMP>),
    /// A holder for any `Node` (necessary for replacing node).
    VRef(Node),
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VNode<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Remove VNode from parent.
    fn remove(self, parent: &Node) -> Option<Node> {
        match self {
            VNode::VTag(vtag) => vtag.remove(parent),
            VNode::VText(vtext) => vtext.remove(parent),
            VNode::VComp(vcomp) => vcomp.remove(parent),
            VNode::VList(vlist) => vlist.remove(parent),
            VNode::VRef(node) => {
                let sibling = node.next_sibling();
                parent
                    .remove_child(&node)
                    .expect("can't remove node by VRef");
                sibling
            }
        }
    }

    fn apply(
        &mut self,
        parent: &Node,
        precursor: Option<&Node>,
        ancestor: Option<VNode<Self::Context, Self::Component>>,
        env: Env<Self::Context, Self::Component>,
    ) -> Option<Node> {
        match *self {
            VNode::VTag(ref mut vtag) => vtag.apply(parent, precursor, ancestor, env),
            VNode::VText(ref mut vtext) => vtext.apply(parent, precursor, ancestor, env),
            VNode::VComp(ref mut vcomp) => vcomp.apply(parent, precursor, ancestor, env),
            VNode::VList(ref mut vlist) => vlist.apply(parent, precursor, ancestor, env),
            VNode::VRef(ref mut node) => {
                let sibling = match ancestor {
                    Some(n) => n.remove(parent),
                    None => None,
                };
                if let Some(sibling) = sibling {
                    parent
                        .insert_before(node, &sibling)
                        .expect("can't insert element before sibling");
                } else {
                    parent.append_child(node);
                }

                Some(node.to_owned())
            }
        }
    }
}

impl<CTX, COMP: Component<CTX>> From<VText<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vtext: VText<CTX, COMP>) -> Self {
        VNode::VText(vtext)
    }
}

impl<CTX, COMP: Component<CTX>> From<VList<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vlist: VList<CTX, COMP>) -> Self {
        VNode::VList(vlist)
    }
}

impl<CTX, COMP: Component<CTX>> From<VTag<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vtag: VTag<CTX, COMP>) -> Self {
        VNode::VTag(vtag)
    }
}

impl<CTX, COMP: Component<CTX>> From<VComp<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vcomp: VComp<CTX, COMP>) -> Self {
        VNode::VComp(vcomp)
    }
}

impl<CTX: 'static, COMP: Component<CTX>, T: ToString> From<T> for VNode<CTX, COMP> {
    fn from(value: T) -> Self {
        VNode::VText(VText::new(value.to_string()))
    }
}

impl<'a, CTX, COMP: Component<CTX>> From<&'a Renderable<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(value: &'a Renderable<CTX, COMP>) -> Self {
        value.view()
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VNode<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VNode::VTag(ref vtag) => vtag.fmt(f),
            VNode::VText(ref vtext) => vtext.fmt(f),
            VNode::VComp(_) => "Component<>".fmt(f),
            VNode::VList(_) => "List<>".fmt(f),
            VNode::VRef(_) => "NodeReference<>".fmt(f),
        }
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VNode<CTX, COMP> {
    fn eq(&self, other: &VNode<CTX, COMP>) -> bool {
        match *self {
            VNode::VTag(ref vtag_a) => match *other {
                VNode::VTag(ref vtag_b) => vtag_a == vtag_b,
                _ => false,
            },
            VNode::VText(ref vtext_a) => match *other {
                VNode::VText(ref vtext_b) => vtext_a == vtext_b,
                _ => false,
            },
            _ => {
                // TODO Implement it
                false
            }
        }
    }
}
