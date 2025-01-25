//! This module contains useful utilities to get information about the current document.

use std::marker::PhantomData;

use yew::html::ChildrenRenderer;

/// Map `IntoIterator<Item = Into<T>>` to `Iterator<Item = T>`
pub fn into_node_iter<IT, T, R>(it: IT) -> impl Iterator<Item = R>
where
    IT: IntoIterator<Item = T>,
    T: Into<R>,
{
    it.into_iter().map(|n| n.into())
}

/// A special type necessary for flattening components returned from nested html macros.
#[derive(Debug)]
pub struct NodeSeq<IN, OUT>(Vec<OUT>, PhantomData<IN>);

impl<IN: Into<OUT>, OUT> From<IN> for NodeSeq<IN, OUT> {
    fn from(val: IN) -> Self {
        Self(vec![val.into()], PhantomData)
    }
}

impl<IN: Into<OUT>, OUT> From<Option<IN>> for NodeSeq<IN, OUT> {
    fn from(val: Option<IN>) -> Self {
        Self(val.map(|s| vec![s.into()]).unwrap_or_default(), PhantomData)
    }
}

impl<IN: Into<OUT>, OUT> From<Vec<IN>> for NodeSeq<IN, OUT> {
    fn from(val: Vec<IN>) -> Self {
        Self(val.into_iter().map(|x| x.into()).collect(), PhantomData)
    }
}

impl<IN: Into<OUT> + Clone, OUT> From<&ChildrenRenderer<IN>> for NodeSeq<IN, OUT> {
    fn from(val: &ChildrenRenderer<IN>) -> Self {
        Self(val.iter().map(|x| x.into()).collect(), PhantomData)
    }
}

impl<IN, OUT> IntoIterator for NodeSeq<IN, OUT> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = OUT;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Hack to force type mismatch compile errors in yew-macro.
// TODO: replace with `compile_error!`, when `type_name_of_val` is stabilised (https://github.com/rust-lang/rust/issues/66359).
#[doc(hidden)]
pub fn __ensure_type<T>(_: T) {}

/// Print the [web_sys::Node]'s contents as a string for debugging purposes
pub fn print_node(n: &web_sys::Node) -> String {
    use wasm_bindgen::JsCast;

    match n.dyn_ref::<web_sys::Element>() {
        Some(el) => el.outer_html(),
        None => n.text_content().unwrap_or_default(),
    }
}

// NOTE: replace this by Rc::unwrap_or_clone() when it becomes stable
pub(crate) trait RcExt<T: Clone> {
    fn unwrap_or_clone(this: Self) -> T;
}

impl<T: Clone> RcExt<T> for std::rc::Rc<T> {
    fn unwrap_or_clone(this: Self) -> T {
        std::rc::Rc::try_unwrap(this).unwrap_or_else(|rc| (*rc).clone())
    }
}
