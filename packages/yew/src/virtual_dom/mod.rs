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
pub mod vportal;
#[doc(hidden)]
pub mod vsuspense;
#[doc(hidden)]
pub mod vtag;
#[doc(hidden)]
pub mod vtext;

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
pub use self::vportal::VPortal;
#[doc(inline)]
pub use self::vsuspense::VSuspense;
#[doc(inline)]
pub use self::vtag::VTag;
#[doc(inline)]
pub use self::vtext::VText;

use indexmap::IndexMap;
use std::hint::unreachable_unchecked;

/// Attribute value
pub type AttrValue = imut::IString;

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

        /// Attribute values. Matches [keys](Attributes::Dynamic::keys). Optional attributes are designated by setting [None].
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
            values: Box::new([]),
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
            values: values.into_iter().map(Some).collect(),
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
