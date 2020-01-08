//! This module contains the implementation of reactive virtual dom concept.

pub mod vcomp;
pub mod vlist;
pub mod vnode;
pub mod vtag;
pub mod vtext;

#[cfg(feature = "std_web")]
use crate::html::EventListener;
#[cfg(feature = "web_sys")]
use gloo::events::EventListener;
use indexmap::set::IndexSet;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
#[cfg(feature = "std_web")]
use stdweb::web::{Element, Node};
#[cfg(feature = "web_sys")]
use web_sys::{Element, Node};

pub use self::vcomp::{VChild, VComp};
pub use self::vlist::VList;
pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;

/// `Listener` trait is an universal implementation of an event listener
/// which helps to bind Rust-listener to JS-listener (DOM).
pub trait Listener {
    /// Returns standard name of DOM's event.
    fn kind(&self) -> &'static str;
    /// Attaches a listener to the element.
    fn attach(&self, element: &Element) -> EventListener;
}

impl fmt::Debug for dyn Listener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

/// A list of event listeners.
type Listeners = Vec<Rc<dyn Listener>>;

/// A map of attributes.
type Attributes = HashMap<String, String>;

/// A set of classes.
#[derive(Debug, Clone, Default, PartialEq)]
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
    ///
    /// Prevents duplication of class names.
    pub fn push(&mut self, class: &str) {
        if !class.is_empty() {
            self.set.insert(class.into());
        }
    }

    /// Check the set contains a class.
    pub fn contains(&self, class: &str) -> bool {
        self.set.contains(class)
    }

    /// Check the set is empty.
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Adds other classes to this set of classes; returning itself.
    ///
    /// Takes the logical union of both `Classes`.
    pub fn extend<T: Into<Classes>>(mut self, other: T) -> Self {
        self.set.extend(other.into().set.into_iter());
        self
    }
}

impl ToString for Classes {
    fn to_string(&self) -> String {
        self.set
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

impl From<&str> for Classes {
    fn from(t: &str) -> Self {
        let set = t
            .split_whitespace()
            .map(String::from)
            .filter(|c| !c.is_empty())
            .collect();
        Self { set }
    }
}

impl From<String> for Classes {
    fn from(t: String) -> Self {
        let set = t
            .split_whitespace()
            .map(String::from)
            .filter(|c| !c.is_empty())
            .collect();
        Self { set }
    }
}

impl From<&String> for Classes {
    fn from(t: &String) -> Self {
        let set = t
            .split_whitespace()
            .map(String::from)
            .filter(|c| !c.is_empty())
            .collect();
        Self { set }
    }
}

impl<T: AsRef<str>> From<Vec<T>> for Classes {
    fn from(t: Vec<T>) -> Self {
        let set = t
            .iter()
            .map(|x| x.as_ref().to_string())
            .filter(|c| !c.is_empty())
            .collect();
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
    /// If it does not exist, a `previous_sibling` must be
    /// specified (see `VDiff::apply()`).
    Before(Option<Node>),
}

// TODO What about to implement `VDiff` for `Element`?
// In makes possible to include ANY element into the tree.
// `Ace` editor embedding for example?

/// This trait provides features to update a tree by calculating a difference against another tree.
pub trait VDiff {
    /// Remove itself from parent and return the next sibling.
    fn detach(&mut self, parent: &Element) -> Option<Node>;

    /// Scoped diff apply to other tree.
    ///
    /// Virtual rendering for the node. It uses parent node and existing children (virtual and DOM)
    /// to check the difference and apply patches to the actual DOM representation.
    ///
    /// Parameters:
    /// - `parent`: the parent node in the DOM.
    /// - `previous_sibling`: the "previous node" in a list of nodes, used to efficiently
    ///   find where to put the node.
    /// - `ancestor`: the node that this node will be replacing in the DOM.
    ///   This method will _always_ remove the `ancestor` from the `parent`.
    /// - `parent_scope`: the parent `Scope` used for passing messages to the parent `Component`.
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
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node>;
}

/// Transform properties to the expected type.
pub trait Transformer<FROM, TO> {
    /// Transforms one type to another.
    fn transform(from: FROM) -> TO;
}
