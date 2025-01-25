use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Token};

use super::ToNodeIterator;
use crate::PeekValue;

pub struct HtmlIterable(Expr);

impl PeekValue<()> for HtmlIterable {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "for").then_some(())
    }
}

impl Parse for HtmlIterable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let for_token = input.parse::<Token![for]>()?;

        match input.parse() {
            Ok(expr) => Ok(HtmlIterable(expr)),
            Err(err) => {
                if err.to_string().starts_with("unexpected end of input") {
                    Err(syn::Error::new_spanned(
                        for_token,
                        "expected an expression after the keyword `for`",
                    ))
                } else {
                    Err(err)
                }
            }
        }
    }
}

impl ToTokens for HtmlIterable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.0;
        let new_tokens = quote_spanned! {expr.span()=>
            #[allow(unused_braces)]
            ::std::iter::Iterator::collect::<::yew::virtual_dom::VNode>(::std::iter::IntoIterator::into_iter(#expr))
        };

        tokens.extend(new_tokens);
    }
}

impl ToNodeIterator for HtmlIterable {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        let Self(expr) = self;
        // #expr can return anything that implements IntoIterator<Item=Into<T>>
        // We use a util method to avoid clippy warnings and reduce generated code size
        Some(quote_spanned! {expr.span()=>
            ::yew::utils::into_node_iter(#expr)
        })
    }
}
