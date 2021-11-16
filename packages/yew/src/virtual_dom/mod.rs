//! This module contains Yew's implementation of a reactive virtual DOM.

#[doc(hidden)]
pub mod key;
#[doc(hidden)]
pub mod listeners;
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
use indexmap::IndexMap;
use std::{collections::HashMap, fmt, hint::unreachable_unchecked, iter};
use web_sys::{Element, Node};

#[doc(inline)]
pub use self::key::Key;
#[doc(inline)]
pub use self::listeners::*;
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
use std::fmt::Formatter;
use std::ops::Deref;
use std::rc::Rc;

/// Attribute value
#[derive(Eq, PartialEq, Debug)]
pub enum AttrValue {
    /// String living for `'static`
    Static(&'static str),
    /// Owned string
    Owned(String),
    /// Reference counted string
    Rc(Rc<str>),
}

impl Deref for AttrValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            AttrValue::Static(s) => *s,
            AttrValue::Owned(s) => s.as_str(),
            AttrValue::Rc(s) => &*s,
        }
    }
}

impl From<&'static str> for AttrValue {
    fn from(s: &'static str) -> Self {
        AttrValue::Static(s)
    }
}

impl From<String> for AttrValue {
    fn from(s: String) -> Self {
        AttrValue::Owned(s)
    }
}

impl From<Rc<str>> for AttrValue {
    fn from(s: Rc<str>) -> Self {
        AttrValue::Rc(s)
    }
}

impl Clone for AttrValue {
    fn clone(&self) -> Self {
        match self {
            AttrValue::Static(s) => AttrValue::Static(s),
            AttrValue::Owned(s) => AttrValue::Owned(s.clone()),
            AttrValue::Rc(s) => AttrValue::Rc(Rc::clone(s)),
        }
    }
}

impl AsRef<str> for AttrValue {
    fn as_ref(&self) -> &str {
        &*self
    }
}

impl fmt::Display for AttrValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AttrValue::Static(s) => write!(f, "{}", s),
            AttrValue::Owned(s) => write!(f, "{}", s),
            AttrValue::Rc(s) => write!(f, "{}", s),
        }
    }
}

/// Applies contained changes to DOM [Element]
trait Apply {
    /// [Element] type to apply the changes to
    type Element;

    /// Apply contained values to [Element] with no ancestor
    fn apply(&mut self, el: &Self::Element);

    /// Apply diff between [self] and `ancestor` to [Element].
    fn apply_diff(&mut self, el: &Self::Element, ancestor: Self);
}

/// A collection of attributes for an element
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Attributes {
    /// Static list of attributes.
    ///
    /// Allows optimizing comparison to a simple pointer equality check and reducing allocations,
    /// if the attributes do not change on a node.
    Static(&'static [[&'static str; 2]]),

    /// Static list of attribute keys with possibility to exclude attributes and dynamic attribute
    /// values.
    ///
    /// Allows optimizing comparison to a simple pointer equality check and reducing allocations,
    /// if the attributes keys do not change on a node.
    Dynamic {
        /// Attribute keys. Includes both always set and optional attribute keys.
        keys: &'static [&'static str],

        /// Attribute values. Matches [keys]. Optional attributes are designated by setting [None].
        values: Box<[Option<AttrValue>]>,
    },

    /// IndexMap is used to provide runtime attribute deduplication in cases where the html! macro
    /// was not used to guarantee it.
    IndexMap(IndexMap<&'static str, AttrValue>),
}

