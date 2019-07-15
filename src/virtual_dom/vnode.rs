//! This module contains the implementation of abstract virtual node.

use super::{VComp, VDiff, VList, VTag, VText};
use crate::html::{Component, Renderable, Scope};
use std::cmp::PartialEq;
use std::fmt;
use stdweb::web::{INode, Node};

/// Bind virtual element to a DOM reference.
pub enum VNode<COMP: Component> {
    /// A bind between `VTag` and `Element`.
    VTag(VTag<COMP>),
    /// A bind between `VText` and `TextNode`.
    VText(VText<COMP>),
    /// A bind between `VComp` and `Element`.
    VComp(VComp<COMP>),
    /// A holder for a list of other nodes.
    VList(VList<COMP>),
    /// A holder for any `Node` (necessary for replacing node).
    VRef(Node),
}

impl<COMP: Component> VDiff for VNode<COMP> {
    type Component = COMP;

    /// Remove VNode from parent.
    fn detach(&mut self, parent: &Node) -> Option<Node> {
        match *self {
            VNode::VTag(ref mut vtag) => vtag.detach(parent),
            VNode::VText(ref mut vtext) => vtext.detach(parent),
            VNode::VComp(ref mut vcomp) => vcomp.detach(parent),
            VNode::VList(ref mut vlist) => vlist.detach(parent),
            VNode::VRef(ref node) => {
                let sibling = node.next_sibling();
                parent
                    .remove_child(node)
                    .expect("can't remove node by VRef");
                sibling
            }
        }
    }

    fn apply(
        &mut self,
        parent: &Node,
        precursor: Option<&Node>,
        ancestor: Option<VNode<Self::Component>>,
        env: &Scope<Self::Component>,
    ) -> Option<Node> {
        match *self {
            VNode::VTag(ref mut vtag) => vtag.apply(parent, precursor, ancestor, env),
            VNode::VText(ref mut vtext) => vtext.apply(parent, precursor, ancestor, env),
            VNode::VComp(ref mut vcomp) => vcomp.apply(parent, precursor, ancestor, env),
            VNode::VList(ref mut vlist) => vlist.apply(parent, precursor, ancestor, env),
            VNode::VRef(ref mut node) => {
                let sibling = match ancestor {
                    Some(mut n) => n.detach(parent),
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

impl<COMP: Component> From<VText<COMP>> for VNode<COMP> {
    fn from(vtext: VText<COMP>) -> Self {
        VNode::VText(vtext)
    }
}

impl<COMP: Component> From<VList<COMP>> for VNode<COMP> {
    fn from(vlist: VList<COMP>) -> Self {
        VNode::VList(vlist)
    }
}

impl<COMP: Component> From<VTag<COMP>> for VNode<COMP> {
    fn from(vtag: VTag<COMP>) -> Self {
        VNode::VTag(vtag)
    }
}

impl<COMP: Component> From<VComp<COMP>> for VNode<COMP> {
    fn from(vcomp: VComp<COMP>) -> Self {
        VNode::VComp(vcomp)
    }
}

impl<COMP: Component, T: ToString> From<T> for VNode<COMP> {
    fn from(value: T) -> Self {
        VNode::VText(VText::new(value.to_string()))
    }
}

impl<'a, COMP: Component> From<&'a dyn Renderable<COMP>> for VNode<COMP> {
    fn from(value: &'a dyn Renderable<COMP>) -> Self {
        value.view()
    }
}

impl<COMP: Component> fmt::Debug for VNode<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VNode::VTag(ref vtag) => vtag.fmt(f),
            VNode::VText(ref vtext) => vtext.fmt(f),
            VNode::VComp(_) => "Component<>".fmt(f),
            VNode::VList(_) => "List<>".fmt(f),
            VNode::VRef(_) => "NodeReference<>".fmt(f),
        }
    }
}

impl<COMP: Component> PartialEq for VNode<COMP> {
    fn eq(&self, other: &VNode<COMP>) -> bool {
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
