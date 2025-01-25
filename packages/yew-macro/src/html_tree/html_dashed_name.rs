use std::fmt;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{LitStr, Token};

use crate::stringify::Stringify;
use crate::{non_capitalized_ascii, Peek};

#[derive(Clone, PartialEq, Eq)]
pub struct HtmlDashedName {
    pub name: Ident,
    pub extended: Vec<(Token![-], Ident)>,
}

impl HtmlDashedName {
    /// Checks if this name is equal to the provided item (which can be anything implementing
    /// `Into<String>`).
    pub fn eq_ignore_ascii_case<S>(&self, other: S) -> bool
    where
        S: Into<String>,
    {
        let mut s = other.into();
        s.make_ascii_lowercase();
        s == self.to_ascii_lowercase_string()
    }

    pub fn to_ascii_lowercase_string(&self) -> String {
        let mut s = self.to_string();
        s.make_ascii_lowercase();
        s
    }

    pub fn to_lit_str(&self) -> LitStr {
        LitStr::new(&self.to_string(), self.span())
    }
}

impl fmt::Display for HtmlDashedName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for (_, ident) in &self.extended {
            write!(f, "-{ident}")?;
        }
        Ok(())
    }
}

impl Peek<'_, Self> for HtmlDashedName {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (name, cursor) = cursor.ident()?;
        if !non_capitalized_ascii(&name.to_string()) {
            return None;
        }

        let mut extended = Vec::new();
        let mut cursor = cursor;
        loop {
            if let Some((punct, p_cursor)) = cursor.punct() {
                if punct.as_char() == '-' {
                    let (ident, i_cursor) = p_cursor.ident()?;
                    cursor = i_cursor;
                    extended.push((Token![-](Span::mixed_site()), ident));
                    continue;
                }
            }
            break;
        }

        Some((HtmlDashedName { name, extended }, cursor))
    }
}

impl Parse for HtmlDashedName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.call(Ident::parse_any)?;
        let mut extended = Vec::new();
        while input.peek(Token![-]) {
            extended.push((input.parse::<Token![-]>()?, input.call(Ident::parse_any)?));
        }

        Ok(HtmlDashedName { name, extended })
    }
}

impl ToTokens for HtmlDashedName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let HtmlDashedName { name, extended } = self;
        let dashes = extended.iter().map(|(dash, _)| quote! {#dash});
        let idents = extended.iter().map(|(_, ident)| quote! {#ident});
        let extended = quote! { #(#dashes #idents)* };
        tokens.extend(quote! { #name #extended });
    }
}

impl Stringify for HtmlDashedName {
    fn try_into_lit(&self) -> Option<LitStr> {
        Some(self.to_lit_str())
    }

    fn stringify(&self) -> TokenStream {
        self.to_lit_str().stringify()
    }
}

impl From<Ident> for HtmlDashedName {
    fn from(name: Ident) -> Self {
        HtmlDashedName {
            name,
            extended: vec![],
        }
    }
}