impl Attributes {
    /// Construct a default Attributes instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Return iterator over attribute key-value pairs.
    /// This function is suboptimal and does not inline well. Avoid on hot paths.
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'static str, &'a str)> + 'a> {
        match self {
            Self::Static(arr) => Box::new(arr.iter().map(|kv| (kv[0], kv[1] as &'a str))),
            Self::Dynamic { keys, values } => Box::new(
                keys.iter()
                    .zip(values.iter())
                    .filter_map(|(k, v)| v.as_ref().map(|v| (*k, v.as_ref()))),
            ),
            Self::IndexMap(m) => Box::new(m.iter().map(|(k, v)| (*k, v.as_ref()))),
        }
    }

    /// Get a mutable reference to the underlying `IndexMap`.
    /// If the attributes are stored in the `Vec` variant, it will be converted.
    pub fn get_mut_index_map(&mut self) -> &mut IndexMap<&'static str, AttrValue> {
        macro_rules! unpack {
            () => {
                match self {
                    Self::IndexMap(m) => m,
                    // SAFETY: unreachable because we set self to the `IndexMap` variant above.
                    _ => unsafe { unreachable_unchecked() },
                }
            };
        }

        match self {
            Self::IndexMap(m) => m,
            Self::Static(arr) => {
                *self = Self::IndexMap(arr.iter().map(|kv| (kv[0], kv[1].into())).collect());
                unpack!()
            }
            Self::Dynamic { keys, values } => {
                *self = Self::IndexMap(
                    std::mem::take(values)
                        .iter_mut()
                        .zip(keys.iter())
                        .filter_map(|(v, k)| v.take().map(|v| (*k, v)))
                        .collect(),
                );
                unpack!()
            }
        }
    }

    #[cold]
    fn apply_diff_index_maps<'a, A, B>(
        el: &Element,
        // this makes it possible to diff `&'a IndexMap<_, A>` and `IndexMap<_, &'a A>`.
        mut new_iter: impl Iterator<Item = (&'static str, &'a str)>,
        new: &IndexMap<&'static str, A>,
        old: &IndexMap<&'static str, B>,
    ) where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let mut old_iter = old.iter();
        loop {
            match (new_iter.next(), old_iter.next()) {
                (Some((new_key, new_value)), Some((old_key, old_value))) => {
                    if new_key != *old_key {
                        break;
                    }
                    if new_value != old_value.as_ref() {
                        Self::set_attribute(el, new_key, new_value);
                    }
                }
                // new attributes
                (Some(attr), None) => {
                    for (key, value) in iter::once(attr).chain(new_iter) {
                        match old.get(key) {
                            Some(old_value) => {
                                if value != old_value.as_ref() {
                                    Self::set_attribute(el, key, value);
                                }
                            }
                            None => {
                                Self::set_attribute(el, key, value);
                            }
                        }
                    }
                    break;
                }
                // removed attributes
                (None, Some(attr)) => {
                    for (key, _) in iter::once(attr).chain(old_iter) {
                        if !new.contains_key(key) {
                            Self::remove_attribute(el, key);
                        }
                    }
                    break;
                }
                (None, None) => break,
            }
        }
    }

    /// Convert [Attributes] pair to [HashMap]s and patch changes to `el`.
    /// Works with any [Attributes] variants.
    #[cold]
    fn apply_diff_as_maps<'a>(el: &Element, new: &'a Self, old: &'a Self) {
        fn collect<'a>(src: &'a Attributes) -> HashMap<&'static str, &'a str> {
            use Attributes::*;

            match src {
                Static(arr) => (*arr).iter().map(|[k, v]| (*k, *v)).collect(),
                Dynamic { keys, values } => keys
                    .iter()
                    .zip(values.iter())
                    .filter_map(|(k, v)| v.as_ref().map(|v| (*k, v.as_ref())))
                    .collect(),
                IndexMap(m) => m.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            }
        }

        let new = collect(new);
        let old = collect(old);

        // Update existing or set new
        for (k, new) in new.iter() {
            if match old.get(k) {
                Some(old) => old != new,
                None => true,
            } {
                el.set_attribute(k, new).unwrap();
            }
        }

        // Remove missing
        for k in old.keys() {
            if !new.contains_key(k) {
                Self::remove_attribute(el, k);
            }
        }
    }

    fn set_attribute(el: &Element, key: &str, value: &str) {
        el.set_attribute(key, value).expect("invalid attribute key")
    }

    fn remove_attribute(el: &Element, key: &str) {
        el.remove_attribute(key)
            .expect("could not remove attribute")
    }
}

impl Apply for Attributes {
    type Element = Element;

