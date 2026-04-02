use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn comp(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    if matches!(func.sig.output, ReturnType::Default) {
        func.sig.output = syn::parse_quote!(-> ::yew::Html);
    }

    let block = &func.block;
    let mgr_ident = Ident::new("__stylist_style_manager__", Span::mixed_site());

    *func.block = syn::parse_quote!({
        let #mgr_ident =
            ::yew::functional::use_context::<::stylist::manager::StyleManager>()
                .unwrap_or_default();
        #[allow(unused_macros)]
        macro_rules! css {
            ($( $args:tt )*) => {
                ::stylist::css!($($args)*).with_manager({
                    #[allow(clippy::redundant_clone)]
                    #mgr_ident.clone()
                })
            }
        }
        #block
    });

    let attrs = &func.attrs;
    let vis = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    quote! {
        #[::yew_autoprops::autoprops]
        #[::yew::functional::function_component]
        #[allow(unused_variables)]
        #(#attrs)*
        #vis #sig #block
    }
    .into()
}
