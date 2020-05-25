//! This module contains the implementation of reactive virtual dom concept.

#[doc(hidden)]
pub mod key;
#[doc(hidden)]
pub mod vcomp;
#[doc(hidden)]
pub mod vlist;
#[doc(hidden)]
pub mod vnode;
#[doc(hidden)]
pub mod vtag;
#[doc(hidden)]
pub mod vtext;

use crate::html::AnyScope;
use cfg_if::cfg_if;
use indexmap::set::IndexSet;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use crate::html::EventListener;
        use stdweb::web::{Element, INode, Node};
    } else if #[cfg(feature = "web_sys")] {
        use gloo::events::EventListener;
        use web_sys::{Element, Node};
    }
}

#[doc(inline)]
pub use self::key::Key;
#[doc(inline)]
pub use self::vcomp::{VChild, VComp};
#[doc(inline)]
pub use self::vlist::VList;
#[doc(inline)]
pub use self::vnode::VNode;
#[doc(inline)]
pub use self::vtag::VTag;
#[doc(inline)]
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
#[derive(Debug, Clone, Default)]
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
        let classes_to_add: Classes = class.into();
        self.set.extend(classes_to_add.set);
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
        Classes::from(t.as_str())
    }
}

impl From<&String> for Classes {
    fn from(t: &String) -> Self {
        Classes::from(t.as_str())
    }
}

impl<T: AsRef<str>> From<Option<T>> for Classes {
    fn from(t: Option<T>) -> Self {
        t.as_ref()
            .map(|s| <Classes as From<&str>>::from(s.as_ref()))
            .unwrap_or_default()
    }
}

impl<T: AsRef<str>> From<&Option<T>> for Classes {
    fn from(t: &Option<T>) -> Self {
        t.as_ref()
            .map(|s| <Classes as From<&str>>::from(s.as_ref()))
            .unwrap_or_default()
    }
}

impl<T: AsRef<str>> From<Vec<T>> for Classes {
    fn from(t: Vec<T>) -> Self {
        let set = t
            .iter()
            .map(|x| x.as_ref())
            .flat_map(|s| s.split_whitespace())
            .map(String::from)
            .filter(|c| !c.is_empty())
            .collect();
        Self { set }
    }
}

impl PartialEq for Classes {
    fn eq(&self, other: &Self) -> bool {
        self.set.len() == other.set.len() && self.set.iter().eq(other.set.iter())
    }
}

/// Patch for DOM node modification.
#[derive(Debug, PartialEq)]
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
    /// The optional `Node` is used to insert the new node in the correct slot
    /// of the parent.
    ///
    /// If it does not exist, the `node_position` will be used (see
    /// `VDiff::apply()`).
    Replace(VDiffNodePosition),
}

#[derive(Debug)]
pub(crate) enum VDiffNodePosition {
    FirstChild,
    Before(Node),
    After(Node),
    LastChild,
}

// TODO(#938): What about implementing `VDiff` for `Element`?
// It would make it possible to include ANY element into the tree.
// `Ace` editor embedding for example?

/// This trait provides features to update a tree by calculating a difference against another tree.
pub(crate) trait VDiff {
    /// Remove self from parent and return the next sibling.
    fn detach(&mut self, parent: &Element) -> VDiffNodePosition;

    /// Scoped diff apply to other tree.
    ///
    /// Virtual rendering for the node. It uses parent node and existing
    /// children (virtual and DOM) to check the difference and apply patches to
    /// the actual DOM representation.
    ///
    /// TODO: Explain that vnodes must be at their position before apply is called.
    ///
    /// Parameters:
    /// - `parent_scope`: the parent `Scope` used for passing messages to the
    ///   parent `Component`.
    /// - `parent`: the parent node in the DOM.
    /// - `node_position`: the position relative to `parent` used to efficiently
    ///   find where to put the node.
    /// - `ancestor`: the node that this node will be replacing in the DOM. This
    ///   method will _always_ remove the `ancestor` from the `parent`.
    ///
    /// Returns the newly inserted element, if there is one (empty VList don't have one).
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
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        node_position: VDiffNodePosition, // TODO: merge `node_position` and `ancestor`
        ancestor: Option<VNode>,
    ) -> Option<Node>;
}

#[cfg(feature = "web_sys")]
fn insert_node(node: &Node, parent: &Element, node_position: &VDiffNodePosition) -> Node {
    match node_position {
        VDiffNodePosition::FirstChild => parent
            .insert_before(&node, parent.first_child().as_ref())
            .expect("failed to insert tag as first child"),
        VDiffNodePosition::Before(next_sibling) => parent
            .insert_before(&node, Some(next_sibling))
            .expect("failed to insert tag before next sibling"),
        VDiffNodePosition::After(previous_sibling) => parent
            .insert_before(&node, previous_sibling.next_sibling().as_ref())
            .expect("failed to insert tag after previous sibling"),
        VDiffNodePosition::LastChild => parent.append_child(node).expect("failed to append tag"),
    }
}

#[cfg(feature = "std_web")]
fn insert_node(node: &impl INode, parent: &impl INode, node_position: &VDiffNodePosition) -> Node {
    fn insert_before(node: &impl INode, parent: &impl INode, reference: Option<&Node>) {
        if let Some(reference) = reference {
            parent
                .insert_before(node, reference)
                .expect("failed to insert tag before next sibling");
        } else {
            parent.append_child(node);
        }
    }

    match node_position {
        VDiffNodePosition::FirstChild => insert_before(node, parent, parent.first_child().as_ref()),
        VDiffNodePosition::Before(next_sibling) => insert_before(node, parent, Some(next_sibling)),
        VDiffNodePosition::After(previous_sibling) => {
            insert_before(node, parent, previous_sibling.next_sibling().as_ref())
        }
        VDiffNodePosition::LastChild => parent.append_child(node),
    }
    node.as_node().clone()
}

/// Transform properties to the expected type.
pub trait Transformer<FROM, TO> {
    /// Transforms one type to another.
    fn transform(from: FROM) -> TO;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_initially_empty() {
        let subject = Classes::new();
        assert!(subject.is_empty());
    }

    #[test]
    fn it_pushes_value() {
        let mut subject = Classes::new();
        subject.push("foo");
        assert!(!subject.is_empty());
        assert!(subject.contains("foo"));
    }

    #[test]
    fn it_adds_values_via_extend() {
        let mut other = Classes::new();
        other.push("bar");
        let subject = Classes::new().extend(other);
        assert!(subject.contains("bar"));
    }

    #[test]
    fn it_contains_both_values() {
        let mut other = Classes::new();
        other.push("bar");
        let mut subject = Classes::new().extend(other);
        subject.push("foo");
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }

    #[test]
    fn it_splits_class_with_spaces() {
        let mut subject = Classes::new();
        subject.push("foo bar");
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }
}
