use super::Apply;
use crate::dom_bundle::BSubtree;
use crate::virtual_dom::vtag::{InputFields, Value};
use crate::virtual_dom::Attributes;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::iter;
use std::ops::Deref;
use web_sys::{Element, HtmlInputElement as InputElement, HtmlTextAreaElement as TextAreaElement};

impl<T: AccessValue> Apply for Value<T> {
    type Element = T;
    type Bundle = Self;

    fn apply(self, _root: &BSubtree, el: &Self::Element) -> Self {
        if let Some(v) = self.deref() {
            el.set_value(v);
        }
        self
    }

    fn apply_diff(self, _root: &BSubtree, el: &Self::Element, bundle: &mut Self) {
        match (self.deref(), (*bundle).deref()) {
            (Some(new), Some(_)) => {
                // Refresh value from the DOM. It might have changed.
                if new.as_ref() != el.value() {
                    el.set_value(new);
                }
            }
            (Some(new), None) => el.set_value(new),
            (None, Some(_)) => el.set_value(""),
            (None, None) => (),
        }
    }
}

macro_rules! impl_access_value {
    ($( $type:ty )*) => {
        $(
            impl AccessValue for $type {
                #[inline]
                fn value(&self) -> String {
                    <$type>::value(&self)
                }

                #[inline]
                fn set_value(&self, v: &str) {
                    <$type>::set_value(&self, v)
                }
            }
        )*
    };
}
impl_access_value! {InputElement TextAreaElement}

/// Able to have its value read or set
pub(super) trait AccessValue {
    fn value(&self) -> String;
    fn set_value(&self, v: &str);
}

impl Apply for InputFields {
    type Element = InputElement;
    type Bundle = Self;

    fn apply(mut self, root: &BSubtree, el: &Self::Element) -> Self {
        // IMPORTANT! This parameter has to be set every time
        // to prevent strange behaviour in the browser when the DOM changes
        el.set_checked(self.checked);

        self.value = self.value.apply(root, el);
        self
    }

    fn apply_diff(self, root: &BSubtree, el: &Self::Element, bundle: &mut Self) {
        // IMPORTANT! This parameter has to be set every time
        // to prevent strange behaviour in the browser when the DOM changes
        el.set_checked(self.checked);

        self.value.apply_diff(root, el, &mut bundle.value);
    }
}

impl Attributes {
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
    type Bundle = Self;

    fn apply(self, _root: &BSubtree, el: &Element) -> Self {
        match &self {
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
        self
    }

    fn apply_diff(self, _root: &BSubtree, el: &Element, bundle: &mut Self) {
        #[inline]
        fn ptr_eq<T>(a: &[T], b: &[T]) -> bool {
            std::ptr::eq(a, b)
        }

        let ancestor = std::mem::replace(bundle, self);
        let bundle = &*bundle; // reborrow it immutably from here
        match (bundle, ancestor) {
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
            (Self::IndexMap(new), Self::IndexMap(ref old)) => {
                let new_iter = new.iter().map(|(k, v)| (*k, v.as_ref()));
                Self::apply_diff_index_maps(el, new_iter, new, old);
            }
            // Cold path. Happens only with conditional swapping and reordering of `VTag`s with the
            // same tag and no keys.
            (new, ref ancestor) => {
                Self::apply_diff_as_maps(el, new, ancestor);
            }
        }
    }
}
