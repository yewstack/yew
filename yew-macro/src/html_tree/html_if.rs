use super::ToNodeIterator;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, ExprIf};

pub struct HtmlIf(ExprIf);

impl PeekValue<()> for HtmlIf {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "if").as_option()
    }
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let expr = match input.parse() {
            Ok(Expr::If(expr)) => expr,
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    "expected a valid `if` expression",
                ))
            }
        };

        Ok(HtmlIf(expr))
    }
}

impl ToTokens for HtmlIf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.0;
        let cond = &expr.cond;
        let then_branch = &expr.then_branch;
        let default_else_branch = Box::new(syn::parse_str::<Expr>("{ html!() }").unwrap());
        let else_branch = &expr
            .else_branch
            .as_ref()
            .map(|(_, expr)| expr)
            .unwrap_or(&default_else_branch);
        let new_tokens = quote_spanned! {expr.span()=>
            if #cond #then_branch else #else_branch
        };

        tokens.extend(new_tokens);
    }
}

impl ToNodeIterator for HtmlIf {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        todo!();
    }
}
