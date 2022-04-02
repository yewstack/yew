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
use std::borrow::Cow;
use std::fmt::Formatter;
use std::ops::Deref;
use std::rc::Rc;
use std::{fmt, hint::unreachable_unchecked};

/// Attribute value
#[derive(Debug)]
pub enum AttrValue {
    /// String living for `'static`
    Static(&'static str),
    /// Reference counted string
    Rc(Rc<str>),
}

impl Deref for AttrValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            AttrValue::Static(s) => *s,
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
        AttrValue::Rc(Rc::from(s))
    }
}

impl From<Rc<str>> for AttrValue {
    fn from(s: Rc<str>) -> Self {
        AttrValue::Rc(s)
    }
}

impl From<Cow<'static, str>> for AttrValue {
    fn from(s: Cow<'static, str>) -> Self {
        match s {
            Cow::Borrowed(s) => s.into(),
            Cow::Owned(s) => s.into(),
        }
    }
}

impl Clone for AttrValue {
    fn clone(&self) -> Self {
        match self {
            AttrValue::Static(s) => AttrValue::Static(s),
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
            AttrValue::Rc(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for AttrValue {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for AttrValue {}

impl AttrValue {
    /// Consumes the AttrValue and returns the owned String from the AttrValue whenever possible.
    /// For AttrValue::Rc the <str> is cloned to String in case there are other Rc or Weak pointers to the
    /// same allocation.
    pub fn into_string(self) -> String {
        match self {
            AttrValue::Static(s) => (*s).to_owned(),
            AttrValue::Rc(mut rc) => {
                if let Some(s) = Rc::get_mut(&mut rc) {
                    (*s).to_owned()
                } else {
                    rc.to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_attr_value {
    use super::*;

    #[test]
    fn test_into_string() {
        let av = AttrValue::Static("str");
        assert_eq!(av.into_string(), "str");

        let av = AttrValue::Rc("Rc<str>".into());
        assert_eq!(av.into_string(), "Rc<str>");
    }

    #[test]
    fn test_from_string() {
        let av = AttrValue::from("str");
        assert_eq!(av.into_string(), "str");

        let av = AttrValue::from("String".to_string());
        assert_eq!(av.into_string(), "String");

        let av = AttrValue::from(Cow::from("BorrowedCow"));
        assert_eq!(av.into_string(), "BorrowedCow");
    }

    #[test]
    fn test_equality() {
        // construct 3 AttrValue with same embedded value; expectation is that all are equal
        let a = AttrValue::Static("same");
        let b = AttrValue::Rc("same".into());

        assert_eq!(a, b);

        assert_eq!(a, b);
    }
}

#[cfg(feature = "ssr")] // & feature = "hydration"
mod feat_ssr_hydration {
    /// A collectable.
    ///
    /// This indicates a kind that can be collected from fragment to be processed at a later time
    pub(crate) enum Collectable {
        #[cfg(debug_assertions)]
        Component(&'static str),
        #[cfg(not(debug_assertions))]
        Component,
        Suspense,
    }

    impl Collectable {
        pub fn open_start_mark(&self) -> &'static str {
            match self {
                #[cfg(debug_assertions)]
                Self::Component(_) => "<[",
                #[cfg(not(debug_assertions))]
                Self::Component => "<[",
                Self::Suspense => "<?",
            }
        }
        pub fn close_start_mark(&self) -> &'static str {
            match self {
                #[cfg(debug_assertions)]
                Self::Component(_) => "</[",
                #[cfg(not(debug_assertions))]
                Self::Component => "</[",
                Self::Suspense => "</?",
            }
        }

        pub fn end_mark(&self) -> &'static str {
            match self {
                #[cfg(debug_assertions)]
                Self::Component(_) => "]>",
                #[cfg(not(debug_assertions))]
                Self::Component => "]>",
                Self::Suspense => ">",
            }
        }

        #[cfg(feature = "ssr")]
        pub fn write_open_tag(&self, w: &mut String) {
            w.push_str("<!--");
            w.push_str(self.open_start_mark());

            #[cfg(debug_assertions)]
            match self {
                Self::Component(type_name) => w.push_str(type_name),
                Self::Suspense => {}
            }

            w.push_str(self.end_mark());
            w.push_str("-->");
        }

        #[cfg(feature = "ssr")]
        pub fn write_close_tag(&self, w: &mut String) {
            w.push_str("<!--");
            w.push_str(self.close_start_mark());

            #[cfg(debug_assertions)]
            match self {
                Self::Component(type_name) => w.push_str(type_name),
                Self::Suspense => {}
            }

            w.push_str(self.end_mark());
            w.push_str("-->");
        }
    }
}

#[cfg(feature = "ssr")]
pub(crate) use feat_ssr_hydration::*;

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
