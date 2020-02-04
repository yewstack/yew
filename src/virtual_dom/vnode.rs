//! This module contains the implementation of abstract virtual node.

use super::{VChild, VComp, VDiff, VList, VTag, VText};
use crate::html::{Component, Renderable};
use std::cmp::PartialEq;
use std::fmt;
use std::iter::FromIterator;
use stdweb::web::{Element, INode, Node};

/// Bind virtual element to a DOM reference.
#[derive(Clone)]
pub enum VNode {
    /// A bind between `VTag` and `Element`.
    VTag(Box<VTag>),
    /// A bind between `VText` and `TextNode`.
    VText(VText),
    /// A bind between `VComp` and `Element`.
    VComp(VComp),
    /// A holder for a list of other nodes.
    VList(VList),
    /// A holder for any `Node` (necessary for replacing node).
    VRef(Node),
}

impl VDiff for VNode {
    /// Remove VNode from parent.
    fn detach(&mut self, parent: &Element) -> Option<Node> {
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
        parent: &Element,
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        match *self {
            VNode::VTag(ref mut vtag) => vtag.apply(parent, previous_sibling, ancestor),
            VNode::VText(ref mut vtext) => vtext.apply(parent, previous_sibling, ancestor),
            VNode::VComp(ref mut vcomp) => vcomp.apply(parent, previous_sibling, ancestor),
            VNode::VList(ref mut vlist) => vlist.apply(parent, previous_sibling, ancestor),
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

impl Default for VNode {
    fn default() -> Self {
        VNode::VList(VList::default())
    }
}

impl From<VText> for VNode {
    fn from(vtext: VText) -> Self {
        VNode::VText(vtext)
    }
}

impl From<VList> for VNode {
    fn from(vlist: VList) -> Self {
        VNode::VList(vlist)
    }
}

impl From<VTag> for VNode {
    fn from(vtag: VTag) -> Self {
        VNode::VTag(Box::new(vtag))
    }
}

impl From<VComp> for VNode {
    fn from(vcomp: VComp) -> Self {
        VNode::VComp(vcomp)
    }
}

impl<COMP> From<VChild<COMP>> for VNode
where
    COMP: Component,
{
    fn from(vchild: VChild<COMP>) -> Self {
        VNode::VComp(VComp::from(vchild))
    }
}

impl<T: ToString> From<T> for VNode {
    fn from(value: T) -> Self {
        VNode::VText(VText::new(value.to_string()))
    }
}

impl<'a> From<&'a dyn Renderable> for VNode {
    fn from(value: &'a dyn Renderable) -> Self {
        value.render()
    }
}

impl<A: Into<VNode>> FromIterator<A> for VNode {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let vlist = iter.into_iter().fold(VList::default(), |mut acc, x| {
            acc.add_child(x.into());
            acc
        });
        VNode::VList(vlist)
    }
}

impl fmt::Debug for VNode {
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

impl PartialEq for VNode {
    fn eq(&self, other: &VNode) -> bool {
        match (self, other) {
            (VNode::VTag(a), VNode::VTag(b)) => a == b,
            (VNode::VText(a), VNode::VText(b)) => a == b,
            (VNode::VList(a), VNode::VList(b)) => a == b,
            (VNode::VRef(a), VNode::VRef(b)) => a == b,
            // Need to improve PartialEq for VComp before enabling
            (VNode::VComp(_), VNode::VComp(_)) => false,
            _ => false,
        }
    }
}
