//! This module contains the bundle version of an abstract node.

use super::{BComp, BList, BPortal, BSuspense, BTag};
use crate::dom_bundle::{DomBundle, VDiff};
use crate::html::{AnyScope, NodeRef};
use crate::virtual_dom::{Key, VNode, VText};
use gloo::console;
use std::fmt;
use web_sys::{Element, Node};

/// Bind virtual element to a DOM reference.
pub enum BNode {
    /// A bind between `VTag` and `Element`.
    BTag(Box<BTag>),
    /// A bind between `VText` and `TextNode`.
    BText(VText),
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
    pub(crate) fn key(&self) -> Option<&Key> {
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

    /// Returns true if the [VNode] has a key without needlessly cloning the key.
    pub(crate) fn has_key(&self) -> bool {
        match self {
            Self::BComp(bsusp) => bsusp.key().is_some(),
            Self::BList(blist) => blist.key().is_some(),
            Self::BRef(_) | Self::BText(_) => false,
            Self::BTag(vtag) => vtag.key().is_some(),
            Self::BPortal(bportal) => bportal.key().is_some(),
            Self::BSuspense(bsusp) => bsusp.key().is_some(),
        }
    }

    /// Returns the first DOM node that is used to designate the position of the virtual DOM node.
    fn unchecked_first_node(&self) -> Node {
        match self {
            Self::BTag(btag) => btag.reference().clone().into(),
            Self::BText(vtext) => {
                let text_node = vtext.reference.as_ref().expect("VText is not mounted");
                text_node.clone().into()
            }
            Self::BRef(node) => node.clone(),
            Self::BList(_) => unreachable!("no need to get first node of blist"),
            Self::BComp(_) => unreachable!("no need to get first node of bcomp"),
            Self::BSuspense(_) => unreachable!("no need to get first node of bsuspense"),
            Self::BPortal(_) => unreachable!("portals have no first node, they are empty inside"),
        }
    }

    pub(crate) fn move_before(&self, parent: &Element, next_sibling: &Option<Node>) {
        match self {
            Self::BList(blist) => {
                for node in blist.iter().rev() {
                    node.move_before(parent, next_sibling);
                }
            }
            Self::BComp(bcomp) => {
                bcomp
                    .root_bnode()
                    .expect("VComp has no root vnode")
                    .move_before(parent, next_sibling);
            }
            Self::BPortal(_) => {} // no need to move portals
            Self::BSuspense(bsusp) => bsusp.active_node().move_before(parent, next_sibling),
            _ => super::insert_node(&self.unchecked_first_node(), parent, next_sibling.as_ref()),
        };
    }
}

impl DomBundle for BNode {
    /// Remove VNode from parent.
    fn detach(self, parent: &Element) {
        match self {
            Self::BTag(vtag) => vtag.detach(parent),
            Self::BText(vtext) => vtext.detach(parent),
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
            Self::BText(ref vtext) => vtext.shift(next_parent, next_sibling),
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

impl VDiff for VNode {
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

    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
    ) -> NodeRef {
        match self {
            VNode::VTag(vtag) => vtag.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VText(vtext) => vtext.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VComp(vcomp) => vcomp.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VList(vlist) => vlist.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VRef(node) => {
                if let BNode::BRef(ref n) = ancestor {
                    if &node == n {
                        return NodeRef::new(node);
                    }
                }
                let (node_ref, self_) =
                    VNode::VRef(node).attach(parent_scope, parent, next_sibling);
                ancestor.replace(parent, self_);
                node_ref
            }
            VNode::VPortal(vportal) => vportal.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VSuspense(vsuspsense) => {
                vsuspsense.apply(parent_scope, parent, next_sibling, ancestor)
            }
        }
    }
}

impl From<VText> for BNode {
    #[inline]
    fn from(vtext: VText) -> Self {
        Self::BText(vtext)
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
            Self::BText(ref vtext) => vtext.fmt(f),
            Self::BComp(ref bsusp) => bsusp.fmt(f),
            Self::BList(ref vlist) => vlist.fmt(f),
            Self::BRef(ref vref) => write!(f, "VRef ( \"{}\" )", crate::utils::print_node(vref)),
            Self::BPortal(ref vportal) => vportal.fmt(f),
            Self::BSuspense(ref bsusp) => bsusp.fmt(f),
        }
    }
}

impl BNode {
    pub(crate) fn replace(&mut self, parent: &Element, next_node: BNode) {
        let ancestor = std::mem::replace(self, next_node);
        ancestor.detach(parent);
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
