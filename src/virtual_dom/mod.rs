//! This module contains the implementation of reactive virtual dom concept.

pub mod vcomp;
pub mod vlist;
pub mod vnode;
pub mod vtag;
pub mod vtext;

use indexmap::set::IndexSet;
use std::collections::HashMap;
use std::fmt;
use stdweb::web::{Element, EventListenerHandle, Node};

pub use self::vcomp::VComp;
pub use self::vlist::VList;
pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;
use crate::html::{Component, Scope};

/// `Listener` trait is an universal implementation of an event listener
/// which helps to bind Rust-listener to JS-listener (DOM).
pub trait Listener<COMP: Component> {
    /// Returns standard name of DOM's event.
    fn kind(&self) -> &'static str;
    /// Attaches listener to the element and uses scope instance to send
    /// prepared event back to the yew main loop.
    fn attach(&mut self, element: &Element, scope: Scope<COMP>) -> EventListenerHandle;
}

impl<COMP: Component> fmt::Debug for dyn Listener<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

/// A list of event listeners.
type Listeners<COMP> = Vec<Box<dyn Listener<COMP>>>;

/// A map of attributes.
type Attributes = HashMap<String, String>;

/// A set of classes.
#[derive(Debug)]
pub struct Classes {
    set: IndexSet<String>,
}

impl Classes {
    /// Creates empty set of classes.
    pub fn new() -> Self {
        Self {
            set: IndexSet::new(),
        }
    }

    /// Adds a class to a set.
    pub fn push(&mut self, class: &str) {
        self.set.insert(class.into());
    }

    /// Check the set contains a class.
    pub fn contains(&self, class: &str) -> bool {
        self.set.contains(class)
    }
}

impl ToString for Classes {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        for class in &self.set {
            buf.push_str(class);
            buf.push(' ');
        }
        buf.pop();
        buf
    }
}

impl From<&str> for Classes {
    fn from(t: &str) -> Self {
        let set = t.split_whitespace().map(String::from).collect();
        Self { set }
    }
}

impl From<String> for Classes {
    fn from(t: String) -> Self {
        let set = t.split_whitespace().map(String::from).collect();
        Self { set }
    }
}

impl<T: AsRef<str>> From<Vec<T>> for Classes {
    fn from(t: Vec<T>) -> Self {
        let set = t.iter().map(|x| x.as_ref().to_string()).collect();
        Self { set }
    }
}

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
    fn detach(&mut self, parent: &Element) -> Option<Node>;

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
        parent: &Element,
        precursor: Option<&Node>,
        ancestor: Option<VNode<Self::Component>>,
        scope: &Scope<Self::Component>,
    ) -> Option<Node>;
}
