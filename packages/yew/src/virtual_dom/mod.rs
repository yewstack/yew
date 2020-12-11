//! This module contains Yew's implementation of a reactive virtual DOM.
#[doc(hidden)]
pub mod events;
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

mod transform;
pub use transform::Transformer;

// pub use classes::Classes;

use crate::html::{AnyScope, NodeRef};
use indexmap::{IndexMap, IndexSet};
use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    fmt,
    hint::unreachable_unchecked,
    iter::{self, FromIterator},
    mem,
    rc::Rc,
};

#[doc(inline)]
pub use self::{
    key::Key,
    vcomp::{VChild, VComp},
    vlist::VList,
    vnode::VNode,
    vtag::VTag,
    vtext::VText,
};

use attributes::Attributes;
use positional_attributes::PositionalAttr;
use vdiff::{Patch, VDiff};

#[derive(Debug)]
pub struct Element {}
pub struct EventListener {}

mod listener {
    use super::*;
    /// The `Listener` trait is an universal implementation of an event listener
    /// which is used to bind Rust-listener to JS-listener (DOM).
    pub trait Listener {
        /// Returns the name of the event
        fn kind(&self) -> &'static str;
        /// Attaches a listener to the element.
        fn attach(&self) -> EventListener;
        // fn attach(&self, element: &Element) -> EventListener;
    }

    impl fmt::Debug for dyn Listener {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Listener {{ kind: {} }}", self.kind())
        }
    }
}

// /// A list of event listeners.
// type Listeners = Vec<Rc<dyn Listener>>;

mod positional_attributes {
    use super::*;
    /// Key-value tuple which makes up an item of the [`Attributes::Vec`] variant.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct PositionalAttr(pub &'static str, pub Option<Cow<'static, str>>);
    impl PositionalAttr {
        /// Create a positional attribute
        pub fn new(key: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
            Self(key, Some(value.into()))
        }
        /// Create a placeholder for removed attributes
        pub fn new_placeholder(key: &'static str) -> Self {
            Self(key, None)
        }

        fn transpose(self) -> Option<(&'static str, Cow<'static, str>)> {
            let Self(key, value) = self;
            value.map(|v| (key, v))
        }

        fn transposed<'a>(&'a self) -> Option<(&'static str, &'a Cow<'static, str>)> {
            let Self(key, value) = self;
            value.as_ref().map(|v| (*key, v))
        }
    }
}
mod attributes {
    use super::*;

    /// A collection of attributes for an element
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum Attributes {
        /// A vector is ideal because most of the time the list will neither change
        /// length nor key order.
        Vec(Vec<PositionalAttr>),

        /// IndexMap is used to provide runtime attribute deduplication in cases where the html! macro
        /// was not used to guarantee it.
        IndexMap(IndexMap<&'static str, Cow<'static, str>>),
    }
    impl Attributes {
        /// Construct a default Attributes instance
        pub fn new() -> Self {
            Default::default()
        }

        /// Return iterator over attribute key-value pairs
        pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static str, &'a str)> + 'a> {
            match self {
                Self::Vec(v) => Box::new(
                    v.iter()
                        .filter_map(PositionalAttr::transposed)
                        .map(|(k, v)| (k, v.as_ref())),
                ),
                Self::IndexMap(m) => Box::new(m.iter().map(|(k, v)| (*k, v.as_ref()))),
            }
        }

        /// Get a mutable reference to the underlying `IndexMap`.
        /// If the attributes are stored in the `Vec` variant, it will be converted.
        pub fn get_mut_index_map(&mut self) -> &mut IndexMap<&'static str, Cow<'static, str>> {
            match self {
                Self::IndexMap(m) => m,
                Self::Vec(v) => {
                    *self = Self::IndexMap(
                        mem::take(v)
                            .into_iter()
                            .filter_map(PositionalAttr::transpose)
                            .collect(),
                    );
                    match self {
                        Self::IndexMap(m) => m,
                        // SAFETY: unreachable because we set the value to the `IndexMap` variant above.
                        _ => unsafe { unreachable_unchecked() },
                    }
                }
            }
        }

