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
use super::HtmlChildrenTree;
use super::HtmlElement;

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

pub enum HtmlBranch {
    Children {
        brace: token::Brace,
        root: HtmlChildrenTree,
    },
    Block(Block),
}

impl PeekValue<()> for HtmlBranch {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBranch {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if HtmlElement::peek(input.cursor()).is_some() || input.cursor().eof() {
            let content;
            let brace = braced!(content in input);
            let mut root = HtmlChildrenTree::new();
            root.parse_child(&content)?; // TODO

            Ok(Self::Children {
                brace,
                root,
            })
        } else {
            Ok(Self::Block(input.parse()?))
        }
    }
}

impl ToTokens for HtmlBranch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Children { root, .. } => {
                let key = quote! { None }; // TODO

                tokens.extend(quote! {
                    {
                        ::yew::virtual_dom::VNode::VList(
                            ::yew::virtual_dom::VList::new_with_children(#root, #key)
                        )
                    }
                });
            },
            Self::Block(block) => tokens.extend(quote! { #block }),
        }
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
