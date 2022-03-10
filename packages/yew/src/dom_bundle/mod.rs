//! Realizing a virtual dom on the actual DOM
//!
//! A bundle, borrowed from the mathematical meaning, is any structure over some base space.
//! In our case, the base space is the virtual dom we're trying to render.
//! In order to efficiently implement updates, and diffing, additional information has to be
//! kept around. This information is carried in the bundle.

mod app_handle;
mod bcomp;
mod blist;
mod bnode;
mod bportal;
mod bsuspense;
mod btag;
mod btext;
mod tree_root;

#[cfg(test)]
mod tests;

use self::bcomp::BComp;
use self::blist::BList;
use self::bnode::BNode;
use self::bportal::BPortal;
use self::bsuspense::BSuspense;
use self::btag::{BTag, EventDescriptor, Registry};
use self::btext::BText;

pub(crate) use self::bcomp::{ComponentRenderState, Mountable, PropsWrapper, Scoped};
pub(crate) use self::tree_root::BundleRoot;

#[doc(hidden)] // Publicly exported from crate::app_handle
pub use self::app_handle::AppHandle;
#[doc(hidden)] // Publicly exported from crate::events
pub use self::btag::set_event_bubbling;
#[cfg(test)]
#[doc(hidden)] // Publicly exported from crate::tests
pub use self::tests::layout_tests;

use crate::html::AnyScope;
use crate::NodeRef;
use web_sys::{Element, Node};

trait DomBundle {
    /// Remove self from parent.
    ///
    /// Parent to detach is `true` if the parent element will also be detached.
    fn detach(self, root: &BundleRoot, parent: &Element, parent_to_detach: bool);

    /// Move elements from one parent to another parent.
    /// This is for example used by `VSuspense` to preserve component state without detaching
    /// (which destroys component state).
    fn shift(&self, next_root: &BundleRoot, next_parent: &Element, next_sibling: NodeRef);
}

/// This trait provides features to update a tree by calculating a difference against another tree.
trait Reconcilable {
    type Bundle: DomBundle;

    /// Attach a virtual node to the DOM tree.
    ///
    /// Parameters:
    /// - `root`: bundle of the subtree root
    /// - `parent_scope`: the parent `Scope` used for passing messages to the
    ///   parent `Component`.
    /// - `parent`: the parent node in the DOM.
    /// - `next_sibling`: to find where to put the node.
    ///
    /// Returns a reference to the newly inserted element.
    fn attach(
        self,
        root: &BundleRoot,
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
    /// - `bundle`: the node that this node will be replacing in the DOM. This
    ///   method will remove the `bundle` from the `parent` if it is of the wrong
    ///   kind, and otherwise reuse it.
    ///
    /// Returns a reference to the newly inserted element.
    fn reconcile_node(
        self,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef;

    fn reconcile(
        self,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut Self::Bundle,
    ) -> NodeRef;

    /// Replace an existing bundle by attaching self and detaching the existing one
    fn replace(
        self,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef
    where
        Self: Sized,
        Self::Bundle: Into<BNode>,
    {
        let (self_ref, self_) = self.attach(root, parent_scope, parent, next_sibling);
        let ancestor = std::mem::replace(bundle, self_.into());
        ancestor.detach(root, parent, false);
        self_ref
    }
}

/// Insert a concrete [Node] into the DOM
fn insert_node(node: &Node, parent: &Element, next_sibling: Option<&Node>) {
    match next_sibling {
        Some(next_sibling) => parent
            .insert_before(node, Some(next_sibling))
            .expect("failed to insert tag before next sibling"),
        None => parent.append_child(node).expect("failed to append child"),
    };
}

#[cfg(all(test, feature = "wasm_test", verbose_tests))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        ::wasm_bindgen_test::console_log!(concat!("\t  ", $fmt), $($arg),*);
    };
}
#[cfg(not(all(test, feature = "wasm_test", verbose_tests)))]
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        // Only type-check the format expression, do not run any side effects
        let _ = || { std::format_args!(concat!("\t  ", $fmt), $($arg),*); };
    };
}
/// Log an operation during tests for debugging purposes
/// Set RUSTFLAGS="--cfg verbose_tests" environment variable to activate.
pub(self) use test_log;
