//! Lints to catch possible misuse of the `html!` macro use. At the moment these are mostly focused
//! on accessibility.

use proc_macro_error::emit_warning;
use syn::spanned::Spanned;

use super::html_element::{HtmlElement, TagName};
use super::HtmlTree;
use crate::props::{ElementProps, Prop};

/// Lints HTML elements to check if they are well formed. If the element is not well-formed, then
/// use `proc-macro-error` (and the `emit_warning!` macro) to produce a warning. At present, these
/// are only emitted on nightly.
pub trait Lint {
    fn lint(element: &HtmlElement);
}

/// Applies all the lints to the HTML tree.
pub fn lint_all(tree: &HtmlTree) {
    lint::<AHrefLint>(tree);
    lint::<ImgAltLint>(tree);
}

/// Applies a specific lint to the HTML tree.
pub fn lint<L>(tree: &HtmlTree)
where
    L: Lint,
{
    #[cfg(not(yew_lints))]
    let _ = tree;
    #[cfg(yew_lints)]
    match tree {
        HtmlTree::List(list) => {
            for child in &list.children.0 {
                lint::<L>(child)
            }
        }
        HtmlTree::Element(el) => L::lint(el),
        _ => {}
    }
}

/// Retrieves an attribute from an element and returns a reference valid for the lifetime of the
/// element (if that attribute can be found on the prop).
///
/// Attribute names are lowercased before being compared (so pass "href" for `name` and not "HREF").
fn get_attribute<'a>(props: &'a ElementProps, name: &str) -> Option<&'a Prop> {
    props
        .attributes
        .iter()
        .find(|item| item.label.eq_ignore_ascii_case(name))
}

/// Lints to check if anchor (`<a>`) tags have valid `href` attributes defined.
pub struct AHrefLint;

impl Lint for AHrefLint {
    fn lint(element: &HtmlElement) {
        if let TagName::Lit(ref tag_name) = element.name {
            if !tag_name.eq_ignore_ascii_case("a") {
                return;
            };
            if let Some(prop) = get_attribute(&element.props, "href") {
                if let syn::Expr::Lit(lit) = &prop.value {
                    if let syn::Lit::Str(href) = &lit.lit {
                        let href_value = href.value();
                        match href_value.as_ref() {
                            "#" | "javascript:void(0)" => emit_warning!(
                                lit.span(),
                                format!("'{}' is not a suitable value for the `href` attribute. \
                                        Without a meaningful attribute assistive technologies \
                                        will struggle to understand your webpage. \
                                        https://developer.mozilla.org/en-US/docs/Learn/Accessibility/HTML#onclick_events"
                            ,href_value)),
                            _ => {}

                        }
                    }
                };
            } else {
                emit_warning!(
                    quote::quote! {#tag_name}.span(),
                    "All `<a>` elements should have a `href` attribute. This makes it possible \
                        for assistive technologies to correctly interpret what your links point to. \
                        https://developer.mozilla.org/en-US/docs/Learn/Accessibility/HTML#more_on_links"
                )
            }
        }
    }
}

/// Checks to make sure that images have `alt` attributes defined.
pub struct ImgAltLint;

impl Lint for ImgAltLint {
    fn lint(element: &HtmlElement) {
        if let super::html_element::TagName::Lit(ref tag_name) = element.name {
            if !tag_name.eq_ignore_ascii_case("img") {
                return;
            };
            if get_attribute(&element.props, "alt").is_none() {
                emit_warning!(
                    quote::quote! {#tag_name}.span(),
                    "All `<img>` tags should have an `alt` attribute which provides a \
                     human-readable description "
                )
            }
        }
    }
}
