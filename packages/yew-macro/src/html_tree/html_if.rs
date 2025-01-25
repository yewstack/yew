use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Token};

use super::{HtmlRootBraced, ToNodeIterator};
use crate::PeekValue;

pub struct HtmlIf {
    if_token: Token![if],
    cond: Box<Expr>,
    then_branch: HtmlRootBraced,
    else_branch: Option<(Token![else], Box<HtmlRootBracedOrIf>)>,
}

impl PeekValue<()> for HtmlIf {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "if").then_some(())
    }
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let if_token = input.parse()?;
        let cond = Box::new(input.call(Expr::parse_without_eager_brace)?);
        match &*cond {
            Expr::Block(syn::ExprBlock { block, .. }) if block.stmts.is_empty() => {
                return Err(syn::Error::new(
                    cond.span(),
                    "missing condition for `if` expression",
                ))
            }
            _ => {}
        }
        if input.is_empty() {
            return Err(syn::Error::new(
                cond.span(),
                "this `if` expression has a condition, but no block",
            ));
        }

        let then_branch = input.parse()?;
        let else_branch = input
            .parse::<Token![else]>()
            .ok()
            .map(|else_token| {
                if input.is_empty() {
                    return Err(syn::Error::new(
                        else_token.span(),
                        "expected block or `if` after `else`",
                    ));
                }

                input.parse().map(|branch| (else_token, branch))
            })
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
        let HtmlIf {
            if_token,
            cond,
            then_branch,
            else_branch,
        } = self;
        let default_else_branch = syn::parse_quote! { {} };
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

impl ToNodeIterator for HtmlIf {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
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

        Some(quote_spanned! {if_token.span=> #new_tokens})
    }
}

pub enum HtmlRootBracedOrIf {
    Branch(HtmlRootBraced),
    If(HtmlIf),
}

impl PeekValue<()> for HtmlRootBracedOrIf {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlRootBraced::peek(cursor).or_else(|| HtmlIf::peek(cursor))
    }
}

impl Parse for HtmlRootBracedOrIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlRootBraced::peek(input.cursor()).is_some() {
            input.parse().map(Self::Branch)
        } else {
            input.parse().map(Self::If)
        }
    }
}

impl ToTokens for HtmlRootBracedOrIf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Branch(x) => x.to_tokens(tokens),
            Self::If(x) => x.to_tokens(tokens),
        }
    }
}
