#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::{TokenStream, Delimiter};
use quote::{quote_spanned, ToTokens, quote};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, ExprIf, Token, braced, token, Block};
use super::HtmlRoot;
use super::HtmlTree;

pub struct HtmlIf {
    if_token: Token![if],
    cond: Box<Expr>,
    then_branch: HtmlBranch,
    else_branch: Option<(Token![else], Box<HtmlBranchOrIf>)>,
}

impl PeekValue<()> for HtmlIf {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "if").as_option()
    }
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let if_token = input.parse()?;
        let cond = input.parse()?;
        let then_branch = input.parse()?;
        let else_branch = input.parse::<Token![else]>()
            .ok()
            .map(|else_token| input.parse().map(|branch| (else_token, branch)))
            .transpose()?;

        Ok(HtmlIf {
            if_token,
            cond,
            then_branch,
            else_branch,
        })
    }
}

impl ToTokens for HtmlIf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        /*
        let expr = &self.0;
        let cond = &expr.cond;
        let then_branch = &expr.then_branch;
        let default_else_branch = Box::new(syn::parse_str::<Expr>("{html!()}").unwrap());
        let else_branch = &expr
            .else_branch
            .as_ref()
            .map(|(_, expr)| expr)
            .unwrap_or(&default_else_branch);
        let new_tokens = quote_spanned! {expr.span()=>
            if #cond #then_branch else #else_branch
        };
        */
        let HtmlIf {
            if_token,
            cond,
            then_branch,
            else_branch,
        } = self;
        let default_else_branch = syn::parse_str("{}").unwrap();
        let else_branch = else_branch
            .as_ref()
            .map(|(_, branch)| branch)
            .unwrap_or(&default_else_branch);
        let new_tokens = quote_spanned! {if_token.span()=>
            if #cond #then_branch else #else_branch
        };

        tokens.extend(new_tokens);
    }
}

pub struct HtmlBranch {
    brace: token::Brace,
    root: HtmlRoot,
}

impl PeekValue<()> for HtmlBranch {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBranch {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;
        let brace = braced!(content in input);
        let root = content.parse()?;

        Ok(HtmlBranch {
            brace,
            root,
        })
    }
}

impl ToTokens for HtmlBranch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            root,
            ..
        } = self;

        tokens.extend(quote! {
            { #root }
        });
    }
}

pub enum HtmlBranchOrIf {
    Branch(HtmlBranch),
    If(HtmlIf),
}

impl PeekValue<()> for HtmlBranchOrIf {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlBranch::peek(cursor).or_else(|| HtmlIf::peek(cursor))
    }
}

impl Parse for HtmlBranchOrIf {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if HtmlBranch::peek(input.cursor()).is_some() {
            Ok(Self::Branch(input.parse()?))
        } else {
            Ok(Self::If(input.parse()?))
        }
    }
}

impl ToTokens for HtmlBranchOrIf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Branch(x) => x.to_tokens(tokens),
            Self::If(x) => x.to_tokens(tokens),
        }
    }
}
