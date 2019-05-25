use crate::Peek;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Lit;

pub struct HtmlNode(TokenStream);

impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let stream = if HtmlNode::peek(input.cursor()).is_some() {
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Str(_) | Lit::Char(_) | Lit::Int(_) | Lit::Float(_) | Lit::Bool(_) => {}
                _ => return Err(syn::Error::new(lit.span(), "unsupported type")),
            };
            let mut stream = TokenStream::new();
            stream.extend(quote! {#lit});
            stream
        } else {
            input.parse()?
        };

        Ok(HtmlNode(stream))
    }
}

impl Peek<()> for HtmlNode {
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

impl ToTokens for HtmlNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let stream = &self.0;
        tokens.extend(quote! { $crate::virtual_dom::VNode::from({#stream}) });
    }
}