    fn apply(&mut self, el: &Element) {
        match self {
            Self::Static(arr) => {
                for kv in arr.iter() {
                    Self::set_attribute(el, kv[0], kv[1]);
                }
            }
            Self::Dynamic { keys, values } => {
                for (k, v) in keys.iter().zip(values.iter()) {
                    if let Some(v) = v {
                        Self::set_attribute(el, k, v)
                    }
                }
            }
            Self::IndexMap(m) => {
                for (k, v) in m.iter() {
                    Self::set_attribute(el, k, v)
                }
            }
        }
    }

    fn apply_diff(&mut self, el: &Element, ancestor: Self) {
        #[inline]
        fn ptr_eq<T>(a: &[T], b: &[T]) -> bool {
            a.as_ptr() == b.as_ptr()
        }

        match (self, ancestor) {
            // Hot path
            (Self::Static(new), Self::Static(old)) if ptr_eq(new, old) => (),
            // Hot path
            (
                Self::Dynamic {
                    keys: new_k,
                    values: new_v,
                },
                Self::Dynamic {
                    keys: old_k,
                    values: old_v,
                },
            ) if ptr_eq(new_k, old_k) => {
                // Double zipping does not optimize well, so use asserts and unsafe instead
                assert!(new_k.len() == new_v.len());
                assert!(new_k.len() == old_v.len());
                for i in 0..new_k.len() {
                    macro_rules! key {
                        () => {
                            unsafe { new_k.get_unchecked(i) }
                        };
                    }
                    macro_rules! set {
                        ($new:expr) => {
                            Self::set_attribute(el, key!(), $new)
                        };
                    }

                    match unsafe { (new_v.get_unchecked(i), old_v.get_unchecked(i)) } {
                        (Some(new), Some(old)) => {
                            if new != old {
                                set!(new);
                            }
                        }
                        (Some(new), None) => set!(new),
                        (None, Some(_)) => {
                            Self::remove_attribute(el, key!());
                        }
                        (None, None) => (),
                    }
                }
            }
            // For VTag's constructed outside the html! macro
            (Self::IndexMap(new), Self::IndexMap(old)) => {
                let new_iter = new.iter().map(|(k, v)| (*k, v.as_ref()));
                Self::apply_diff_index_maps(el, new_iter, new, &old);
            }
            // Cold path. Happens only with conditional swapping and reordering of `VTag`s with the
            // same tag and no keys.
            (new, ancestor) => {
                Self::apply_diff_as_maps(el, new, &ancestor);
            }
        }
    }
}

impl From<IndexMap<&'static str, AttrValue>> for Attributes {
    fn from(v: IndexMap<&'static str, AttrValue>) -> Self {
        Self::IndexMap(v)
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::Static(&[])
    }
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

