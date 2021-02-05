use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod switch;

/// Implements the `Switch` trait based on attributes present on the struct or enum variants.
///
/// If deriving an enum, each variant should have a `#[at = ""]` attribute,
/// and if deriving a struct, the struct itself should have a `#[at = ""]` attribute.
///
/// Inside the `""` you should put your **route matcher string**.
/// At its simplest, the route matcher string will create your variant/struct if it exactly matches the browser's route.
/// If the route in the url bar is `http://yoursite.com/some/route` and your route matcher string
/// for an enum variant is `/some/route`, then that variant will be created when `switch()` is called with the route.
///
/// But the route matcher has other capabilities.
/// If you want to capture data from the route matcher string, for example, extract an id or user name from the route,
/// you can use `{field_name}` to capture data from the route.
/// For example, `#[at = "/route/{id}"]` will capture the content after "/route/",
/// and if the associated variant is defined as `Route{id: usize}`, then the string that was captured will be
/// transformed into a `usize`.
/// If the conversion fails, then the match won't succeed and the next variant will be tried instead.
///
/// There are also `{*:field_name}` and `{3:field_name}` types of capture sections that will capture
/// _everything_, and the next 3 path sections respectively.
/// `{1:field_name}` is the same as `{field_name}`.
///
/// Tuple-structs and Tuple-enum-variants are also supported.
/// If you don't want to specify keys that don't correspond to any specific field,
/// `{}`, `{*}`, and `{4}` also denote valid capture sections when used on structs and variants without named fields.
/// In datastructures without field names, the captures will be assigned in order - left to right.
///
/// # Note
/// It should be mentioned that the derived function for matching will try enum variants in order,
/// from top to bottom, and that the whole route doesn't need to be matched by the route
/// matcher string in order for the match to succeed.
/// What is meant by this is that `[to = "/"]` will match "/", but also "/anything/else",
/// because as soon as the "/" is satisfied, that is considered a match.
///
/// This can be mitigated by specifying a `!` at the end of your route to inform the matcher that if
/// any characters are left after matching the route matcher string, the match should fail.
/// This means that `[to = "/!"]` will match "/" and _only_ "/".
///
/// -----
/// There are other attributes as well.
/// `#[rest]`, `#[rest="field_name"]` and `#[end]` attributes exist as well.
/// `#[rest]` and `#[rest="field_name"]` are equivalent to `{*}` and `{*:field_name}` respectively.
/// `#[end]` is equivalent to `!`.
/// The `#[rest]` attributes are good if you just want to delegate the whole matching of a variant to a specific
/// wrapped struct or enum that also implements `Switch`.
///
/// ------
/// # Example
/// ```
/// use yew_router::Switch;
///
/// #[derive(Switch, Clone)]
/// enum AppRoute {
///     #[at = "/some/simple/route"]
///     SomeSimpleRoute,
///     #[at = "/capture/{}"]
///     Capture(String),
///     #[at = "/named/capture/{name}"]
///     NamedCapture { name: String },
///     #[at = "/convert/{id}"]
///     Convert { id: usize },
///     #[rest] // shorthand for #[at = "{*}"]
///     Inner(InnerRoute),
/// }
///
/// #[derive(Switch, Clone)]
/// #[at = "/inner/route/{first}/{second}"]
/// struct InnerRoute {
///     first: String,
///     second: String,
/// }
/// ```
/// Check out the examples directory in the repository to see some more usages of the routing syntax.
#[proc_macro_derive(Switch, attributes(to, at, rest, end))]
pub fn switch(tokens: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(tokens as DeriveInput);

    crate::switch::switch_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn to(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn at(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn rest(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn end(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}
