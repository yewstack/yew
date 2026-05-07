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
pub mod vraw;
#[doc(hidden)]
pub mod vsuspense;
#[doc(hidden)]
pub mod vtag;
#[doc(hidden)]
pub mod vtext;

use std::hint::unreachable_unchecked;
use std::rc::Rc;

use indexmap::IndexMap;
use wasm_bindgen::JsValue;

#[doc(inline)]
pub use self::key::Key;
#[doc(inline)]
pub use self::listeners::*;
#[doc(inline)]
pub use self::vcomp::{VChild, VComp};
#[doc(hidden)]
pub use self::vlist::FullyKeyedState;
#[doc(inline)]
pub use self::vlist::VList;
#[doc(inline)]
pub use self::vnode::VNode;
#[doc(inline)]
pub use self::vportal::VPortal;
#[doc(inline)]
pub use self::vraw::VRaw;
#[doc(inline)]
pub use self::vsuspense::VSuspense;
#[doc(inline)]
pub use self::vtag::VTag;
#[doc(inline)]
pub use self::vtext::VText;

/// Attribute value
pub type AttrValue = implicit_clone::unsync::IString;

#[cfg(any(feature = "ssr", feature = "hydration"))]
mod feat_ssr_hydration {
    #[cfg(debug_assertions)]
    type ComponentName = &'static str;
    #[cfg(not(debug_assertions))]
    type ComponentName = std::marker::PhantomData<()>;

    #[cfg(feature = "hydration")]
    use std::borrow::Cow;

    /// A collectable.
    ///
    /// This indicates a kind that can be collected from fragment to be processed at a later time
    pub enum Collectable {
        Component(ComponentName),
        Raw,
        Suspense,
    }

    impl Collectable {
        #[cfg(not(debug_assertions))]
        #[inline(always)]
        pub fn for_component<T: 'static>() -> Self {
            use std::marker::PhantomData;
            // This suppresses the clippy lint about unused generic.
            // We inline this function
            // so the function body is copied to its caller and generics get optimised away.
            let _comp_type: PhantomData<T> = PhantomData;
            Self::Component(PhantomData)
        }

        #[cfg(debug_assertions)]
        pub fn for_component<T: 'static>() -> Self {
            let comp_name = std::any::type_name::<T>();
            Self::Component(comp_name)
        }

        pub fn open_start_mark(&self) -> &'static str {
            match self {
                Self::Component(_) => "<[",
                Self::Raw => "<#",
                Self::Suspense => "<?",
            }
        }

        pub fn close_start_mark(&self) -> &'static str {
            match self {
                Self::Component(_) => "</[",
                Self::Raw => "</#",
                Self::Suspense => "</?",
            }
        }

        pub fn end_mark(&self) -> &'static str {
            match self {
                Self::Component(_) => "]>",
                Self::Raw => ">",
                Self::Suspense => ">",
            }
        }

        #[cfg(feature = "hydration")]
        pub fn name(&self) -> Cow<'static, str> {
            match self {
                #[cfg(debug_assertions)]
                Self::Component(m) => format!("Component({m})").into(),
                #[cfg(not(debug_assertions))]
                Self::Component(_) => "Component".into(),
                Self::Raw => "Raw".into(),
                Self::Suspense => "Suspense".into(),
            }
        }
    }
}

#[cfg(any(feature = "ssr", feature = "hydration"))]
pub(crate) use feat_ssr_hydration::*;

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt::Write;

    use super::*;
    use crate::platform::fmt::BufWriter;

    impl Collectable {
        pub(crate) fn write_open_tag(&self, w: &mut BufWriter) {
            let _ = w.write_str("<!--");
            let _ = w.write_str(self.open_start_mark());

            #[cfg(debug_assertions)]
            match self {
                Self::Component(type_name) => {
                    let _ = w.write_str(type_name);
                }
                Self::Raw => {}
                Self::Suspense => {}
            }

            let _ = w.write_str(self.end_mark());
            let _ = w.write_str("-->");
        }

        pub(crate) fn write_close_tag(&self, w: &mut BufWriter) {
            let _ = w.write_str("<!--");
            let _ = w.write_str(self.close_start_mark());

            #[cfg(debug_assertions)]
            match self {
                Self::Component(type_name) => {
                    let _ = w.write_str(type_name);
                }
                Self::Raw => {}
                Self::Suspense => {}
            }

            let _ = w.write_str(self.end_mark());
            let _ = w.write_str("-->");
        }
    }
}

/// Defines if the [`Attributes`] is set as element's attribute or property and its value.
#[expect(missing_docs)]
#[derive(PartialEq, Clone, Debug)]
pub enum AttributeOrProperty {
    Attribute(AttrValue),
    Property(JsValue),
}

