use super::ToNodeIterator;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, Token};

pub struct HtmlIterable(Expr);

impl PeekValue<()> for HtmlIterable {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "for").as_option()
    }
}

impl Parse for HtmlIterable {
    fn parse(input: ParseStream) -> ParseResult<Self> {
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
            ::std::iter::Iterator::collect::<::yew::virtual_dom::VNode>(::std::iter::IntoIterator::into_iter(#expr))
        };

        tokens.extend(new_tokens);
    }
}

impl ToNodeIterator for HtmlIterable {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        let Self(expr) = self;
        // #expr can return anything that implements IntoIterator<Item=Into<T>>
        // so we generate some extra code to turn it into IntoIterator<Item=T>
        Some(quote_spanned! {expr.span()=>
            ::std::iter::Iterator::map(::std::iter::IntoIterator::into_iter(#expr), |n| n.into())
        })
    }
}
