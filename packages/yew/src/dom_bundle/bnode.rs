//! This module contains the bundle version of an abstract node [BNode]

use super::{BComp, BList, BPortal, BSuspense, BTag, BText};
use crate::dom_bundle::{DomBundle, Reconcilable};
use crate::html::{AnyScope, NodeRef};
use crate::virtual_dom::{Key, VNode};
use gloo::console;
use std::fmt;
use web_sys::{Element, Node};

/// The bundle implementation to [VNode].
pub enum BNode {
    /// A bind between `VTag` and `Element`.
    BTag(Box<BTag>),
    /// A bind between `VText` and `TextNode`.
    BText(BText),
    /// A bind between `VComp` and `Element`.
    BComp(BComp),
    /// A holder for a list of other nodes.
    BList(BList),
    /// A portal to another part of the document
    BPortal(BPortal),
    /// A holder for any `Node` (necessary for replacing node).
    BRef(Node),
    /// A suspendible document fragment.
    BSuspense(Box<BSuspense>),
}

impl BNode {
    /// Get the key of the underlying node
    pub(super) fn key(&self) -> Option<&Key> {
        match self {
            Self::BComp(bsusp) => bsusp.key(),
            Self::BList(blist) => blist.key(),
            Self::BRef(_) => None,
            Self::BTag(btag) => btag.key(),
            Self::BText(_) => None,
            Self::BPortal(bportal) => bportal.key(),
            Self::BSuspense(bsusp) => bsusp.key(),
        }
    }
}

impl DomBundle for BNode {
    /// Remove VNode from parent.
    fn detach(self, parent: &Element) {
        match self {
            Self::BTag(vtag) => vtag.detach(parent),
            Self::BText(btext) => btext.detach(parent),
            Self::BComp(bsusp) => bsusp.detach(parent),
            Self::BList(blist) => blist.detach(parent),
            Self::BRef(ref node) => {
                if parent.remove_child(node).is_err() {
                    console::warn!("Node not found to remove VRef");
                }
            }
            Self::BPortal(bportal) => bportal.detach(parent),
            Self::BSuspense(bsusp) => bsusp.detach(parent),
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        match self {
            Self::BTag(ref vtag) => vtag.shift(next_parent, next_sibling),
            Self::BText(ref btext) => btext.shift(next_parent, next_sibling),
            Self::BComp(ref bsusp) => bsusp.shift(next_parent, next_sibling),
            Self::BList(ref vlist) => vlist.shift(next_parent, next_sibling),
            Self::BRef(ref node) => {
                next_parent
                    .insert_before(node, next_sibling.get().as_ref())
                    .unwrap();
            }
            Self::BPortal(ref vportal) => vportal.shift(next_parent, next_sibling),
            Self::BSuspense(ref vsuspense) => vsuspense.shift(next_parent, next_sibling),
        }
    }
}

impl Reconcilable for VNode {
    type Bundle = BNode;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        match self {
            VNode::VTag(vtag) => {
                let (node_ref, tag) = vtag.attach(parent_scope, parent, next_sibling);
                (node_ref, tag.into())
            }
            VNode::VText(vtext) => {
                let (node_ref, text) = vtext.attach(parent_scope, parent, next_sibling);
                (node_ref, text.into())
            }
            VNode::VComp(vcomp) => {
                let (node_ref, comp) = vcomp.attach(parent_scope, parent, next_sibling);
                (node_ref, comp.into())
            }
            VNode::VList(vlist) => {
                let (node_ref, list) = vlist.attach(parent_scope, parent, next_sibling);
                (node_ref, list.into())
            }
            VNode::VRef(node) => {
                super::insert_node(&node, parent, next_sibling.get().as_ref());
                (NodeRef::new(node.clone()), BNode::BRef(node))
            }
            VNode::VPortal(vportal) => {
                let (node_ref, portal) = vportal.attach(parent_scope, parent, next_sibling);
                (node_ref, portal.into())
            }
            VNode::VSuspense(vsuspsense) => {
                let (node_ref, suspsense) = vsuspsense.attach(parent_scope, parent, next_sibling);
                (node_ref, suspsense.into())
            }
        }
    }

    fn reconcile_node(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        self.reconcile(parent_scope, parent, next_sibling, bundle)
    }

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        match self {
            VNode::VTag(vtag) => vtag.reconcile_node(parent_scope, parent, next_sibling, bundle),
            VNode::VText(vtext) => vtext.reconcile_node(parent_scope, parent, next_sibling, bundle),
            VNode::VComp(vcomp) => vcomp.reconcile_node(parent_scope, parent, next_sibling, bundle),
            VNode::VList(vlist) => vlist.reconcile_node(parent_scope, parent, next_sibling, bundle),
            VNode::VRef(node) => {
                let _existing = match bundle {
                    BNode::BRef(ref n) if &node == n => n,
                    _ => {
                        return VNode::VRef(node).replace(
                            parent_scope,
                            parent,
                            next_sibling,
                            bundle,
                        );
                    }
                };
                NodeRef::new(node)
            }
            VNode::VPortal(vportal) => {
                vportal.reconcile_node(parent_scope, parent, next_sibling, bundle)
            }
            VNode::VSuspense(vsuspsense) => {
                vsuspsense.reconcile_node(parent_scope, parent, next_sibling, bundle)
            }
        }
    }
}

impl From<BText> for BNode {
    #[inline]
    fn from(btext: BText) -> Self {
        Self::BText(btext)
    }
}

impl From<BList> for BNode {
    #[inline]
    fn from(blist: BList) -> Self {
        Self::BList(blist)
    }
}

impl From<BTag> for BNode {
    #[inline]
    fn from(btag: BTag) -> Self {
        Self::BTag(Box::new(btag))
    }
}

impl From<BComp> for BNode {
    #[inline]
    fn from(bcomp: BComp) -> Self {
        Self::BComp(bcomp)
    }
}

impl From<BPortal> for BNode {
    #[inline]
    fn from(bportal: BPortal) -> Self {
        Self::BPortal(bportal)
    }
}

impl From<BSuspense> for BNode {
    #[inline]
    fn from(bsusp: BSuspense) -> Self {
        Self::BSuspense(Box::new(bsusp))
    }
}

impl fmt::Debug for BNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BTag(ref vtag) => vtag.fmt(f),
            Self::BText(ref btext) => btext.fmt(f),
            Self::BComp(ref bsusp) => bsusp.fmt(f),
            Self::BList(ref vlist) => vlist.fmt(f),
            Self::BRef(ref vref) => write!(f, "VRef ( \"{}\" )", crate::utils::print_node(vref)),
            Self::BPortal(ref vportal) => vportal.fmt(f),
            Self::BSuspense(ref bsusp) => bsusp.fmt(f),
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
