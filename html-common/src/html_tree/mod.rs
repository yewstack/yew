pub mod html_block;
pub mod html_list;

use crate::Peek;
use html_block::HtmlBlock;
use html_list::HtmlList;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

pub enum HtmlTree {
    Block(HtmlBlock),
    List(HtmlList),
    Empty,
}

pub struct HtmlRoot(HtmlTree);
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_tree = input.parse::<HtmlTree>()?;
        if !input.is_empty() {
            Err(input.error("only one root html element allowed"))
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
