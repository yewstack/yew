//! This module contains the implementation of abstract virtual node.

use super::{VChild, VComp, VDiff, VList, VTag, VText};
use crate::html::{AnyScope, Component, Renderable};
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use log::warn;
use std::cmp::PartialEq;
use std::fmt;
use std::iter::FromIterator;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, INode, Node};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node};
    }
}

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

impl From<&VNode> for Node {
    fn from(vnode: &VNode) -> Self {
        match vnode {
            VNode::VTag(vtag) => vtag
                .reference
                .as_ref()
                .expect("VTag should always wrap a node")
                .clone()
                .into(),
            VNode::VText(vtext) => {
                let text_node = vtext
                    .reference
                    .as_ref()
                    .expect("VText should always wrap a node");
                cfg_match! {
                    feature = "std_web" => text_node.as_node(),
                    feature = "web_sys" => text_node.clone().into(),
                }
            }
            VNode::VComp(vcomp) => vcomp
                .node_ref
                .get()
                .expect("VComp should always wrap a node"),
            VNode::VList(vlist) => (&vlist.children[0]).into(),
            VNode::VRef(node) => node.clone(),
        }
    }
}

impl VNode {
    pub fn key(&self) -> &Option<String> {
        match self {
            VNode::VTag(vtag) => &vtag.key,
            VNode::VText(_) => &None,
            VNode::VComp(vcomp) => &vcomp.key,
            VNode::VList(vlist) => &vlist.key,
            VNode::VRef(_) => &None,
        }
    }
}

impl VDiff for VNode {
    /// Remove VNode from parent.
    fn detach(&mut self, parent: &Element) {
        match *self {
            VNode::VTag(ref mut vtag) => vtag.detach(parent),
            VNode::VText(ref mut vtext) => vtext.detach(parent),
            VNode::VComp(ref mut vcomp) => vcomp.detach(parent),
            VNode::VList(ref mut vlist) => vlist.detach(parent),
            VNode::VRef(ref node) => {
                if parent.remove_child(node).is_err() {
                    warn!("Node not found to remove VRef");
                }
            }
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        mut next_sibling: Option<Node>,
        ancestor: Option<VNode>,
    ) -> Node {
        match *self {
            VNode::VTag(ref mut vtag) => vtag.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VText(ref mut vtext) => {
                vtext.apply(parent_scope, parent, next_sibling, ancestor)
            }
            VNode::VComp(ref mut vcomp) => {
                vcomp.apply(parent_scope, parent, next_sibling, ancestor)
            }
            VNode::VList(ref mut vlist) => {
                vlist.apply(parent_scope, parent, next_sibling, ancestor)
            }
            VNode::VRef(ref mut node) => {
                let _ancestor = if let Some(ancestor) = ancestor {
                    if let VNode::VRef(n) = &ancestor {
                        if node == n {
                            return node.clone();
                        }
                    }

                    next_sibling = Some((&ancestor).into());
                    Some(super::DelayDetach { ancestor, parent })
                } else {
                    None
                };

                super::insert_node(node, parent, next_sibling);
                node.clone()
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
            VNode::VComp(ref vcomp) => vcomp.fmt(f),
            VNode::VList(ref vlist) => vlist.fmt(f),
            VNode::VRef(ref vref) => vref.fmt(f),
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