        fn diff_vec<'a>(
            new: &'a [PositionalAttr],
            old: &[PositionalAttr],
        ) -> Vec<Patch<&'static str, &'a str>> {
            let mut out = Vec::new();
            let mut new_iter = new.iter();
            let mut old_iter = old.iter();

            loop {
                match (new_iter.next(), old_iter.next()) {
                    (
                        Some(PositionalAttr(key, new_value)),
                        Some(PositionalAttr(old_key, old_value)),
                    ) if key == old_key => match (new_value, old_value) {
                        (Some(new), Some(old)) => {
                            if new != old {
                                out.push(Patch::Replace(*key, new.as_ref()));
                            }
                        }
                        (Some(value), None) => out.push(Patch::Add(*key, value.as_ref())),
                        (None, Some(_)) => out.push(Patch::Remove(*key)),
                        (None, None) => {}
                    },
                    // keys don't match, we can no longer compare linearly from here on out
                    (Some(new_attr), Some(old_attr)) => {
                        // assume that every attribute is new.
                        let mut added = iter::once(new_attr)
                            .chain(new_iter)
                            .filter_map(PositionalAttr::transposed)
                            .map(|(key, value)| (key, value.as_ref()))
                            .collect::<HashMap<_, _>>();

                        // now filter out all the attributes that aren't new
                        for (key, old_value) in iter::once(old_attr)
                            .chain(old_iter)
                            .filter_map(PositionalAttr::transposed)
                        {
                            if let Some(new_value) = added.remove(key) {
                                // attribute still exists but changed value
                                if new_value != old_value.as_ref() {
                                    out.push(Patch::Replace(key, new_value));
                                }
                            } else {
                                // attribute no longer exists
                                out.push(Patch::Remove(key));
                            }
                        }

                        // finally, we're left with the attributes that are actually new.
                        out.extend(added.into_iter().map(|(k, v)| Patch::Add(k, v)));
                        break;
                    }
                    // added attributes
                    (Some(attr), None) => {
                        for PositionalAttr(key, value) in iter::once(attr).chain(new_iter) {
                            // only add value if it has a value
                            if let Some(value) = value {
                                out.push(Patch::Add(*key, value));
                            }
                        }
                        break;
                    }
                    // removed attributes
                    (None, Some(attr)) => {
                        for PositionalAttr(key, value) in iter::once(attr).chain(old_iter) {
                            // only remove the attribute if it had a value before
                            if value.is_some() {
                                out.push(Patch::Remove(*key));
                            }
                        }
                        break;
                    }
                    (None, None) => break,
                }
            }

            out
        }

