//! This module contains the implementation of abstract virtual node.

use super::{Key, VChild, VComp, VList, VPortal, VSuspense, VTag, VText};
use crate::dom_bundle::{DomBundle, VDiff};
use crate::html::{AnyScope, BaseComponent, NodeRef};
use gloo::console;
use std::cmp::PartialEq;
use std::fmt;
use std::iter::FromIterator;

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
            VNode::VSuspense(_) => unreachable!("no need to get the first node of a suspense"),
        }
    }

    pub(crate) fn move_before(&self, parent: &Element, next_sibling: &Option<Node>) {
        match self {
            VNode::VList(vlist) => {
                for node in vlist.iter().rev() {
                    node.move_before(parent, next_sibling);
                }
            }
            VNode::VComp(vcomp) => {
                vcomp
                    .root_vnode()
                    .expect("VComp has no root vnode")
                    .move_before(parent, next_sibling);
            }
            VNode::VSuspense(vsuspense) => {
                vsuspense.active_node().move_before(parent, next_sibling)
            }
            VNode::VPortal(_) => {} // no need to move portals
            _ => super::insert_node(&self.unchecked_first_node(), parent, next_sibling.as_ref()),
        };
    }
}

impl DomBundle for VNode {
    /// Remove VNode from parent.
    fn detach(self, parent: &Element) {
        match self {
            VNode::VTag(vtag) => vtag.detach(parent),
            VNode::VText(vtext) => vtext.detach(parent),
            VNode::VComp(vcomp) => vcomp.detach(parent),
            VNode::VList(vlist) => vlist.detach(parent),
            VNode::VRef(ref node) => {
                if parent.remove_child(node).is_err() {
                    console::warn!("Node not found to remove VRef");
                }
            }
            VNode::VPortal(vportal) => vportal.detach(parent),
            VNode::VSuspense(vsuspense) => vsuspense.detach(parent),
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        match *self {
            VNode::VTag(ref vtag) => vtag.shift(next_parent, next_sibling),
            VNode::VText(ref vtext) => vtext.shift(next_parent, next_sibling),
            VNode::VComp(ref vcomp) => vcomp.shift(next_parent, next_sibling),
            VNode::VList(ref vlist) => vlist.shift(next_parent, next_sibling),
            VNode::VRef(ref node) => {
                next_parent
                    .insert_before(node, next_sibling.get().as_ref())
                    .unwrap();
            }
            VNode::VPortal(ref vportal) => vportal.shift(next_parent, next_sibling),
            VNode::VSuspense(ref vsuspense) => vsuspense.shift(next_parent, next_sibling),
        }
    }
}

impl VDiff for VNode {
    type Bundle = VNode;

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
                (NodeRef::new(node.clone()), VNode::VRef(node))
            }
            VNode::VPortal(vportal) => {
                let (node_ref, portal) = vportal.attach(parent_scope, parent, next_sibling);
                (node_ref, portal.into())
            }
            VNode::VSuspense(vsuspense) => {
                let (node_ref, suspense) = vsuspense.attach(parent_scope, parent, next_sibling);
                (node_ref, suspense.into())
            }
        }
    }

    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut VNode,
    ) -> NodeRef {
        match self {
            VNode::VTag(vtag) => vtag.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VText(vtext) => vtext.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VComp(vcomp) => vcomp.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VList(vlist) => vlist.apply(parent_scope, parent, next_sibling, ancestor),
            VNode::VRef(node) => {
                if let VNode::VRef(ref n) = ancestor {
                    if &node == n {
                        return NodeRef::new(node);
                    }
                }
                let (node_ref, self_) =
                    VNode::VRef(node).attach(parent_scope, parent, next_sibling);
                ancestor.replace(parent, self_);
                node_ref
            }
            VNode::VSuspense(vsuspense) => {
                vsuspense.apply(parent_scope, parent, next_sibling, ancestor)
            }
            VNode::VPortal(vportal) => vportal.apply(parent_scope, parent, next_sibling, ancestor),
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

impl VNode {
    pub(crate) fn replace(&mut self, parent: &Element, next_node: VNode) {
        let ancestor = std::mem::replace(self, next_node);
        ancestor.detach(parent);
    }

    #[cfg(test)]
    pub(crate) fn apply_sequentially(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut Option<VNode>,
    ) -> NodeRef {
        match ancestor {
            None => {
                let (node_ref, node) = self.attach(parent_scope, parent, next_sibling);
                *ancestor = Some(node);
                node_ref
            }
            Some(ref mut ancestor) => self.apply(parent_scope, parent, next_sibling, ancestor),
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
