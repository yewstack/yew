//! This module contains the implementation of abstract virtual node.

use super::{VChild, VComp, VDiff, VList, VTag, VText};
use crate::html::{Component, Renderable, Scope};
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::fmt;
use std::iter::FromIterator;
use stdweb::web::{Element, INode, Node};

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

impl<COMP, CHILD> From<VChild<CHILD, COMP>> for VNode<COMP>
where
    COMP: Component,
    CHILD: Component + Renderable<CHILD>,
{
    fn from(vchild: VChild<CHILD, COMP>) -> Self {
        VNode::VComp(VComp::from(vchild))
    }
}

impl<COMP: Component> From<String> for VNode<COMP> {
    fn from(value: String) -> Self {
        VNode::VText(VText::new(value))
    }
}

impl<COMP: Component> From<&String> for VNode<COMP> {
    fn from(value: &String) -> Self {
        VNode::VText(VText::new(value.clone()))
    }
}
/// Macro used to implement From<T> for VNode<COMP> for types that implement to_string.
macro_rules! impl_from_for_vnode {
    ($t: ty) => {
        impl<COMP: Component> From<$t> for VNode<COMP> {
            fn from(value: $t) -> Self {
                VNode::VText(VText::new(value.to_string()))
            }
        }
    }
}

impl_from_for_vnode!(&str);
impl_from_for_vnode!(Cow<'_, &str>);
impl_from_for_vnode!(bool);

impl_from_for_vnode!(usize);
impl_from_for_vnode!(u128);
impl_from_for_vnode!(u64);
impl_from_for_vnode!(u32);
impl_from_for_vnode!(u16);
impl_from_for_vnode!(u8);
impl_from_for_vnode!(std::num::NonZeroU128);
impl_from_for_vnode!(std::num::NonZeroU64);
impl_from_for_vnode!(std::num::NonZeroU32);
impl_from_for_vnode!(std::num::NonZeroU16);
impl_from_for_vnode!(std::num::NonZeroU8);

impl_from_for_vnode!(isize);
impl_from_for_vnode!(i128);
impl_from_for_vnode!(i64);
impl_from_for_vnode!(i32);
impl_from_for_vnode!(i16);
impl_from_for_vnode!(i8);
impl_from_for_vnode!(std::num::NonZeroI128);
impl_from_for_vnode!(std::num::NonZeroI64);
impl_from_for_vnode!(std::num::NonZeroI32);
impl_from_for_vnode!(std::num::NonZeroI16);
impl_from_for_vnode!(std::num::NonZeroI8);



impl<'a, COMP: Component> From<&'a dyn Renderable<COMP>> for VNode<COMP> {
    fn from(value: &'a dyn Renderable<COMP>) -> Self {
        value.view()
    }
}

impl<COMP: Component, T: Into<VNode<COMP>>> From<Option<T>> for VNode<COMP> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(vnode) => vnode.into(),
            None => VNode::VList(VList { childs: vec![] }),
        }
    }
}

impl<COMP: Component, A: Into<VNode<COMP>>> FromIterator<A> for VNode<COMP> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let vlist = iter.into_iter().fold(VList::new(), |mut acc, x| {
            acc.add_child(x.into());
            acc
        });
        VNode::VList(vlist)
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
        match (self, other) {
            (VNode::VTag(vtag_a), VNode::VTag(vtag_b)) => vtag_a == vtag_b,
            (VNode::VText(vtext_a), VNode::VText(vtext_b)) => vtext_a == vtext_b,
            _ => false, // TODO: Implement other variants
        }
    }
}
