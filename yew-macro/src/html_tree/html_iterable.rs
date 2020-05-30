use super::ToChildrenTokens;
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
        let new_tokens = quote_spanned! {expr.span()=> {
            (#expr).into_iter().collect::<::yew::virtual_dom::VNode>()
        }};

        tokens.extend(new_tokens);
    }
}

impl ToChildrenTokens for HtmlIterable {
    fn single_child(&self) -> bool {
        false
    }

    fn to_children_tokens(&self, tokens: &mut TokenStream) {
        let Self(expr) = self;
        tokens.extend(quote_spanned! {expr.span()=> {
            (#expr).into_iter().map(|n| ::yew::virtual_dom::VNode::from(n))
        }});
    }
}
