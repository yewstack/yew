//! This module contains the implementation of reactive virtual dom concept.

pub mod vnode;
pub mod vtag;
pub mod vtext;
pub mod vcomp;

use std::fmt;
use std::collections::{HashMap, HashSet};
use stdweb::web::{Element, EventListenerHandle};

pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;
pub use self::vcomp::VComp;
use html::AppSender;

/// `Listener` trait is an universal implementation of an event listener
/// which helps to bind Rust-listener to JS-listener (DOM).
pub trait Listener<MSG, CTX> {
    /// Returns standard name of DOM's event.
    fn kind(&self) -> &'static str;
    /// Attaches listener to the element and uses sender instance to send
    /// prepaired event back to the yew main loop.
    fn attach(&mut self, element: &Element, sender: AppSender<MSG, CTX>) -> EventListenerHandle;
}

impl<MSG, CTX> fmt::Debug for Listener<MSG, CTX> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

/// A list of event listeners.
type Listeners<MSG, CTX> = Vec<Box<Listener<MSG, CTX>>>;

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