fn is_valid_attr_name(attr: &str) -> bool {
    // https://dom.spec.whatwg.org/#valid-attribute-local-name specifies:
    // > at least one character, no ASCII whitespace, no \x00, \x2F (/), \x3D (=), \x3E (>)
    // Browsers are more strict in setAttribute(), and the parser for a html document is too.
    // The parser specifies that names must consist of
    // > one or more [unicode] characters other than the space characters [ \t\r\n\f],
    // > U+0000 NULL, U+0022 QUOTATION MARK ("), U+0027 APOSTROPHE ('), U+003E GREATER-THAN SIGN
    // > (>),
    // > U+002F SOLIDUS (/), and U+003D EQUALS SIGN (=) characters, the control characters , [...]
    // ref https://w3c.github.io/html-reference/syntax.html#attribute-name
    // A fun thing is trying to use non-ascii-whitespace WHITE_SPACE characters such as " "
    // as part of an attribute name, which seem to get accepted by the parser but not by
    // setAttribute.

    // Anyway, the goal here is to allow a reasonable subset of names that will parse correctly
    // in all browsers when delivered via SSR and also work correctly when used in calls to
    // setAttribute. Here is what we will prohibit:
    // - all whitespace characters (unicode WS) and control characters (unicode Cc)
    // - all syntactically special characters for XHTML parsing [/"'>=]
    // - disallow "high" unicode characters in the range [#x10000-#xEFFFF]
    // see also: https://github.com/whatwg/dom/issues/849

    // If you really need attribute names that are not covered by this open an issue and a self-help
    // group. Know that I do care for your pain.
    !attr.is_empty()
        && attr.chars().all(|c| match c {
            c if c.is_control() => false,
            '/' | '"' | '\'' | '>' | '=' => false,
            c if c.is_whitespace() => false,
            // For example, try the following in a browser of your choice:
            // ```
            // let d = document.createElement("div")
            // d.setAttribute("𐊖𐊗𐊒𐊓", "value2")
            // ```
            // At the time of writing, the above works in chrome (131) but not in firefox (136)
            '\u{10000}'.. => false,
            _ => true,
        })
}

#[track_caller]
fn validate_attr_name(attr: &str) {
    assert!(
        is_valid_attr_name(attr),
        "{attr:?} is not a valid attribute name"
    );
}

/// A collection of attributes for an element
#[derive(PartialEq, Clone, Debug)]
#[non_exhaustive]
pub enum Attributes {
    #[doc(hidden)]
    #[deprecated = "Attribute names are not validated. Use one of the conversion functions"]
    Static(&'static [(&'static str, AttributeOrProperty)]),

    #[doc(hidden)]
    #[deprecated = "Attribute names are not validated. Use one of the conversion functions"]
    Dynamic {
        /// Attribute keys. Includes both always set and optional attribute keys.
        keys: &'static [&'static str],

        /// Attribute values. Matches [keys](Attributes::Dynamic::keys). Optional attributes are
        /// designated by setting [None].
        values: Box<[Option<AttributeOrProperty>]>,
    },

    #[doc(hidden)]
    #[deprecated = "Attribute names are not validated. Use one of the conversion functions"]
    IndexMap(Rc<IndexMap<AttrValue, AttributeOrProperty>>),
}

