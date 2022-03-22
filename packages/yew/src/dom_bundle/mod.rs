//! Realizing a virtual dom on the actual DOM
//!
//! A bundle, borrowed from the mathematical meaning, is any structure over some base space.
//! In our case, the base space is the virtual dom we're trying to render.
//! In order to efficiently implement updates, and diffing, additional information has to be
//! kept around. This information is carried in the bundle.

mod bcomp;
mod blist;
mod bnode;
mod bportal;
mod bsuspense;
mod btag;
mod btext;
mod subtree_root;

mod traits;
mod utils;

use web_sys::Element;

use crate::html::AnyScope;
use crate::html::NodeRef;
use crate::virtual_dom::VNode;

use bcomp::BComp;
use blist::BList;
use bnode::BNode;
use bportal::BPortal;
use bsuspense::BSuspense;
use btag::{BTag, Registry};
use btext::BText;
use subtree_root::EventDescriptor;
use traits::{Reconcilable, ReconcileTarget};
use utils::{insert_node, test_log};

#[doc(hidden)] // Publically exported from crate::events
pub use subtree_root::set_event_bubbling;

pub(crate) use subtree_root::BSubtree;

/// A Bundle.
///
/// Each component holds a bundle that represents a realised layout, designated by a [VNode].
///
/// This is not to be confused with [BComp], which represents a component in the position of a
/// bundle layout.
#[derive(Debug)]
pub(crate) struct Bundle(BNode);

impl Bundle {
    /// Creates a new bundle.
    pub fn new() -> Self {
        Self(BNode::List(BList::new()))
    }

    /// Shifts the bundle into a different position.
    pub fn shift(&self, next_root: &BSubtree, next_parent: &Element, next_sibling: NodeRef) {
        self.0.shift(next_root, next_parent, next_sibling);
    }

    /// Applies a virtual dom layout to current bundle.
    pub fn reconcile(
        &mut self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        next_node: VNode,
    ) -> NodeRef {
        next_node.reconcile_node(root, parent_scope, parent, next_sibling, &mut self.0)
    }

    /// Detaches current bundle.
    pub fn detach(self, root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        self.0.detach(root, parent, parent_to_detach);
    }
}
