use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

/// List of HTML classes.
pub struct Classes(Punctuated<Expr, Token![,]>);

impl Parse for Classes {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse_terminated(Expr::parse).map(Self)
    }
}

impl ToTokens for Classes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let n = self.0.len();
        let classes = self.0.iter();
        let new_tokens = quote! {
            let mut __yew_classes = ::yew::virtual_dom::Classes::with_capacity(#n);
            #(__yew_classes.push(#classes);)*
            __yew_classes
        };

        tokens.extend(quote! {{
            #[allow(clippy::useless_conversion, unused_braces)]
            #new_tokens
        }});
    }
}
