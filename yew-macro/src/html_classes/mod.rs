use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Expr;

/// List of HTML classes.
pub struct HtmlClasses(Punctuated::<Expr, Comma>);

impl Parse for HtmlClasses {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(HtmlClasses(Punctuated::<Expr, Comma>::parse_terminated(input)?))
    }
}

impl ToTokens for HtmlClasses {
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
