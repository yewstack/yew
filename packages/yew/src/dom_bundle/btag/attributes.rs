use std::collections::HashMap;
use std::ops::Deref;

use indexmap::IndexMap;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlInputElement as InputElement, HtmlTextAreaElement as TextAreaElement};
use yew::AttrValue;

use super::Apply;
use crate::dom_bundle::BSubtree;
use crate::virtual_dom::vtag::{InputFields, Value};
use crate::virtual_dom::{ApplyAttributeAs, Attributes};

impl<T: AccessValue> Apply for Value<T> {
    type Bundle = Self;
    type Element = T;

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
    type Bundle = Self;
    type Element = InputElement;

    fn apply(mut self, root: &BSubtree, el: &Self::Element) -> Self {
        // IMPORTANT! This parameter has to be set every time it's explicitly given
        // to prevent strange behaviour in the browser when the DOM changes
        if let Some(checked) = self.checked {
            el.set_checked(checked);
        }

        self.value = self.value.apply(root, el);
        self
    }

    fn apply_diff(self, root: &BSubtree, el: &Self::Element, bundle: &mut Self) {
        // IMPORTANT! This parameter has to be set every time it's explicitly given
        // to prevent strange behaviour in the browser when the DOM changes
        if let Some(checked) = self.checked {
            el.set_checked(checked);
        }

        self.value.apply_diff(root, el, &mut bundle.value);
    }
}

impl Attributes {
    #[cold]
    fn apply_diff_index_maps(
        el: &Element,
        new: &IndexMap<AttrValue, (AttrValue, ApplyAttributeAs)>,
        old: &IndexMap<AttrValue, (AttrValue, ApplyAttributeAs)>,
    ) {
        for (key, value) in new.iter() {
            match old.get(key) {
                Some(old_value) => {
                    if value != old_value {
                        Self::set(el, key, value.0.as_ref(), value.1);
                    }
                }
                None => Self::set(el, key, value.0.as_ref(), value.1),
            }
        }

        for (key, (_, apply_as)) in old.iter() {
            if !new.contains_key(key) {
                Self::remove(el, key, *apply_as);
            }
        }
    }

    /// Convert [Attributes] pair to [HashMap]s and patch changes to `el`.
    /// Works with any [Attributes] variants.
    #[cold]
    fn apply_diff_as_maps<'a>(el: &Element, new: &'a Self, old: &'a Self) {
        fn collect(src: &Attributes) -> HashMap<&str, (&str, ApplyAttributeAs)> {
            use Attributes::*;

            match src {
                Static(arr) => (*arr)
                    .iter()
                    .map(|(k, v, apply_as)| (*k, (*v, *apply_as)))
                    .collect(),
                Dynamic { keys, values } => keys
                    .iter()
                    .zip(values.iter())
                    .filter_map(|(k, v)| {
                        v.as_ref()
                            .map(|(v, apply_as)| (*k, (v.as_ref(), *apply_as)))
                    })
                    .collect(),
                IndexMap(m) => m
                    .iter()
                    .map(|(k, (v, apply_as))| (k.as_ref(), (v.as_ref(), *apply_as)))
                    .collect(),
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
                Self::set(el, k, new.0, new.1);
            }
        }

        // Remove missing
        for (k, (_, apply_as)) in old.iter() {
            if !new.contains_key(k) {
                Self::remove(el, k, *apply_as);
            }
        }
    }

    fn set(el: &Element, key: &str, value: &str, apply_as: ApplyAttributeAs) {
        match apply_as {
            ApplyAttributeAs::Attribute => {
                el.set_attribute(key, value).expect("invalid attribute key")
            }
            ApplyAttributeAs::Property => {
                let key = JsValue::from_str(key);
                let value = JsValue::from_str(value);
                js_sys::Reflect::set(el.as_ref(), &key, &value).expect("could not set property");
            }
        }
    }

    fn remove(el: &Element, key: &str, apply_as: ApplyAttributeAs) {
        match apply_as {
            ApplyAttributeAs::Attribute => el
                .remove_attribute(key)
                .expect("could not remove attribute"),
            ApplyAttributeAs::Property => {
                let key = JsValue::from_str(key);
                js_sys::Reflect::set(el.as_ref(), &key, &JsValue::UNDEFINED)
                    .expect("could not remove property");
            }
        }
    }
}

impl Apply for Attributes {
    type Bundle = Self;
    type Element = Element;

