mod routable_derive;
use routable_derive::{routable_derive_impl, Routable};
use syn::parse_macro_input;

/// Derive macro used to mark an enum as Routable.
///
/// This macro can only be used on enums. Every varient of the macro needs to be marked
/// with the `at` attribute to specify the URL of the route. It generates an implementation of
///  `yew_router::Routable` trait and `const`s for the routes passed which are used with `Route`
/// component.
///
/// The type must also implement `Default`. This implementation will be used if none of the routes
/// match the URL. If there is at least one "wildcard" variant in the enum that matches any route
/// then the `Default` implementation will never be used, but it must still be present to satisfy
/// the required bounds.
///
/// # Example
///
/// ```
/// # use yew_router::Routable;
/// #[derive(Debug, Clone, Copy, PartialEq, Routable)]
/// enum Routes {
///     #[at("/")]
///     Home,
///     #[at("/secure")]
///     Secure,
///     #[at("/404")]
///     NotFound,
/// }
///
/// impl Default for Routes {
///     fn default() -> Self {
///         // The browser will be redirected to this route if the URL doesn't match any of
///         // the patterns above.
///         Self::NotFound
///     }
/// }
/// ```
#[proc_macro_derive(Routable, attributes(at, bind))]
pub fn routable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Routable);
    routable_derive_impl(input).into()
}
