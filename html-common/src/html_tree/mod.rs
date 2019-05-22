pub mod html_block;
pub mod html_list;
pub mod html_tag;

use crate::Peek;
use html_block::HtmlBlock;
use html_list::HtmlList;
use html_tag::HtmlTag;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream, Result};

pub enum HtmlTree {
    Block(HtmlBlock),
    List(HtmlList),
    Tag(HtmlTag),
    Empty,
}

impl FromIterator<HtmlTree> for HtmlTree {
    fn from_iter<I: IntoIterator<Item = HtmlTree>>(iter: I) -> Self {
        let mut trees = vec![];
        for tree in iter {
            trees.push(tree);
        }

        match trees.len() {
            0 => HtmlTree::Empty,
            1 => trees.remove(0),
            _ => HtmlTree::List(HtmlList(trees)),
        }
    }
}

pub struct HtmlRoot(HtmlTree);
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_tree = input.parse::<HtmlTree>()?;
        if !input.is_empty() {
            Err(syn::Error::new(
                Span::call_site(),
                "only one root html element allowed",
            ))
        } else {
            Ok(HtmlRoot(html_tree))
        }
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlRoot(html_tree) = self;
        tokens.extend(quote! { #html_tree });
    }
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        if HtmlList::peek(input.cursor()).is_some() {
            Ok(HtmlTree::List(input.parse()?))
        } else if HtmlBlock::peek(input.cursor()).is_some() {
            Ok(HtmlTree::Block(input.parse()?))
        } else if HtmlTag::peek(input.cursor()).is_some() {
            Ok(HtmlTree::Tag(input.parse()?))
        } else if input.is_empty() {
            Ok(HtmlTree::Empty)
        } else {
            Err(input.error("expected valid html element"))
        }
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token_stream = match self {
            HtmlTree::Empty => quote! {
                ::yew_html_common::html_tree::HtmlTree::Empty
            },
            HtmlTree::Tag(tag) => quote! {
                ::yew_html_common::html_tree::HtmlTree::Tag(#tag)
            },
            HtmlTree::List(list) => quote! {
                ::yew_html_common::html_tree::HtmlTree::List(#list)
            },
            HtmlTree::Block(block) => quote! {
                ::yew_html_common::html_tree::HtmlTree::Block(#block)
            },
        };

        tokens.extend(token_stream);
    }
}