impl Attributes {
    /// Static list of attributes.
    ///
    /// Allows optimizing comparison to a simple pointer equality check and reducing allocations,
    /// if the attributes do not change on a node.
    #[track_caller]
    pub fn from_static(statics: &'static [(&'static str, AttributeOrProperty)]) -> Self {
        for &(key, _) in statics {
            validate_attr_name(key); // Not in a closure for #[track_caller]
        }
        Self::from_static_unchecked(statics)
    }

    /// Same as [Self::from_static] but without verifying keys. This can lead to loss of
    /// validity of an SSR document!
    pub fn from_static_unchecked(statics: &'static [(&'static str, AttributeOrProperty)]) -> Self {
        #[expect(deprecated)]
        Self::Static(statics)
    }

    /// Static list of attribute keys with possibility to exclude attributes and dynamic attribute
    /// values.
    ///
    /// Allows optimizing comparison to a simple pointer equality check and reducing allocations,
    /// if the attributes keys do not change on a node.
    #[track_caller]
    pub fn from_dynamic_values(
        keys: &'static [&'static str],
        values: Box<[Option<AttributeOrProperty>]>,
    ) -> Self {
        for &key in keys {
            validate_attr_name(key); // Not in a closure for #[track_caller]
        }
        Self::from_dynamic_values_unchecked(keys, values)
    }

    /// Same as [Self::from_dynamic_values] but without verifying keys. This can lead to loss of
    /// validity of an SSR document!
    pub fn from_dynamic_values_unchecked(
        keys: &'static [&'static str],
        values: Box<[Option<AttributeOrProperty>]>,
    ) -> Self {
        #[expect(deprecated)]
        Self::Dynamic { keys, values }
    }

    /// IndexMap is used to provide runtime attribute deduplication in cases where the html! macro
    /// was not used to guarantee it.
    #[track_caller]
    pub fn from_index_map(map: Rc<IndexMap<AttrValue, AttributeOrProperty>>) -> Self {
        for (key, _) in map.iter() {
            validate_attr_name(key); // Not in a closure for #[track_caller]
        }
        Self::from_index_map_unchecked(map)
    }

    /// Same as [Self::from_index_map] but without verifying keys. This can lead to loss of validity
    /// of an SSR document!
    pub fn from_index_map_unchecked(map: Rc<IndexMap<AttrValue, AttributeOrProperty>>) -> Self {
        #[expect(deprecated)]
        Self::IndexMap(map)
    }

    /// Validate a single attribute name for validity to be used as a key for an attribute or
    /// property. All keys must be valid according to this method when constructing
    /// [`Attributes`]. Usually, this is ensured by the usage context.
    ///
    /// Specifically, this checks that the passed string is a valid XHTML attribute name. This
    /// implies that it consists of at least one character, does not contain whitespace or other
    /// special characters.
    pub fn is_valid_attr_key(name: &str) -> bool {
        is_valid_attr_name(name)
    }

    /// Construct a default Attributes instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Return iterator over attribute key-value pairs.
    /// This function is suboptimal and does not inline well. Avoid on hot paths.
    ///
    /// This function only returns attributes
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a> {
        #[expect(deprecated)]
        match self {
            Self::Static(arr) => Box::new(arr.iter().filter_map(|(k, v)| match v {
                AttributeOrProperty::Attribute(v) => Some((*k, v.as_ref())),
                AttributeOrProperty::Property(_) => None,
            })),
            Self::Dynamic { keys, values } => {
                Box::new(keys.iter().zip(values.iter()).filter_map(|(k, v)| match v {
                    Some(AttributeOrProperty::Attribute(v)) => Some((*k, v.as_ref())),
                    _ => None,
                }))
            }
            Self::IndexMap(m) => Box::new(m.iter().filter_map(|(k, v)| match v {
                AttributeOrProperty::Attribute(v) => Some((k.as_ref(), v.as_ref())),
                _ => None,
            })),
        }
    }

    /// Get a mutable reference to the underlying `IndexMap`. Deprecated for
    /// [`Self::get_mut_index_map_unchecked`].
    #[doc(hidden)]
    #[deprecated = "Attribute names are not validated. Use `get_mut_index_map_unchecked` to signal \
                    this properly and validate your modifications."]
    pub fn get_mut_index_map(&mut self) -> &mut IndexMap<AttrValue, AttributeOrProperty> {
        self.get_mut_index_map_unchecked()
    }

    /// Get a mutable reference to the underlying `IndexMap`.
    /// If the attributes are stored in the `Vec` variant, it will be converted.
    /// The caller is responsible to check that inserted attribute names are valid, see
    /// [`Self::is_valid_attr_key`]
    pub fn get_mut_index_map_unchecked(&mut self) -> &mut IndexMap<AttrValue, AttributeOrProperty> {
        macro_rules! unpack {
            () => {
                #[expect(deprecated)]
                match self {
                    Self::IndexMap(m) => Rc::make_mut(m),
                    // SAFETY: unreachable because we set self to the `IndexMap` variant above.
                    _ => unsafe { unreachable_unchecked() },
                }
            };
        }

        #[expect(deprecated)]
        match self {
            Self::IndexMap(m) => Rc::make_mut(m),
            Self::Static(arr) => {
                *self = Self::IndexMap(Rc::new(
                    arr.iter().map(|(k, v)| ((*k).into(), v.clone())).collect(),
                ));
                unpack!()
            }
            Self::Dynamic { keys, values } => {
                *self = Self::from_index_map_unchecked(Rc::new(
                    std::mem::take(values)
                        .iter_mut()
                        .zip(keys.iter())
                        .filter_map(|(v, k)| v.take().map(|v| (AttrValue::from(*k), v)))
                        .collect(),
                ));
                unpack!()
            }
        }
    }
}

impl From<IndexMap<AttrValue, AttrValue>> for Attributes {
    fn from(map: IndexMap<AttrValue, AttrValue>) -> Self {
        let v = map
            .into_iter()
            .map(|(k, v)| (k, AttributeOrProperty::Attribute(v)))
            .collect();
        Self::from_index_map(Rc::new(v))
    }
}

impl From<IndexMap<&'static str, AttrValue>> for Attributes {
    fn from(v: IndexMap<&'static str, AttrValue>) -> Self {
        let v = v
            .into_iter()
            .map(|(k, v)| (AttrValue::Static(k), (AttributeOrProperty::Attribute(v))))
            .collect();
        Self::from_index_map(Rc::new(v))
    }
}

impl From<IndexMap<&'static str, JsValue>> for Attributes {
    fn from(v: IndexMap<&'static str, JsValue>) -> Self {
        let v = v
            .into_iter()
            .map(|(k, v)| (AttrValue::Static(k), (AttributeOrProperty::Property(v))))
            .collect();
        Self::from_index_map(Rc::new(v))
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::from_static(&[])
    }
}
