//! This module contains the implementation of abstract virtual node.

use super::{Key, VChild, VComp, VList, VPortal, VSuspense, VTag, VText};
use crate::html::BaseComponent;
use std::cmp::PartialEq;
use std::fmt;
use std::iter::FromIterator;
use web_sys::Node;

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
    /// A portal to another part of the document
    VPortal(VPortal),
    /// A holder for any `Node` (necessary for replacing node).
    VRef(Node),
    /// A suspendible document fragment.
    VSuspense(VSuspense),
}

impl VNode {
    pub fn key(&self) -> Option<&Key> {
        match self {
            VNode::VComp(vcomp) => vcomp.key.as_ref(),
            VNode::VList(vlist) => vlist.key.as_ref(),
            VNode::VRef(_) => None,
            VNode::VTag(vtag) => vtag.key.as_ref(),
            VNode::VText(_) => None,
            VNode::VPortal(vportal) => vportal.node.key(),
            VNode::VSuspense(vsuspense) => vsuspense.key.as_ref(),
        }
    }

    /// Returns true if the [VNode] has a key.
    pub fn has_key(&self) -> bool {
        self.key().is_some()
    }
}

impl Default for VNode {
    fn default() -> Self {
        VNode::VList(VList::default())
    }
}

impl From<VText> for VNode {
    #[inline]
    fn from(vtext: VText) -> Self {
        VNode::VText(vtext)
    }
}

impl From<VList> for VNode {
    #[inline]
    fn from(vlist: VList) -> Self {
        VNode::VList(vlist)
    }
}

impl From<VTag> for VNode {
    #[inline]
    fn from(vtag: VTag) -> Self {
        VNode::VTag(Box::new(vtag))
    }
}

impl From<VComp> for VNode {
    #[inline]
    fn from(vcomp: VComp) -> Self {
        VNode::VComp(vcomp)
    }
}

impl From<VSuspense> for VNode {
    #[inline]
    fn from(vsuspense: VSuspense) -> Self {
        VNode::VSuspense(vsuspense)
    }
}

impl From<VPortal> for VNode {
    #[inline]
    fn from(vportal: VPortal) -> Self {
        VNode::VPortal(vportal)
    }
}

impl<COMP> From<VChild<COMP>> for VNode
where
    COMP: BaseComponent,
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

impl<A: Into<VNode>> FromIterator<A> for VNode {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        VNode::VList(VList::with_children(
            iter.into_iter().map(|n| n.into()).collect(),
            None,
        ))
    }
}

impl fmt::Debug for VNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VNode::VTag(ref vtag) => vtag.fmt(f),
            VNode::VText(ref vtext) => vtext.fmt(f),
            VNode::VComp(ref vcomp) => vcomp.fmt(f),
            VNode::VList(ref vlist) => vlist.fmt(f),
            VNode::VRef(ref vref) => write!(f, "VRef ( \"{}\" )", crate::utils::print_node(vref)),
            VNode::VPortal(ref vportal) => vportal.fmt(f),
            VNode::VSuspense(ref vsuspense) => vsuspense.fmt(f),
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
            // TODO: Need to improve PartialEq for VComp before enabling.
            (VNode::VComp(_), VNode::VComp(_)) => false,
            _ => false,
        }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;
    use crate::html::AnyScope;
    use futures::future::{FutureExt, LocalBoxFuture};

    impl VNode {
        // Boxing is needed here, due to: https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
        pub(crate) fn render_to_string<'a>(
            &'a self,
            w: &'a mut String,
            parent_scope: &'a AnyScope,
            hydratable: bool,
        ) -> LocalBoxFuture<'a, ()> {
            async move {
                match self {
                    VNode::VTag(vtag) => vtag.render_to_string(w, parent_scope, hydratable).await,
                    VNode::VText(vtext) => {
                        vtext.render_to_string(w, parent_scope, hydratable).await
                    }
                    VNode::VComp(vcomp) => {
                        vcomp.render_to_string(w, parent_scope, hydratable).await
                    }
                    VNode::VList(vlist) => {
                        vlist.render_to_string(w, parent_scope, hydratable).await
                    }
                    // We are pretty safe here as it's not possible to get a web_sys::Node without DOM
                    // support in the first place.
                    //
                    // The only exception would be to use `ServerRenderer` in a browser or wasm32 environment with
                    // jsdom present.
                    VNode::VRef(_) => {
                        panic!("VRef is not possible to be rendered in to a string.")
                    }
                    // Portals are not rendered.
                    VNode::VPortal(_) => {}
                    VNode::VSuspense(vsuspense) => {
                        vsuspense
                            .render_to_string(w, parent_scope, hydratable)
                            .await
                    }
                }
            }
            .boxed_local()
        }
    }
}
