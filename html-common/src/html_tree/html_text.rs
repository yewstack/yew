use crate::Peek;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Lit;

pub struct HtmlText(Lit);

impl Parse for HtmlText {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: Lit = input.parse()?;
        match lit {
            Lit::Str(_) | Lit::Char(_) | Lit::Int(_) | Lit::Float(_) | Lit::Bool(_) => {}
            _ => return Err(syn::Error::new(lit.span(), "unsupported type")),
        };

        Ok(HtmlText(lit))
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
        let literal = &self.0;
        tokens.extend(quote! {::yew::virtual_dom::VNode::VText(
            ::yew::virtual_dom::vtext::VText::new(#literal.to_string())
        )});
    }
}
