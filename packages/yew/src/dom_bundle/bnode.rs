//! This module contains the bundle version of an abstract node [BNode]

use std::fmt;

use web_sys::{Element, Node};

use super::{BComp, BList, BPortal, BRaw, BSubtree, BSuspense, BTag, BText, DomSlot};
use crate::dom_bundle::{Reconcilable, ReconcileTarget};
use crate::html::AnyScope;
use crate::utils::RcExt;
use crate::virtual_dom::{Key, VNode};

/// The bundle implementation to [VNode].
pub(super) enum BNode {
    /// A bind between `VTag` and `Element`.
    Tag(Box<BTag>),
    /// A bind between `VText` and `TextNode`.
    Text(BText),
    /// A bind between `VComp` and `Element`.
    Comp(BComp),
    /// A holder for a list of other nodes.
    List(BList),
    /// A portal to another part of the document
    Portal(BPortal),
    /// A holder for any `Node` (necessary for replacing node).
    Ref(Node),
    /// A suspendible document fragment.
    Suspense(Box<BSuspense>),
    /// A raw HTML string, represented by [`AttrValue`](crate::AttrValue).
    Raw(BRaw),
}

impl BNode {
    /// Get the key of the underlying node
    pub fn key(&self) -> Option<&Key> {
        match self {
            Self::Comp(bsusp) => bsusp.key(),
            Self::List(blist) => blist.key(),
            Self::Ref(_) => None,
            Self::Tag(btag) => btag.key(),
            Self::Text(_) => None,
            Self::Portal(bportal) => bportal.key(),
            Self::Suspense(bsusp) => bsusp.key(),
            Self::Raw(_) => None,
        }
    }
}

impl ReconcileTarget for BNode {
    /// Remove VNode from parent.
    fn detach(self, root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        match self {
            Self::Tag(vtag) => vtag.detach(root, parent, parent_to_detach),
            Self::Text(btext) => btext.detach(root, parent, parent_to_detach),
            Self::Comp(bsusp) => bsusp.detach(root, parent, parent_to_detach),
            Self::List(blist) => blist.detach(root, parent, parent_to_detach),
            Self::Ref(ref node) => {
                // Always remove user-defined nodes to clear possible parent references of them
                if parent.remove_child(node).is_err() {
                    tracing::warn!("Node not found to remove VRef");
                }
            }
            Self::Portal(bportal) => bportal.detach(root, parent, parent_to_detach),
            Self::Suspense(bsusp) => bsusp.detach(root, parent, parent_to_detach),
            Self::Raw(raw) => raw.detach(root, parent, parent_to_detach),
        }
    }

    fn shift(&self, next_parent: &Element, slot: DomSlot) -> DomSlot {
        match self {
            Self::Tag(ref vtag) => vtag.shift(next_parent, slot),
            Self::Text(ref btext) => btext.shift(next_parent, slot),
            Self::Comp(ref bsusp) => bsusp.shift(next_parent, slot),
            Self::List(ref vlist) => vlist.shift(next_parent, slot),
            Self::Ref(ref node) => {
                slot.insert(next_parent, node);

                DomSlot::at(node.clone())
            }
            Self::Portal(ref vportal) => vportal.shift(next_parent, slot),
            Self::Suspense(ref vsuspense) => vsuspense.shift(next_parent, slot),
            Self::Raw(ref braw) => braw.shift(next_parent, slot),
        }
    }
}

impl Reconcilable for VNode {
    type Bundle = BNode;

    fn attach(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
    ) -> (DomSlot, Self::Bundle) {
        match self {
            VNode::VTag(vtag) => {
                let (node_ref, tag) =
                    RcExt::unwrap_or_clone(vtag).attach(root, parent_scope, parent, slot);
                (node_ref, tag.into())
            }
            VNode::VText(vtext) => {
                let (node_ref, text) = vtext.attach(root, parent_scope, parent, slot);
                (node_ref, text.into())
            }
            VNode::VComp(vcomp) => {
                let (node_ref, comp) =
                    RcExt::unwrap_or_clone(vcomp).attach(root, parent_scope, parent, slot);
                (node_ref, comp.into())
            }
            VNode::VList(vlist) => {
                let (node_ref, list) =
                    RcExt::unwrap_or_clone(vlist).attach(root, parent_scope, parent, slot);
                (node_ref, list.into())
            }
            VNode::VRef(node) => {
                slot.insert(parent, &node);
                (DomSlot::at(node.clone()), BNode::Ref(node))
            }
            VNode::VPortal(vportal) => {
                let (node_ref, portal) =
                    RcExt::unwrap_or_clone(vportal).attach(root, parent_scope, parent, slot);
                (node_ref, portal.into())
            }
            VNode::VSuspense(vsuspsense) => {
                let (node_ref, suspsense) =
                    RcExt::unwrap_or_clone(vsuspsense).attach(root, parent_scope, parent, slot);
                (node_ref, suspsense.into())
            }
            VNode::VRaw(vraw) => {
                let (node_ref, raw) = vraw.attach(root, parent_scope, parent, slot);
                (node_ref, raw.into())
            }
        }
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut BNode,
    ) -> DomSlot {
        self.reconcile(root, parent_scope, parent, slot, bundle)
    }

