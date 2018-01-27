//! This module contains the implementation of reactive virtual dom concept.

pub mod vnode;
pub mod vtag;
pub mod vtext;
pub mod vcomp;
pub mod vlist;

use std::fmt;
use std::collections::{HashMap, HashSet};
use stdweb::web::{Node, Element, EventListenerHandle};

pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;
pub use self::vcomp::VComp;
pub use self::vlist::VList;
use html::{ScopeSender, ScopeEnv, Component};

/// `Listener` trait is an universal implementation of an event listener
/// which helps to bind Rust-listener to JS-listener (DOM).
pub trait Listener<CTX, COMP: Component<CTX>> {
    /// Returns standard name of DOM's event.
    fn kind(&self) -> &'static str;
    /// Attaches listener to the element and uses sender instance to send
    /// prepaired event back to the yew main loop.
    fn attach(&mut self, element: &Element, sender: ScopeSender<CTX, COMP>) -> EventListenerHandle;
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for Listener<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

/// A list of event listeners.
type Listeners<CTX, COMP> = Vec<Box<Listener<CTX, COMP>>>;

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
enum Reform<T> {
    Keep(T),
    Append,
    Replace(Node),
}

// TODO What about to implement `VDiff` for `Element`?
// In makes possible to include ANY element into the tree.
// `Ace` editor embedding for example?

/// This trait provides features to update a tree by other tree comparsion.
pub trait VDiff {
    /// The context where this instance live.
    type Context;
    /// The component which this instance put into.
    type Component: Component<Self::Context>;

    /// Get binded node.
    fn get_node(&self) -> Option<Node>;
    /// Remove itself from parent.
    fn remove(self, parent: &Node);
    /// Scoped diff apply to other tree.
    fn apply(&mut self,
             parent: &Node,
             precursor: Option<&Node>,
             opposite: Option<VNode<Self::Context, Self::Component>>,
             scope: ScopeEnv<Self::Context, Self::Component>) -> Option<Node>;
}
