//! Realizing a virtual dom on the actual DOM
//!
//! A bundle, borrowed from the mathematical meaning, is any structure over some base space.
//! In our case, the base space is the virtual dom we're trying to render.
//! In order to efficiently implement updates, and diffing, additional information has to be
//! kept around. This information is carried in the bundle.

mod attributes;
mod bcomp;
mod blist;
mod bnode;
mod bportal;
mod bsuspense;
mod btag;
mod listeners;

#[cfg(debug_assertions)]
pub(crate) use self::bcomp::log_event;

pub use self::bcomp::BComp;
pub use self::blist::BList;
pub use self::bnode::BNode;
pub use self::bportal::BPortal;
pub use self::bsuspense::BSuspense;
pub use self::btag::BTag;

pub(crate) use self::attributes::{Apply, InputFields, Value};
pub(crate) use self::bcomp::{Mountable, PropsWrapper};
#[doc(hidden)]
pub use self::listeners::set_event_bubbling;

use crate::{html::AnyScope, NodeRef};
use web_sys::{Element, Node};

pub(crate) trait DomBundle {
    /// Remove self from parent.
    fn detach(self, parent: &Element);

    /// Move elements from one parent to another parent.
    /// This is for example used by `VSuspense` to preserve component state without detaching
    /// (which destroys component state).
    fn shift(&self, next_parent: &Element, next_sibling: NodeRef);
}

// TODO(#938): What about implementing `VDiff` for `Element`?
// It would make it possible to include ANY element into the tree.
// `Ace` editor embedding for example?

/// This trait provides features to update a tree by calculating a difference against another tree.
pub(crate) trait VDiff {
    type Bundle: DomBundle;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle);

    /// Scoped diff apply to other tree.
    ///
    /// Virtual rendering for the node. It uses parent node and existing
    /// children (virtual and DOM) to check the difference and apply patches to
    /// the actual DOM representation.
    ///
    /// Parameters:
    /// - `parent_scope`: the parent `Scope` used for passing messages to the
    ///   parent `Component`.
    /// - `parent`: the parent node in the DOM.
    /// - `next_sibling`: the next sibling, used to efficiently find where to
    ///   put the node.
    /// - `ancestor`: the node that this node will be replacing in the DOM. This
    ///   method will _always_ remove the `ancestor` from the `parent`.
    ///
    /// Returns a reference to the newly inserted element.
    ///
    /// ### Internal Behavior Notice:
    ///
    /// Note that these modify the DOM by modifying the reference that _already_
    /// exists on the `ancestor`. If `self.reference` exists (which it
    /// _shouldn't_) this method will panic.
    ///
    /// The exception to this is obviously `VRef` which simply uses the inner
    /// `Node` directly (always removes the `Node` that exists).
    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
    ) -> NodeRef;
}

pub(crate) fn insert_node(node: &Node, parent: &Element, next_sibling: Option<&Node>) {
    match next_sibling {
        Some(next_sibling) => parent
            .insert_before(node, Some(next_sibling))
            .expect("failed to insert tag before next sibling"),
        None => parent.append_child(node).expect("failed to append child"),
    };
}

/// Log an operation during tests for debugging purposes
/// Set RUSTFLAGS="--cfg verbose_tests" environment variable to activate.
#[cfg(all(test, feature = "wasm_test", verbose_tests))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        ::wasm_bindgen_test::console_log!(concat!("\t  ", $fmt), $($arg),*);
    };
}
#[cfg(not(all(test, feature = "wasm_test", verbose_tests)))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        let _ = std::format_args!(concat!("\t  ", $fmt), $($arg),*);
    };
}
pub(self) use test_log;
