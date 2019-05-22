use crate::Peek;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Lit;

pub struct HtmlText {
    text: String,
    literal: Option<Lit>,
}

impl HtmlText {
    pub fn new(text: String) -> Self {
        HtmlText {
            text,
            literal: None,
        }
    }
}

impl Parse for HtmlText {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: Lit = input.parse()?;
        match lit {
            Lit::Str(_) | Lit::Char(_) | Lit::Int(_) | Lit::Float(_) | Lit::Bool(_) => {}
            _ => return Err(syn::Error::new(lit.span(), "unsupported type")),
        };

        Ok(HtmlText {
            text: String::from(""),
            literal: Some(lit),
        })
    }
}

impl Peek<()> for HtmlText {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.literal().map(|_| ()).or_else(|| {
            let (ident, _) = cursor.ident()?;
            match ident.to_string().as_str() {
                "true" | "false" => Some(()),
                _ => None,
            }
        })
    }
}

impl ToTokens for HtmlText {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let literal = self.literal.clone().unwrap();
        tokens.extend(quote! {{
            ::yew_html_common::html_tree::html_text::HtmlText::new(#literal.to_string())
        }});
    }
}