pub(crate) fn insert_node(node: &Node, parent: &Element, next_sibling: Option<&Node>) {
    match next_sibling {
        Some(next_sibling) => parent
            .insert_before(node, Some(next_sibling))
            .expect("failed to insert tag before next sibling"),
        None => parent.append_child(node).expect("failed to append child"),
    };
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::html::{AnyScope, Scope};
    use crate::{Component, Context, Html};

    struct Comp;
    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn create(_: &Context<Self>) -> Self {
            unimplemented!()
        }

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            unimplemented!();
        }

        fn changed(&mut self, _ctx: &Context<Self>) -> bool {
            unimplemented!()
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            unimplemented!()
        }
    }

    pub(crate) struct TestLayout<'a> {
        pub(crate) name: &'a str,
        pub(crate) node: VNode,
        pub(crate) expected: &'a str,
    }

    pub(crate) fn diff_layouts(layouts: Vec<TestLayout<'_>>) {
        let document = gloo_utils::document();
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

#[cfg(all(test, feature = "wasm_bench"))]
mod benchmarks {
    use super::*;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    macro_rules! run {
        ($name:ident => {
            $( $old:expr => $new:expr )+
        }) => {
            // NB: these benchmarks only compare diffing. They do not take into account aspects like
            // allocation impact, which is lower for both `Static` and `Dynamic`.

            let results = vec![
                $(
                    {
                        let mut old = $old.clone();
                        let new = $new.clone();
                        let el = gloo_utils::document().create_element("div").unwrap();
                        old.apply(&el);
                        (
                            format!("{} -> {}", attr_variant(&old), attr_variant(&new)),
                            easybench_wasm::bench_env_limit(
                                2.0,
                                (NodeCloner(el), new, old),
                                |(el, mut new, old)| new.apply_diff(&el.0, old),
                            ),
                        )
                    },
                )+
            ];

            let max_name_len = results.iter().map(|(name, _)| name.len()).max().unwrap_or_default();
            wasm_bindgen_test::console_log!(
                "{}:{}",
                stringify!($name),
                results.into_iter().fold(String::new(), |mut acc, (name, res)| {
                    use std::fmt::Write;

                    write!(&mut acc, "\n\t\t{:<width$}: ", name, width=max_name_len).unwrap();

                    if res.ns_per_iter.is_nan() {
                        acc += "benchmark too slow to produce meaningful results";
                    } else {
                        write!(
                            &mut acc,
                            "{:>7.4} ns (R²={:.3}, {:>7} iterations in {:>3} samples)",
                            res.ns_per_iter,
                            res.goodness_of_fit,
                            res.iterations,
                            res.samples,
                        )
                        .unwrap();
                    }

                    acc
                })
            );
        };
    }

    #[wasm_bindgen_test]
    fn bench_diff_empty() {
        let static_ = Attributes::Static(&[]);
        let dynamic = Attributes::Dynamic {
            keys: &[],
            values: vec![],
        };
        let map = Attributes::IndexMap(Default::default());

        run! {
            empty => {
                static_ => static_
                dynamic => dynamic
                map => map
                static_ => dynamic
                static_ => map
                dynamic => map
            }
        }
    }

    #[wasm_bindgen_test]
    fn bench_diff_equal() {
        let static_ = Attributes::Static(sample_attrs());
        let dynamic = make_dynamic(sample_values());
        let map = make_indexed_map(sample_values());

        run! {
            equal => {
                static_ => static_
                dynamic => dynamic
                map => map
                static_ => dynamic
                static_ => map
                dynamic => map
            }
        }
    }

    #[wasm_bindgen_test]
    fn bench_diff_change_first() {
        let old = sample_values();
        let mut new = old.clone();
        new[0] = AttrValue::Static("changed");

        let dynamic = (make_dynamic(old.clone()), make_dynamic(new.clone()));
        let map = (make_indexed_map(old), make_indexed_map(new));

        run! {
            changed_first => {
                dynamic.0 => dynamic.1
                map.0 => map.1
                dynamic.0 => map.1
            }
        }
    }

    fn make_dynamic(values: Vec<AttrValue>) -> Attributes {
        Attributes::Dynamic {
            keys: sample_keys(),
            values: values.into_iter().map(|v| Some(v)).collect(),
        }
    }

    fn make_indexed_map(values: Vec<AttrValue>) -> Attributes {
        Attributes::IndexMap(
            sample_keys()
                .iter()
                .copied()
                .zip(values.into_iter())
                .collect(),
        )
    }

    fn sample_keys() -> &'static [&'static str] {
        &[
            "oh", "boy", "pipes", "are", "from", "to", "and", "the", "side",
        ]
    }

    fn sample_values() -> Vec<AttrValue> {
        [
            "danny", "the", "the", "calling", "glen", "glen", "down", "mountain", "",
        ]
        .iter()
        .map(|v| AttrValue::Static(*v))
        .collect()
    }

    fn sample_attrs() -> &'static [[&'static str; 2]] {
        &[
            ["oh", "danny"],
            ["boy", "the"],
            ["pipes", "the"],
            ["are", "calling"],
            ["from", "glen"],
            ["to", "glen"],
            ["and", "down"],
            ["the", "mountain"],
            ["side", ""],
        ]
    }

    fn attr_variant(attrs: &Attributes) -> &'static str {
        use Attributes::*;

        match attrs {
            Static(_) => "static",
            Dynamic { .. } => "dynamic",
            IndexMap(_) => "indexed_map",
        }
    }

    /// Clones the node on [Clone] call
    struct NodeCloner(Element);

    impl Clone for NodeCloner {
        fn clone(&self) -> Self {
            use wasm_bindgen::JsCast;

            Self(self.0.clone_node().unwrap().dyn_into().unwrap())
        }
    }
}