    fn apply(self, _root: &BSubtree, el: &Element) -> Self {
        match &self {
            Self::Static(arr) => {
                for (k, v, apply_as) in arr.iter() {
                    Self::set(el, k, v, *apply_as);
                }
            }
            Self::Dynamic { keys, values } => {
                for (k, v) in keys.iter().zip(values.iter()) {
                    if let Some((v, apply_as)) = v {
                        Self::set(el, k, v, *apply_as)
                    }
                }
            }
            Self::IndexMap(m) => {
                for (k, (v, apply_as)) in m.iter() {
                    Self::set(el, k, v, *apply_as)
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
                assert_eq!(new_k.len(), new_v.len());
                assert_eq!(new_k.len(), old_v.len());
                for i in 0..new_k.len() {
                    macro_rules! key {
                        () => {
                            unsafe { new_k.get_unchecked(i) }
                        };
                    }
                    macro_rules! set {
                        ($new:expr) => {
                            Self::set(el, key!(), $new.0.as_ref(), $new.1)
                        };
                    }

                    match unsafe { (new_v.get_unchecked(i), old_v.get_unchecked(i)) } {
                        (Some(new), Some(old)) => {
                            if new != old {
                                set!(new);
                            }
                        }
                        (Some(new), None) => set!(new),
                        (None, Some(old)) => {
                            Self::remove(el, key!(), old.1);
                        }
                        (None, None) => (),
                    }
                }
            }
            // For VTag's constructed outside the html! macro
            (Self::IndexMap(new), Self::IndexMap(ref old)) => {
                Self::apply_diff_index_maps(el, new, old);
            }
            // Cold path. Happens only with conditional swapping and reordering of `VTag`s with the
            // same tag and no keys.
            (new, ref ancestor) => {
                Self::apply_diff_as_maps(el, new, ancestor);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use gloo::utils::document;
    use js_sys::Reflect;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;
    use crate::{function_component, html, Html};

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_element() -> (Element, BSubtree) {
        let element = document()
            .create_element("a")
            .expect("failed to create element");
        let btree = BSubtree::create_root(&element);
        (element, btree)
    }

    #[test]
    fn properties_are_set() {
        let attrs = Attributes::Static(&[
            ("href", "https://example.com/", ApplyAttributeAs::Property),
            ("alt", "somewhere", ApplyAttributeAs::Property),
        ]);
        let (element, btree) = create_element();
        attrs.apply(&btree, &element);
        assert_eq!(
            Reflect::get(element.as_ref(), &JsValue::from_str("href"))
                .expect("no href")
                .as_string()
                .expect("not a string"),
            "https://example.com/",
            "property `href` not set properly"
        );
        assert_eq!(
            Reflect::get(element.as_ref(), &JsValue::from_str("alt"))
                .expect("no alt")
                .as_string()
                .expect("not a string"),
            "somewhere",
            "property `alt` not set properly"
        );
    }

    #[test]
    fn respects_apply_as() {
        let attrs = Attributes::Static(&[
            ("href", "https://example.com/", ApplyAttributeAs::Attribute),
            ("alt", "somewhere", ApplyAttributeAs::Property),
        ]);
        let (element, btree) = create_element();
        attrs.apply(&btree, &element);
        assert_eq!(
            element.outer_html(),
            "<a href=\"https://example.com/\"></a>",
            "should be set as attribute"
        );
        assert_eq!(
            Reflect::get(element.as_ref(), &JsValue::from_str("alt"))
                .expect("no alt")
                .as_string()
                .expect("not a string"),
            "somewhere",
            "property `alt` not set properly"
        );
    }

    #[test]
    fn class_is_always_attrs() {
        let attrs = Attributes::Static(&[("class", "thing", ApplyAttributeAs::Attribute)]);

        let (element, btree) = create_element();
        attrs.apply(&btree, &element);
        assert_eq!(element.get_attribute("class").unwrap(), "thing");
    }

    #[test]
    async fn macro_syntax_works() {
        #[function_component]
        fn Comp() -> Html {
            html! { <a href="https://example.com/" ~alt="abc" /> }
        }

        let output = gloo::utils::document().get_element_by_id("output").unwrap();
        yew::Renderer::<Comp>::with_root(output.clone()).render();

        gloo::timers::future::sleep(Duration::from_secs(1)).await;
        let element = output.query_selector("a").unwrap().unwrap();
        assert_eq!(
            element.get_attribute("href").unwrap(),
            "https://example.com/"
        );

        assert_eq!(
            Reflect::get(element.as_ref(), &JsValue::from_str("alt"))
                .expect("no alt")
                .as_string()
                .expect("not a string"),
            "abc",
            "property `alt` not set properly"
        );
    }
}