        fn diff_index_map<'a, A, B>(
            // this makes it possible to diff `&'a IndexMap<_, A>` and `IndexMap<_, &'a A>`.
            mut new_iter: impl Iterator<Item = (&'static str, &'a str)>,
            new: &IndexMap<&'static str, A>,
            old: &IndexMap<&'static str, B>,
        ) -> Vec<Patch<&'static str, &'a str>>
        where
            A: AsRef<str>,
            B: AsRef<str>,
        {
            let mut out = Vec::new();
            let mut old_iter = old.iter();
            loop {
                match (new_iter.next(), old_iter.next()) {
                    (Some((new_key, new_value)), Some((old_key, old_value))) => {
                        if new_key != *old_key {
                            break;
                        }
                        if new_value != old_value.as_ref() {
                            out.push(Patch::Replace(new_key, new_value));
                        }
                    }
                    // new attributes
                    (Some(attr), None) => {
                        for (key, value) in iter::once(attr).chain(new_iter) {
                            match old.get(key) {
                                Some(old_value) => {
                                    if value != old_value.as_ref() {
                                        out.push(Patch::Replace(key, value));
                                    }
                                }
                                None => out.push(Patch::Add(key, value)),
                            }
                        }
                        break;
                    }
                    // removed attributes
                    (None, Some(attr)) => {
                        for (key, _) in iter::once(attr).chain(old_iter) {
                            if !new.contains_key(key) {
                                out.push(Patch::Remove(*key));
                            }
                        }
                        break;
                    }
                    (None, None) => break,
                }
            }

            out
        }

        fn diff<'a>(new: &'a Self, old: &'a Self) -> Vec<Patch<&'static str, &'a str>> {
            match (new, old) {
                (Self::Vec(new), Self::Vec(old)) => Self::diff_vec(new, old),
                (Self::Vec(new), Self::IndexMap(old)) => {
                    // this case is somewhat tricky because we need to return references to the values in `new`
                    // but we also want to turn `new` into a hash map for performance reasons
                    let new_iter = new
                        .iter()
                        .filter_map(PositionalAttr::transposed)
                        .map(|(k, v)| (k, v.as_ref()));
                    // create a "view" over references to the actual data in `new`.
                    let new = new.iter().filter_map(PositionalAttr::transposed).collect();
                    Self::diff_index_map(new_iter, &new, old)
                }
                (Self::IndexMap(new), Self::Vec(old)) => {
                    let new_iter = new.iter().map(|(k, v)| (*k, v.as_ref()));
                    Self::diff_index_map(
                        new_iter,
                        new,
                        &old.iter().filter_map(PositionalAttr::transposed).collect(),
                    )
                }
                (Self::IndexMap(new), Self::IndexMap(old)) => {
                    let new_iter = new.iter().map(|(k, v)| (*k, v.as_ref()));
                    Self::diff_index_map(new_iter, new, old)
                }
            }
        }
    }

    impl From<Vec<PositionalAttr>> for Attributes {
        fn from(v: Vec<PositionalAttr>) -> Self {
            Self::Vec(v)
        }
    }
    impl From<IndexMap<&'static str, Cow<'static, str>>> for Attributes {
        fn from(v: IndexMap<&'static str, Cow<'static, str>>) -> Self {
            Self::IndexMap(v)
        }
    }

    impl Default for Attributes {
        fn default() -> Self {
            Self::Vec(Default::default())
        }
    }
}

mod vdiff {
    use super::*;

    /// Patch for DOM node modification.
    #[derive(Debug, PartialEq)]
    pub(crate) enum Patch<ID, T> {
        Add(ID, T),
        Replace(ID, T),
        Remove(ID),
    }

    // TODO(#938): What about implementing `VDiff` for `Element`?
    // It would make it possible to include ANY element into the tree.
    // `Ace` editor embedding for example?

    /// This trait provides features to update a tree by calculating a difference against another tree.
    pub(crate) trait VDiff {
        /// Remove self from parent.
        fn detach(&mut self, parent: &Element);

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
            &mut self,
            parent_scope: &AnyScope,
            parent: &Element,
            next_sibling: NodeRef,
            ancestor: Option<VNode>,
        ) -> NodeRef;
    }
}

fn insert_node() {
    // fn insert_node(node: &Node, parent: &Element, next_sibling: Option<Node>) {
    todo!("@jon, insert node needs to work");
    // match next_sibling {
    //     Some(next_sibling) => parent
    //         .insert_before(&node, Some(&next_sibling))
    //         .expect("failed to insert tag before next sibling"),
    //     None => parent.append_child(node).expect("failed to append child"),
    // };
    // match next_sibling {
    //     Some(next_sibling) => parent
    //         .insert_before(&node, Some(&next_sibling))
    //         .expect("failed to insert tag before next sibling"),
    //     None => parent.append_child(node).expect("failed to append child"),
    // };
    // if let Some(next_sibling) = next_sibling {
    //     parent
    //         .insert_before(node, &next_sibling)
    //         .expect("failed to insert tag before next sibling");
    // } else {
    //     parent.append_child(node);
    // }
}
