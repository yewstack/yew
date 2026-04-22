use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::While;
use syn::{Expr, Stmt, braced};

use super::HtmlChildrenTree;
use super::html_loop::{emit_loop, parse_loop_body};
use crate::PeekValue;

pub struct HtmlWhile {
    cond: Box<Expr>,
    stmts: Vec<Stmt>,
    body: HtmlChildrenTree,
    deprecations: TokenStream,
}

impl PeekValue<()> for HtmlWhile {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "while").then_some(())
    }
}

impl Parse for HtmlWhile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        While::parse(input)?;
        let cond = Box::new(input.call(Expr::parse_without_eager_brace)?);
        match &*cond {
            Expr::Block(syn::ExprBlock { block, .. }) if block.stmts.is_empty() => {
                return Err(syn::Error::new(
                    cond.span(),
                    "missing condition for `while` expression",
                ));
            }
            _ => {}
        }
        if input.is_empty() {
            return Err(syn::Error::new(
                cond.span(),
                "this `while` expression has a condition, but no block",
            ));
        }

        let body_stream;
        braced!(body_stream in input);

        let (stmts, body, deprecations) = parse_loop_body(&body_stream, "while")?;

        Ok(Self {
            cond,
            stmts,
            body,
            deprecations,
        })
    }
}

impl ToTokens for HtmlWhile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            cond,
            stmts,
            body,
            deprecations,
        } = self;
        let header = quote!(while #cond);
        tokens.extend(emit_loop(header, stmts, body, deprecations));
    }
}
