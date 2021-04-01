mod routable_derive;
use routable_derive::{routable_derive_impl, Routable};
use syn::parse_macro_input;

#[proc_macro_derive(Routable, attributes(at))]
pub fn routable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Routable);
    routable_derive_impl(input).into()
}
