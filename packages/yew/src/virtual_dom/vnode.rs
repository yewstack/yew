//! This module contains the implementation of abstract virtual node.

use super::{Key, VChild, VComp, VDiff, VList, VPortal, VSuspense, VTag, VText};
use crate::html::{AnyScope, BaseComponent, NodeRef};
use gloo::console;
use std::cmp::PartialEq;
use std::fmt;
use std::iter::FromIterator;
use wasm_bindgen::JsCast;

use web_sys::{Element, Node};

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
    pub fn key(&self) -> Option<Key> {
        match self {
            VNode::VComp(vcomp) => vcomp.key.clone(),
            VNode::VList(vlist) => vlist.key.clone(),
            VNode::VRef(_) => None,
            VNode::VTag(vtag) => vtag.key.clone(),
            VNode::VText(_) => None,
            VNode::VPortal(vportal) => vportal.node.key(),
            VNode::VSuspense(vsuspense) => vsuspense.key.clone(),
        }
    }

    /// Returns true if the [VNode] has a key without needlessly cloning the key.
    pub fn has_key(&self) -> bool {
        match self {
            VNode::VComp(vcomp) => vcomp.key.is_some(),
            VNode::VList(vlist) => vlist.key.is_some(),
            VNode::VRef(_) | VNode::VText(_) => false,
            VNode::VTag(vtag) => vtag.key.is_some(),
            VNode::VPortal(vportal) => vportal.node.has_key(),
            VNode::VSuspense(vsuspense) => vsuspense.key.is_some(),
        }
    }

    /// Returns the first DOM node if available
    pub(crate) fn first_node(&self) -> Option<Node> {
        match self {
            VNode::VTag(vtag) => vtag.reference().cloned().map(JsCast::unchecked_into),
            VNode::VText(vtext) => vtext
                .reference
                .as_ref()
                .cloned()
                .map(JsCast::unchecked_into),
            VNode::VComp(vcomp) => vcomp.node_ref.get(),
            VNode::VList(vlist) => vlist.get(0).and_then(VNode::first_node),
            VNode::VRef(node) => Some(node.clone()),
            VNode::VPortal(vportal) => vportal.next_sibling(),
            VNode::VSuspense(vsuspense) => vsuspense.first_node(),
        }
    }

    /// Returns the first DOM node that is used to designate the position of the virtual DOM node.
    pub(crate) fn unchecked_first_node(&self) -> Node {
        match self {
            VNode::VTag(vtag) => vtag
                .reference()
                .expect("VTag is not mounted")
                .clone()
                .into(),
            VNode::VText(vtext) => {
                let text_node = vtext.reference.as_ref().expect("VText is not mounted");
                text_node.clone().into()
            }
            VNode::VComp(vcomp) => vcomp.node_ref.get().unwrap_or_else(|| {
                #[cfg(not(debug_assertions))]
                panic!("no node_ref; VComp should be mounted");

                #[cfg(debug_assertions)]
                panic!(
                    "no node_ref; VComp should be mounted after: {:?}",
                    crate::virtual_dom::vcomp::get_event_log(vcomp.id),
                );
            }),
            VNode::VList(vlist) => vlist
                .get(0)
                .expect("VList is not mounted")
                .unchecked_first_node(),
            VNode::VRef(node) => node.clone(),
            VNode::VPortal(_) => panic!("portals have no first node, they are empty inside"),
            VNode::VSuspense(vsuspense) => {
                vsuspense.first_node().expect("VSuspense is not mounted")
            }
        }
    }

    pub(crate) fn move_before(&self, parent: &Element, next_sibling: &Option<Node>) {
        match self {
            VNode::VList(vlist) => {
                for node in vlist.iter() {
                    node.move_before(parent, next_sibling);
                }
            }
            VNode::VComp(vcomp) => {
                vcomp
                    .root_vnode()
                    .expect("VComp has no root vnode")
                    .move_before(parent, next_sibling);
            }
            VNode::VPortal(_) => {} // no need to move portals
            _ => super::insert_node(&self.unchecked_first_node(), parent, next_sibling.as_ref()),
        };
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
                    console::warn!("Node not found to remove VRef");
                }
            }
            VNode::VPortal(ref mut vportal) => vportal.detach(parent),
            VNode::VSuspense(ref mut vsuspense) => vsuspense.detach(parent),
        }
    }

    fn shift(&self, previous_parent: &Element, next_parent: &Element, next_sibling: NodeRef) {
        match *self {
            VNode::VTag(ref vtag) => vtag.shift(previous_parent, next_parent, next_sibling),
            VNode::VText(ref vtext) => vtext.shift(previous_parent, next_parent, next_sibling),
            VNode::VComp(ref vcomp) => vcomp.shift(previous_parent, next_parent, next_sibling),
            VNode::VList(ref vlist) => vlist.shift(previous_parent, next_parent, next_sibling),
            VNode::VRef(ref node) => {
                previous_parent.remove_child(node).unwrap();
                next_parent
                    .insert_before(node, next_sibling.get().as_ref())
                    .unwrap();
            }
            VNode::VPortal(ref vportal) => {
                vportal.shift(previous_parent, next_parent, next_sibling)
            }
            VNode::VSuspense(ref vsuspense) => {
                vsuspense.shift(previous_parent, next_parent, next_sibling)
            }
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
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
                if let Some(mut ancestor) = ancestor {
                    if let VNode::VRef(n) = &ancestor {
                        if node == n {
                            return NodeRef::new(node.clone());
                        }
                    }
                    ancestor.detach(parent);
                }
                super::insert_node(node, parent, next_sibling.get().as_ref());
                NodeRef::new(node.clone())
            }
            VNode::VPortal(ref mut vportal) => {
                vportal.apply(parent_scope, parent, next_sibling, ancestor)
            }
            VNode::VSuspense(ref mut vsuspense) => {
                vsuspense.apply(parent_scope, parent, next_sibling, ancestor)
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
    use futures::future::{FutureExt, LocalBoxFuture};

    use super::*;

    impl VNode {
        // Boxing is needed here, due to: https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
        pub(crate) fn render_to_string<'a>(
            &'a self,
            w: &'a mut String,
            parent_scope: &'a AnyScope,
        ) -> LocalBoxFuture<'a, ()> {
            async move {
                match self {
                    VNode::VTag(vtag) => vtag.render_to_string(w, parent_scope).await,
                    VNode::VText(vtext) => vtext.render_to_string(w).await,
                    VNode::VComp(vcomp) => vcomp.render_to_string(w, parent_scope).await,
                    VNode::VList(vlist) => vlist.render_to_string(w, parent_scope).await,
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
                        vsuspense.render_to_string(w, parent_scope).await
                    }
                }
            }
            .boxed_local()
        }
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let document = gloo_utils::document();
        let vref_node_1 = VNode::VRef(document.create_element("i").unwrap().into());
        let vref_node_2 = VNode::VRef(document.create_element("b").unwrap().into());

        let layout1 = TestLayout {
            name: "1",
            node: vref_node_1,
            expected: "<i></i>",
        };

        let layout2 = TestLayout {
            name: "2",
            node: vref_node_2,
            expected: "<b></b>",
        };

        diff_layouts(vec![layout1, layout2]);
    }
}
