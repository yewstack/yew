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

use indexmap::IndexMap;

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
        Suspense,
    }

    impl Collectable {
        pub fn for_component<T: 'static>() -> Self {
            #[cfg(debug_assertions)]
            let comp_name = std::any::type_name::<T>();
            #[cfg(not(debug_assertions))]
            let comp_name = std::marker::PhantomData;
            Self::Component(comp_name)
        }

        pub fn open_start_mark(&self) -> &'static str {
            match self {
                Self::Component(_) => "<[",
                Self::Suspense => "<?",
            }
        }

        pub fn close_start_mark(&self) -> &'static str {
            match self {
                Self::Component(_) => "</[",
                Self::Suspense => "</?",
            }
        }

        pub fn end_mark(&self) -> &'static str {
            match self {
                Self::Component(_) => "]>",
                Self::Suspense => ">",
            }
        }

        #[cfg(feature = "hydration")]
        pub fn name(&self) -> Cow<'static, str> {
            match self {
                #[cfg(debug_assertions)]
                Self::Component(m) => format!("Component({})", m).into(),
                #[cfg(not(debug_assertions))]
                Self::Component(_) => "Component".into(),
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
                Self::Suspense => {}
            }

            let _ = w.write_str(self.end_mark());
            let _ = w.write_str("-->");
        }
    }
}

/// Defines if the [`Attributes`] is set as element's attribute or property
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ApplyAttributeAs {
    Attribute,
    Property,
}

/// A collection of attributes for an element
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Attributes {
    /// Static list of attributes.
    ///
    /// Allows optimizing comparison to a simple pointer equality check and reducing allocations,
    /// if the attributes do not change on a node.
    Static(&'static [(&'static str, &'static str, ApplyAttributeAs)]),

    /// Static list of attribute keys with possibility to exclude attributes and dynamic attribute
    /// values.
    ///
    /// Allows optimizing comparison to a simple pointer equality check and reducing allocations,
    /// if the attributes keys do not change on a node.
    Dynamic {
        /// Attribute keys. Includes both always set and optional attribute keys.
        keys: &'static [&'static str],

        /// Attribute values. Matches [keys](Attributes::Dynamic::keys). Optional attributes are
        /// designated by setting [None].
        values: Box<[Option<(AttrValue, ApplyAttributeAs)>]>,
    },

    /// IndexMap is used to provide runtime attribute deduplication in cases where the html! macro
    /// was not used to guarantee it.
    IndexMap(IndexMap<AttrValue, (AttrValue, ApplyAttributeAs)>),
}

impl Attributes {
    /// Construct a default Attributes instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Return iterator over attribute key-value pairs.
    /// This function is suboptimal and does not inline well. Avoid on hot paths.
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a> {
        match self {
            Self::Static(arr) => Box::new(arr.iter().map(|(k, v, _)| (*k, *v as &'a str))),
            Self::Dynamic { keys, values } => Box::new(
                keys.iter()
                    .zip(values.iter())
                    .filter_map(|(k, v)| v.as_ref().map(|(v, _)| (*k, v.as_ref()))),
            ),
            Self::IndexMap(m) => Box::new(m.iter().map(|(k, (v, _))| (k.as_ref(), v.as_ref()))),
        }
    }

    /// Get a mutable reference to the underlying `IndexMap`.
    /// If the attributes are stored in the `Vec` variant, it will be converted.
    pub fn get_mut_index_map(&mut self) -> &mut IndexMap<AttrValue, (AttrValue, ApplyAttributeAs)> {
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
                *self = Self::IndexMap(
                    arr.iter()
                        .map(|(k, v, ty)| ((*k).into(), ((*v).into(), *ty)))
                        .collect(),
                );
                unpack!()
            }
            Self::Dynamic { keys, values } => {
                *self = Self::IndexMap(
                    std::mem::take(values)
                        .iter_mut()
                        .zip(keys.iter())
                        .filter_map(|(v, k)| v.take().map(|v| (AttrValue::from(*k), v)))
                        .collect(),
                );
                unpack!()
            }
        }
    }
}

impl From<IndexMap<AttrValue, AttrValue>> for Attributes {
    fn from(map: IndexMap<AttrValue, AttrValue>) -> Self {
        let v = map
            .into_iter()
            .map(|(k, v)| (k, (v, ApplyAttributeAs::Attribute)))
            .collect();
        Self::IndexMap(v)
    }
}

impl From<IndexMap<&'static str, AttrValue>> for Attributes {
    fn from(v: IndexMap<&'static str, AttrValue>) -> Self {
        let v = v
            .into_iter()
            .map(|(k, v)| (AttrValue::Static(k), (v, ApplyAttributeAs::Attribute)))
            .collect();
        Self::IndexMap(v)
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::Static(&[])
    }
}
