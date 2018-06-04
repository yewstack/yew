//! This module contains the implementation of reactive virtual dom concept.

pub mod vcomp;
pub mod vlist;
pub mod vnode;
pub mod vtag;
pub mod vtext;

use std::collections::{HashMap, HashSet};
use std::fmt;
use stdweb::web::{Element, EventListenerHandle, Node};

pub use self::vcomp::VComp;
pub use self::vlist::VList;
pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;
use html::{Component, Scope};

/// `Listener` trait is an universal implementation of an event listener
/// which helps to bind Rust-listener to JS-listener (DOM).
pub trait Listener<COMP: Component> {
    /// Returns standard name of DOM's event.
    fn kind(&self) -> &'static str;
    /// Attaches listener to the element and uses scope instance to send
    /// prepaired event back to the yew main loop.
    fn attach(&mut self, element: &Element, scope: Scope<COMP>) -> EventListenerHandle;
}

impl<COMP: Component> fmt::Debug for Listener<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

/// A list of event listeners.
type Listeners<COMP> = Vec<Box<Listener<COMP>>>;

/// A map of attributes.
type Attributes = HashMap<String, String>;

/// A set of classes.
type Classes = HashSet<String>;

/// Patch for DOM node modification.
enum Patch<ID, T> {
    Add(ID, T),
    Replace(ID, T),
    Remove(ID),
}

/// Reform of a node.
enum Reform {
    /// Don't create a NEW reference (js Node).
    ///
    /// The reference _may still be mutated_.
    Keep,

    /// Create a new reference (js Node).
    ///
    /// The optional `Node` is used to insert the
    /// new node in the correct slot of the parent.
    ///
    /// If it does not exist, a `precursor` must be
    /// speccified (see `VDiff::apply()`).
    Before(Option<Node>),
}

// TODO What about to implement `VDiff` for `Element`?
// In makes possible to include ANY element into the tree.
// `Ace` editor embedding for example?

/// This trait provides features to update a tree by other tree comparsion.
pub trait VDiff {
    /// The component which this instance put into.
    type Component: Component;

    /// Remove itself from parent and return the next sibling.
    fn detach(&mut self, parent: &Node) -> Option<Node>;

    /// Scoped diff apply to other tree.
    ///
    /// Virtual rendering for the node. It uses parent node and existing children (virtual and DOM)
    /// to check the difference and apply patches to the actual DOM represenatation.
    ///
    /// Parameters:
    /// - `parent`: the parent node in the DOM.
    /// - `precursor`: the "previous node" in a list of nodes, used to efficiently
    ///   find where to put the node.
    /// - `ancestor`: the node that this node will be replacing in the DOM.
    ///   This method will _always_ remove the `ancestor` from the `parent`.
    /// - `env`: the `Env`.
    ///
    /// ### Internal Behavior Notice:
    ///
    /// Note that these modify the DOM by modifying the reference that _already_ exists
    /// on the `ancestor`. If `self.reference` exists (which it _shouldn't_) this method
    /// will panic.
    ///
    /// The exception to this is obviously `VRef` which simply uses the inner `Node` directly
    /// (always removes the `Node` that exists).
    fn apply(
        &mut self,
        parent: &Node,
        precursor: Option<&Node>,
        ancestor: Option<VNode<Self::Component>>,
        scope: &Scope<Self::Component>,
    ) -> Option<Node>;
}
