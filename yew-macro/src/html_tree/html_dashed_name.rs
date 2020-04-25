use crate::{non_capitalized_ascii, Peek};
use boolinator::Boolinator;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::fmt;
use syn::buffer::Cursor;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::Token;

#[derive(PartialEq)]
pub struct HtmlDashedName {
    pub name: Ident,
    pub extended: Vec<(Token![-], Ident)>,
}

impl HtmlDashedName {
    pub fn new(name: Ident) -> Self {
        HtmlDashedName {
            name,
            extended: Vec::new(),
        }
    }
}

impl fmt::Display for HtmlDashedName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for (_, ident) in &self.extended {
            write!(f, "-{}", ident)?;
        }
        Ok(())
    }
}

impl Peek<'_, Self> for HtmlDashedName {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (name, cursor) = cursor.ident()?;
        non_capitalized_ascii(&name.to_string()).as_option()?;

        let mut extended = Vec::new();
        let mut cursor = cursor;
        loop {
            if let Some((punct, p_cursor)) = cursor.punct() {
                if punct.as_char() == '-' {
                    let (ident, i_cursor) = p_cursor.ident()?;
                    cursor = i_cursor;
                    extended.push((Token![-](Span::call_site()), ident));
                    continue;
                }
            }
            break;
        }

        Some((HtmlDashedName { name, extended }, cursor))
    }
}

impl Parse for HtmlDashedName {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.call(Ident::parse_any)?;
        let mut extended = Vec::new();
        while input.peek(Token![-]) {
            extended.push((input.parse::<Token![-]>()?, input.parse::<Ident>()?));
        }

        Ok(HtmlDashedName { name, extended })
    }
}

impl ToTokens for HtmlDashedName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlDashedName { name, extended } = self;
        let dashes = extended.iter().map(|(dash, _)| quote! {#dash});
        let idents = extended.iter().map(|(_, ident)| quote! {#ident});
        let extended = quote! { #(#dashes#idents)* };
        tokens.extend(quote! {#name#extended});
    }
}
