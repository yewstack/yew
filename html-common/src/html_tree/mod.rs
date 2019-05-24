pub mod html_block;
pub mod html_list;
pub mod html_tag;
pub mod html_text;

use crate::Peek;
use html_block::HtmlBlock;
use html_list::HtmlList;
use html_tag::HtmlTag;
use html_text::HtmlText;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};

pub enum HtmlType {
    Block,
    List,
    Tag,
    Empty,
}

pub enum HtmlTree {
    Block(HtmlBlock),
    List(HtmlList),
    Tag(HtmlTag),
    Text(HtmlText),
    Empty,
}

pub struct HtmlRoot(HtmlTree);
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_root = if HtmlTree::peek(input.cursor()).is_some() {
            HtmlRoot(input.parse()?)
        } else if HtmlText::peek(input.cursor()).is_some() {
            HtmlRoot(HtmlTree::Text(input.parse()?))
        } else {
            return Err(input.error("invalid root html element"));
        };

        if !input.is_empty() {
            Err(syn::Error::new(
                Span::call_site(),
                "only one root html element allowed",
            ))
        } else {
            Ok(html_root)
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
        let html_type =
            HtmlTree::peek(input.cursor()).ok_or(input.error("expected valid html element"))?;
        let html_tree = match html_type {
            HtmlType::Empty => HtmlTree::Empty,
            HtmlType::Tag => HtmlTree::Tag(input.parse()?),
            HtmlType::Block => HtmlTree::Block(input.parse()?),
            HtmlType::List => HtmlTree::List(input.parse()?),
        };
        Ok(html_tree)
    }
}

impl Peek<HtmlType> for HtmlTree {
    fn peek(cursor: Cursor) -> Option<HtmlType> {
        if cursor.eof() {
            Some(HtmlType::Empty)
        } else if HtmlTag::peek(cursor).is_some() {
            Some(HtmlType::Tag)
        } else if HtmlBlock::peek(cursor).is_some() {
            Some(HtmlType::Block)
        } else if HtmlList::peek(cursor).is_some() {
            Some(HtmlType::List)
        } else {
            None
        }
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token_stream = match self {
            HtmlTree::Empty => quote! {
                ::yew::virtual_dom::VNode::VList(
                    ::yew::virtual_dom::vlist::VList::new()
                )
            },
            HtmlTree::Tag(tag) => quote! {
                ::yew::virtual_dom::VNode::VTag(#tag)
            },
            HtmlTree::List(list) => quote! {
                ::yew::virtual_dom::VNode::VList(#list)
            },
            HtmlTree::Text(text) => quote! {#text},
            HtmlTree::Block(block) => quote! {#block},
        };

        tokens.extend(token_stream);
    }
}