    fn reconcile(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut BNode,
    ) -> DomSlot {
        match self {
            VNode::VTag(vtag) => RcExt::unwrap_or_clone(vtag).reconcile_node(
                root,
                parent_scope,
                parent,
                slot,
                bundle,
            ),
            VNode::VText(vtext) => vtext.reconcile_node(root, parent_scope, parent, slot, bundle),
            VNode::VComp(vcomp) => RcExt::unwrap_or_clone(vcomp).reconcile_node(
                root,
                parent_scope,
                parent,
                slot,
                bundle,
            ),
            VNode::VList(vlist) => RcExt::unwrap_or_clone(vlist).reconcile_node(
                root,
                parent_scope,
                parent,
                slot,
                bundle,
            ),
            VNode::VRef(node) => match bundle {
                BNode::Ref(ref n) if &node == n => DomSlot::at(node),
                _ => VNode::VRef(node).replace(root, parent_scope, parent, slot, bundle),
            },
            VNode::VPortal(vportal) => RcExt::unwrap_or_clone(vportal).reconcile_node(
                root,
                parent_scope,
                parent,
                slot,
                bundle,
            ),
            VNode::VSuspense(vsuspsense) => RcExt::unwrap_or_clone(vsuspsense).reconcile_node(
                root,
                parent_scope,
                parent,
                slot,
                bundle,
            ),
            VNode::VRaw(vraw) => vraw.reconcile_node(root, parent_scope, parent, slot, bundle),
        }
    }
}

impl From<BText> for BNode {
    #[inline]
    fn from(btext: BText) -> Self {
        Self::Text(btext)
    }
}

impl From<BList> for BNode {
    #[inline]
    fn from(blist: BList) -> Self {
        Self::List(blist)
    }
}

impl From<BTag> for BNode {
    #[inline]
    fn from(btag: BTag) -> Self {
        Self::Tag(Box::new(btag))
    }
}

impl From<BComp> for BNode {
    #[inline]
    fn from(bcomp: BComp) -> Self {
        Self::Comp(bcomp)
    }
}

impl From<BPortal> for BNode {
    #[inline]
    fn from(bportal: BPortal) -> Self {
        Self::Portal(bportal)
    }
}

impl From<BSuspense> for BNode {
    #[inline]
    fn from(bsusp: BSuspense) -> Self {
        Self::Suspense(Box::new(bsusp))
    }
}

impl From<BRaw> for BNode {
    #[inline]
    fn from(braw: BRaw) -> Self {
        Self::Raw(braw)
    }
}

impl fmt::Debug for BNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Tag(ref vtag) => vtag.fmt(f),
            Self::Text(ref btext) => btext.fmt(f),
            Self::Comp(ref bsusp) => bsusp.fmt(f),
            Self::List(ref vlist) => vlist.fmt(f),
            Self::Ref(ref vref) => write!(f, "VRef ( \"{}\" )", crate::utils::print_node(vref)),
            Self::Portal(ref vportal) => vportal.fmt(f),
            Self::Suspense(ref bsusp) => bsusp.fmt(f),
            Self::Raw(ref braw) => braw.fmt(f),
        }
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::{Fragment, Hydratable};

    impl Hydratable for VNode {
        fn hydrate(
            self,
            root: &BSubtree,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> Self::Bundle {
            match self {
                VNode::VTag(vtag) => RcExt::unwrap_or_clone(vtag)
                    .hydrate(root, parent_scope, parent, fragment)
                    .into(),
                VNode::VText(vtext) => vtext.hydrate(root, parent_scope, parent, fragment).into(),
                VNode::VComp(vcomp) => RcExt::unwrap_or_clone(vcomp)
                    .hydrate(root, parent_scope, parent, fragment)
                    .into(),
                VNode::VList(vlist) => RcExt::unwrap_or_clone(vlist)
                    .hydrate(root, parent_scope, parent, fragment)
                    .into(),
                // You cannot hydrate a VRef.
                VNode::VRef(_) => {
                    panic!(
                        "VRef is not hydratable. Try moving it to a component mounted after an \
                         effect."
                    )
                }
                // You cannot hydrate a VPortal.
                VNode::VPortal(_) => {
                    panic!(
                        "VPortal is not hydratable. Try creating your portal by delaying it with \
                         use_effect."
                    )
                }
                VNode::VSuspense(vsuspense) => RcExt::unwrap_or_clone(vsuspense)
                    .hydrate(root, parent_scope, parent, fragment)
                    .into(),
                VNode::VRaw(vraw) => vraw.hydrate(root, parent_scope, parent, fragment).into(),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod layout_tests {
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let document = gloo::utils::document();
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
