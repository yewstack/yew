use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{Expr, Lit};

use super::ToNodeIterator;
use crate::stringify::Stringify;
use crate::PeekValue;

pub enum HtmlNode {
    Literal(Box<Lit>),
    Expression(Box<Expr>),
}

impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let node = if HtmlNode::peek(input.cursor()).is_some() {
            let lit: Lit = input.parse()?;
            if matches!(lit, Lit::ByteStr(_) | Lit::Byte(_) | Lit::Verbatim(_)) {
                return Err(syn::Error::new(lit.span(), "unsupported type"));
            }
            HtmlNode::Literal(Box::new(lit))
        } else {
            HtmlNode::Expression(Box::new(input.parse()?))
        };

        Ok(node)
    }
}

impl PeekValue<()> for HtmlNode {
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match &self {
            HtmlNode::Literal(lit) => {
                let sr = lit.stringify();
                quote_spanned! {lit.span()=> ::yew::virtual_dom::VText::new(#sr) }
            }
            HtmlNode::Expression(expr) => quote! {#expr},
        });
    }
}

impl ToNodeIterator for HtmlNode {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        match self {
            HtmlNode::Literal(_) => None,
            HtmlNode::Expression(expr) => {
                // NodeSeq turns both Into<T> and Vec<Into<T>> into IntoIterator<Item = T>
                Some(quote_spanned! {expr.span().resolved_at(Span::call_site())=>
                    ::std::convert::Into::<::yew::utils::NodeSeq<_, _>>::into(#expr)
                })
            }
        }
    }
}
