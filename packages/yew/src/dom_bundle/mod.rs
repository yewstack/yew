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
mod traits;
mod utils;

use gloo::utils::document;
use web_sys::{Element, Node};

use crate::html::AnyScope;
use crate::html::NodeRef;
use crate::virtual_dom::VNode;

use bcomp::BComp;
use blist::BList;
use bnode::BNode;
use bportal::BPortal;
use bsuspense::BSuspense;
use btag::BTag;
use btext::BText;
use traits::{DomBundle, Reconcilable};
use utils::{insert_node, test_log};

#[doc(hidden)] // Publically exported from crate::events
pub use self::btag::set_event_bubbling;

/// Bundle of a Component.
#[derive(Debug)]
pub(crate) struct Bundle(BNode);

impl Bundle {
    /// Creates a new root.
    pub fn new(parent: &Element, next_sibling: &NodeRef, node_ref: &NodeRef) -> Self {
        let placeholder: Node = document().create_text_node("").into();
        insert_node(&placeholder, parent, next_sibling.get().as_ref());
        node_ref.set(Some(placeholder.clone()));
        Self(BNode::Ref(placeholder))
    }

    /// Shifts the bundle into a different position.
    pub fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        self.0.shift(next_parent, next_sibling);
    }

    /// Applies a virtual dom layout to current root.
    pub fn reconcile(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        next_node: VNode,
    ) -> NodeRef {
        next_node.reconcile_node(parent_scope, parent, next_sibling, &mut self.0)
    }

    /// Detaches current root.
    pub fn detach(self, parent: &Element, parent_to_detach: bool) {
        self.0.detach(parent, parent_to_detach);
    }
}
