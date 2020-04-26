use crate::PeekValue;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Expr;
use syn::Lit;

pub struct HtmlNode(Node);

impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let node = if HtmlNode::peek(input.cursor()).is_some() {
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Str(_) | Lit::Char(_) | Lit::Int(_) | Lit::Float(_) | Lit::Bool(_) => {}
                _ => return Err(syn::Error::new(lit.span(), "unsupported type")),
            }
            Node::Literal(Box::new(lit))
        } else {
            Node::Expression(Box::new(input.parse()?))
        };

        Ok(HtmlNode(node))
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
        self.0.to_tokens(tokens);
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let node_token = match &self {
            Node::Literal(lit) => quote! {#lit},
            Node::Expression(expr) => quote_spanned! {expr.span()=> {#expr} },
        };

        tokens.extend(node_token);
    }
}

enum Node {
    Literal(Box<Lit>),
    Expression(Box<Expr>),
}
