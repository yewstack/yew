//! Lints to catch possible misuse of the `html!` macro use. At the moment these are mostly focused
//! on accessibility.

use crate::html_tree::html_component::HtmlComponent;
use proc_macro_error::emit_warning;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Type;

use crate::props::{ComponentProps, Prop};

use super::HtmlTree;

/// Lints HTML elements to check if they are well formed. If the element is not well-formed, then
/// use `proc-macro-error` (and the `emit_warning!` macro) to produce a warning. At present, these
/// are only emitted on nightly.
pub trait Lint {
    fn lint(element: &HtmlComponent);
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
    match tree {
        HtmlTree::List(list) => {
            for child in &list.children.0 {
                lint::<L>(child)
            }
        }
        HtmlTree::Component(el) => L::lint(el),
        _ => {}
    }
}

/// Retrieves an attribute from an element and returns a reference valid for the lifetime of the
/// element (if that attribute can be found on the prop).
///
/// Attribute names are lowercased before being compared (so pass "href" for `name` and not "HREF").
fn get_attribute<'a>(props: &'a ComponentProps, name: &str) -> Option<&'a Prop<true>> {
    props
        .props
        .iter()
        .find(|item| item.label.to_string().eq_ignore_ascii_case(name))
}

fn get_component_name(component: &HtmlComponent) -> String {
    match &component.ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .as_ref()
            .map(|it| it.ident.to_token_stream().to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

/// Lints to check if anchor (`<a>`) tags have valid `href` attributes defined.
pub struct AHrefLint;

impl Lint for AHrefLint {
    fn lint(element: &HtmlComponent) {
        let name = &element.ty;
        if !get_component_name(element).eq_ignore_ascii_case("a") {
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
                quote::quote! {#name}.span(),
                "All `<a>` elements should have a `href` attribute. This makes it possible \
                    for assistive technologies to correctly interpret what your links point to. \
                    https://developer.mozilla.org/en-US/docs/Learn/Accessibility/HTML#more_on_links"
            )
        }
    }
}

/// Checks to make sure that images have `alt` attributes defined.
pub struct ImgAltLint;

impl Lint for ImgAltLint {
    fn lint(element: &HtmlComponent) {
        let name = &element.ty;
        if !get_component_name(element).eq_ignore_ascii_case("img") {
            return;
        };
        if get_attribute(&element.props, "alt").is_none() {
            emit_warning!(
                quote::quote! {#name}.span(),
                "All `<img>` tags should have an `alt` attribute which provides a \
                    human-readable description "
            )
        }
    }
}
