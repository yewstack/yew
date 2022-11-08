//! This module contains the bundle version of an abstract node [BNode]

use std::fmt;

use web_sys::{Element, Node};

use super::{BComp, BList, BPortal, BRaw, BSubtree, BSuspense, BTag, BText};
use crate::dom_bundle::{Reconcilable, ReconcileTarget};
use crate::html::{AnyScope, NodeRef};
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

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) -> NodeRef {
        match self {
            Self::Tag(ref vtag) => vtag.shift(next_parent, next_sibling),
            Self::Text(ref btext) => btext.shift(next_parent, next_sibling),
            Self::Comp(ref bsusp) => bsusp.shift(next_parent, next_sibling),
            Self::List(ref vlist) => vlist.shift(next_parent, next_sibling),
            Self::Ref(ref node) => {
                next_parent
                    .insert_before(node, next_sibling.get().as_ref())
                    .unwrap();

                NodeRef::new(node.clone())
            }
            Self::Portal(ref vportal) => vportal.shift(next_parent, next_sibling),
            Self::Suspense(ref vsuspense) => vsuspense.shift(next_parent, next_sibling),
            Self::Raw(ref braw) => braw.shift(next_parent, next_sibling),
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
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        match self {
            VNode::VTag(vtag) => {
                let (node_ref, tag) = vtag.attach(root, parent_scope, parent, next_sibling);
                (node_ref, tag.into())
            }
            VNode::VText(vtext) => {
                let (node_ref, text) = vtext.attach(root, parent_scope, parent, next_sibling);
                (node_ref, text.into())
            }
            VNode::VComp(vcomp) => {
                let (node_ref, comp) = vcomp.attach(root, parent_scope, parent, next_sibling);
                (node_ref, comp.into())
            }
            VNode::VList(vlist) => {
                let (node_ref, list) = vlist.attach(root, parent_scope, parent, next_sibling);
                (node_ref, list.into())
            }
            VNode::VRef(node) => {
                super::insert_node(&node, parent, next_sibling.get().as_ref());
                (NodeRef::new(node.clone()), BNode::Ref(node))
            }
            VNode::VPortal(vportal) => {
                let (node_ref, portal) = vportal.attach(root, parent_scope, parent, next_sibling);
                (node_ref, portal.into())
            }
            VNode::VSuspense(vsuspsense) => {
                let (node_ref, suspsense) =
                    vsuspsense.attach(root, parent_scope, parent, next_sibling);
                (node_ref, suspsense.into())
            }
            VNode::VRaw(vraw) => {
                let (node_ref, raw) = vraw.attach(root, parent_scope, parent, next_sibling);
                (node_ref, raw.into())
            }
        }
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        self.reconcile(root, parent_scope, parent, next_sibling, bundle)
    }

    fn reconcile(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        match self {
            VNode::VTag(vtag) => {
                vtag.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
            VNode::VText(vtext) => {
                vtext.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
            VNode::VComp(vcomp) => {
                vcomp.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
            VNode::VList(vlist) => {
                vlist.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
            VNode::VRef(node) => {
                let _existing = match bundle {
                    BNode::Ref(ref n) if &node == n => n,
                    _ => {
                        return VNode::VRef(node).replace(
                            root,
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
                vportal.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
            VNode::VSuspense(vsuspsense) => {
                vsuspsense.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
            VNode::VRaw(vraw) => {
                vraw.reconcile_node(root, parent_scope, parent, next_sibling, bundle)
            }
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
        ) -> (NodeRef, Self::Bundle) {
            match self {
                VNode::VTag(vtag) => {
                    let (node_ref, tag) = vtag.hydrate(root, parent_scope, parent, fragment);
                    (node_ref, tag.into())
                }
                VNode::VText(vtext) => {
                    let (node_ref, text) = vtext.hydrate(root, parent_scope, parent, fragment);
                    (node_ref, text.into())
                }
                VNode::VComp(vcomp) => {
                    let (node_ref, comp) = vcomp.hydrate(root, parent_scope, parent, fragment);
                    (node_ref, comp.into())
                }
                VNode::VList(vlist) => {
                    let (node_ref, list) = vlist.hydrate(root, parent_scope, parent, fragment);
                    (node_ref, list.into())
                }
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
                VNode::VSuspense(vsuspense) => {
                    let (node_ref, suspense) =
                        vsuspense.hydrate(root, parent_scope, parent, fragment);
                    (node_ref, suspense.into())
                }
                VNode::VRaw(_) => {
                    panic!("VRaw is not hydratable (raw HTML string cannot be hydrated)")
                }
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
