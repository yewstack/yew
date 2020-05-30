use super::ToChildrenTokens;
use crate::PeekValue;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Expr;
use syn::Lit;

pub enum HtmlNode {
    Literal(Box<Lit>),
    Expression(Box<Expr>),
}

impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let node = if HtmlNode::peek(input.cursor()).is_some() {
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Str(_) | Lit::Char(_) | Lit::Int(_) | Lit::Float(_) | Lit::Bool(_) => {}
                _ => return Err(syn::Error::new(lit.span(), "unsupported type")),
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
            HtmlNode::Literal(lit) => quote! {#lit},
            HtmlNode::Expression(expr) => quote_spanned! {expr.span()=> {#expr}},
        });
    }
}

impl ToChildrenTokens for HtmlNode {
    fn single_child(&self) -> bool {
        matches!(self, HtmlNode::Literal(_))
    }

    fn to_children_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match &self {
            HtmlNode::Literal(lit) => quote_spanned! {lit.span()=> ::std::iter::once(#lit)},
            HtmlNode::Expression(expr) => {
                quote_spanned! {expr.span()=> {::yew::utils::NodeSeq::from(#expr)} }
            }
        });
    }
}
