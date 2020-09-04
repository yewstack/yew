//! This module contains Yew's implementation of a reactive virtual DOM.

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

use crate::html::{AnyScope, NodeRef};
use cfg_if::cfg_if;
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;
use std::fmt;
use std::{collections::HashMap, hint::unreachable_unchecked, iter, mem, rc::Rc};
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

/// The `Listener` trait is an universal implementation of an event listener
/// which is used to bind Rust-listener to JS-listener (DOM).
pub trait Listener {
    /// Returns the name of the event
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

/// Key-value tuple which makes up an item of the [`Attributes::Vec`] variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositionalAttr(pub &'static str, pub Option<Cow<'static, str>>);
impl PositionalAttr {
    /// Create a positional attribute
    pub fn new(key: &'static str, value: Cow<'static, str>) -> Self {
        Self(key, Some(value))
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
                (Some(_), Some(_)) => {
                    let new = new_iter
                        .filter_map(PositionalAttr::transposed)
                        .collect::<HashMap<_, _>>();
                    let old = old_iter
                        .filter_map(PositionalAttr::transposed)
                        .collect::<HashMap<_, _>>();

                    for (key, value) in new.iter() {
                        match old.get(key) {
                            Some(old_value) => {
                                if value != old_value {
                                    out.push(Patch::Replace(*key, (*value).as_ref()));
                                }
                            }
                            None => out.push(Patch::Add(*key, (*value).as_ref())),
                        };
                    }

                    for key in old.keys() {
                        if !new.contains_key(key) {
                            out.push(Patch::Remove(*key));
                        }
                    }
                    break;
                }
                // added attributes
                (Some(attr), None) => {
                    for PositionalAttr(key, value) in new_iter.chain(iter::once(attr)) {
                        // only add value if it has a value
                        if let Some(value) = value {
                            out.push(Patch::Add(*key, value));
                        }
                    }
                    break;
                }
                // removed attributes
                (None, Some(attr)) => {
                    for PositionalAttr(key, value) in old_iter.chain(iter::once(attr)) {
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
        new: &'a IndexMap<&'static str, A>,
        old: &IndexMap<&'static str, B>,
    ) -> Vec<Patch<&'static str, &'a str>>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let mut out = Vec::new();
        let mut new_iter = new.iter();
        let mut old_iter = old.iter();
        loop {
            match (new_iter.next(), old_iter.next()) {
                (Some((new_key, new_value)), Some((old_key, old_value))) => {
                    if new_key != old_key {
                        break;
                    }
                    let (new_value, old_value) = (new_value.as_ref(), old_value.as_ref());
                    if new_value != old_value {
                        out.push(Patch::Replace(*new_key, new_value));
                    }
                }
                // new attributes
                (Some(attr), None) => {
                    for (key, value) in new_iter.chain(iter::once(attr)) {
                        let value = value.as_ref();
                        match old.get(key) {
                            Some(old_value) => {
                                let old_value = old_value.as_ref();
                                if value != old_value {
                                    out.push(Patch::Replace(*key, value));
                                }
                            }
                            None => out.push(Patch::Add(*key, value)),
                        }
                    }
                    break;
                }
                // removed attributes
                (None, Some(attr)) => {
                    for (key, _) in old_iter.chain(iter::once(attr)) {
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

    fn diff<'a>(new: &'a mut Self, old: &'a Self) -> Vec<Patch<&'static str, &'a str>> {
        match (new, old) {
            // both vectors
            (Self::Vec(new), Self::Vec(old)) => Self::diff_vec(new, old),
            // mixed -> mutate `new` to be an indexmap
            // TODO update when "move_ref_pattern" lands (<https://github.com/rust-lang/rust/issues/68354>)
            (new @ Self::Vec(_), old @ Self::IndexMap(_)) => {
                if let Self::IndexMap(old) = old {
                    Self::diff_index_map(new.as_mut(), old)
                } else {
                    // SAFETY: we already know `old` is an `IndexMap` because of the pattern,
                    //         but we can't mix by-move and by-ref patterns until 'move_ref_pattern' lands
                    unsafe { unreachable_unchecked() }
                }
            }
            (Self::IndexMap(new), Self::Vec(old)) => Self::diff_index_map(
                new,
                &old.iter().filter_map(PositionalAttr::transposed).collect(),
            ),
            // both indexmap
            (Self::IndexMap(new), Self::IndexMap(old)) => Self::diff_index_map(new, old),
        }
    }
}

impl AsMut<IndexMap<&'static str, Cow<'static, str>>> for Attributes {
    fn as_mut(&mut self) -> &mut IndexMap<&'static str, Cow<'static, str>> {
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

/// A set of classes.
#[derive(Debug, Clone, Default)]
pub struct Classes {
    set: IndexSet<String>,
}

impl Classes {
    /// Creates an empty set of classes.
    pub fn new() -> Self {
        Self {
            set: IndexSet::new(),
        }
    }

    /// Adds a class to a set.
    ///
    /// If the provided class has already been added, this method will ignore it.
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
        Classes::from(t.as_slice())
    }
}

impl<T: AsRef<str>> From<&[T]> for Classes {
    fn from(t: &[T]) -> Self {
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

#[cfg(feature = "web_sys")]
fn insert_node(node: &Node, parent: &Element, next_sibling: Option<Node>) {
    match next_sibling {
        Some(next_sibling) => parent
            .insert_before(&node, Some(&next_sibling))
            .expect("failed to insert tag before next sibling"),
        None => parent.append_child(node).expect("failed to append child"),
    };
}

#[cfg(feature = "std_web")]
fn insert_node(node: &impl INode, parent: &impl INode, next_sibling: Option<Node>) {
    if let Some(next_sibling) = next_sibling {
        parent
            .insert_before(node, &next_sibling)
            .expect("failed to insert tag before next sibling");
    } else {
        parent.append_child(node);
    }
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

// stdweb lacks the `inner_html` method
#[cfg(all(test, feature = "web_sys"))]
mod layout_tests {
    use super::*;
    use crate::html::{AnyScope, Scope};
    use crate::{Component, ComponentLink, Html, ShouldRender};

    struct Comp;
    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            unimplemented!()
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            unimplemented!()
        }

        fn view(&self) -> Html {
            unimplemented!()
        }
    }

    pub(crate) struct TestLayout<'a> {
        pub(crate) name: &'a str,
        pub(crate) node: VNode,
        pub(crate) expected: &'a str,
    }

    pub(crate) fn diff_layouts(layouts: Vec<TestLayout<'_>>) {
        let document = crate::utils::document();
        let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
        let parent_element = document.create_element("div").unwrap();
        let parent_node: Node = parent_element.clone().into();
        let end_node = document.create_text_node("END");
        parent_node.append_child(&end_node).unwrap();
        let mut empty_node: VNode = VText::new("").into();

        // Tests each layout independently
        let next_sibling = NodeRef::new(end_node.into());
        for layout in layouts.iter() {
            // Apply the layout
            let mut node = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Independently apply layout '{}'", layout.name);
            node.apply(&parent_scope, &parent_element, next_sibling.clone(), None);
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Independent apply failed for layout '{}'",
                layout.name,
            );

            // Diff with no changes
            let mut node_clone = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Independently reapply layout '{}'", layout.name);
            node_clone.apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                Some(node),
            );
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Independent reapply failed for layout '{}'",
                layout.name,
            );

            // Detach
            empty_node.clone().apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                Some(node_clone),
            );
            assert_eq!(
                parent_element.inner_html(),
                "END",
                "Independent detach failed for layout '{}'",
                layout.name,
            );
        }

        // Sequentially apply each layout
        let mut ancestor: Option<VNode> = None;
        for layout in layouts.iter() {
            let mut next_node = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Sequentially apply layout '{}'", layout.name);
            next_node.apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                ancestor,
            );
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Sequential apply failed for layout '{}'",
                layout.name,
            );
            ancestor = Some(next_node);
        }

        // Sequentially detach each layout
        for layout in layouts.into_iter().rev() {
            let mut next_node = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Sequentially detach layout '{}'", layout.name);
            next_node.apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                ancestor,
            );
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Sequential detach failed for layout '{}'",
                layout.name,
            );
            ancestor = Some(next_node);
        }

        // Detach last layout
        empty_node.apply(&parent_scope, &parent_element, next_sibling, ancestor);
        assert_eq!(
            parent_element.inner_html(),
            "END",
            "Failed to detach last layout"
        );
    }
}
