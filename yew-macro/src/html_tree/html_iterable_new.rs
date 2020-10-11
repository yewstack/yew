use crate::PeekValue;
use proc_macro2::{Delimiter, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Block, Token};

pub struct HtmlIterableNew(Block);

impl PeekValue<()> for HtmlIterableNew {
    fn peek(cursor: Cursor) -> Option<()> {
        let (_, cursor) = cursor.ident().filter(|(ident, _)| ident == "for")?;
        let (_, _, _cursor) = cursor.group(Delimiter::Brace)?;

        Some(())
    }
}

impl Parse for HtmlIterableNew {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let for_token = input.parse::<Token![for]>()?;

        match input.parse() {
            Ok(expr) => Ok(HtmlIterableNew(expr)),
            Err(err) => {
                if err.to_string().starts_with("unexpected end of input") {
                    Err(syn::Error::new_spanned(
                        for_token,
                        "expected a block after the keyword `for`",
                    ))
                } else {
                    Err(err)
                }
            }
        }
    }
}

impl ToTokens for HtmlIterableNew {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.0;
        let new_tokens = quote_spanned! {expr.span()=>
            #[allow(unused_braces)]
            ::std::iter::Iterator::collect::<::yew::virtual_dom::VNode>(
                ::std::iter::IntoIterator::into_iter(#expr),
            )
        };

        tokens.extend(new_tokens);
    }
}
