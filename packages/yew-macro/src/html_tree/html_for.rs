use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{For, In};
use syn::{Expr, Local, Pat, braced};

use super::HtmlChildrenTree;
use super::html_loop::{emit_loop, parse_loop_body};
use crate::PeekValue;

pub struct HtmlFor {
    pat: Pat,
    iter: Expr,
    let_stmts: Vec<Local>,
    body: HtmlChildrenTree,
    deprecations: TokenStream,
}

impl PeekValue<()> for HtmlFor {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "for").then_some(())
    }
}

impl Parse for HtmlFor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        For::parse(input)?;
        let pat = Pat::parse_single(input)?;
        In::parse(input)?;
        let iter = Expr::parse_without_eager_brace(input)?;

        let body_stream;
        braced!(body_stream in input);

        let (let_stmts, body, deprecations) = parse_loop_body(&body_stream, "for")?;

        Ok(Self {
            pat,
            iter,
            let_stmts,
            body,
            deprecations,
        })
    }
}

impl ToTokens for HtmlFor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            pat,
            iter,
            let_stmts,
            body,
            deprecations,
        } = self;
        let header = quote!(for #pat in #iter);
        tokens.extend(emit_loop(
            header,
            iter.span(),
            let_stmts,
            body,
            deprecations,
        ));
    }
}
