use proc_macro2::Delimiter;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::braced;

use super::{HtmlIterable, HtmlNode, ToNodeIterator};
use crate::PeekValue;

pub struct HtmlBlock {
    pub content: BlockContent,
    brace: Option<Brace>,
}

pub enum BlockContent {
    Node(Box<HtmlNode>),
    Iterable(Box<HtmlIterable>),
}

impl PeekValue<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        if cursor.group(Delimiter::Brace).is_some() {return Some(())}
        cursor.literal().map(drop)
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(lit) = input.parse() {
            return Ok(Self { content: BlockContent::Node(HtmlNode::Literal(lit).into()), brace: None })
        }
        let content;
        let brace = Some(braced!(content in input));
        let content = if HtmlIterable::peek(content.cursor()).is_some() {
            BlockContent::Iterable(Box::new(content.parse()?))
        } else {
            BlockContent::Node(Box::new(content.parse()?))
        };

        Ok(Self { content, brace })
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match &self.content {
            BlockContent::Iterable(html_iterable) => quote! {#html_iterable},
            BlockContent::Node(html_node) => quote! {#html_node},
        })
    }
}

impl ToNodeIterator for HtmlBlock {
    fn to_node_iterator_stream(&self) -> Option<proc_macro2::TokenStream> {
        let new_tokens = match &self.content {
            BlockContent::Iterable(iterable) => iterable.to_node_iterator_stream(),
            BlockContent::Node(node) => node.to_node_iterator_stream(),
        }?;

        Some(if let Some(brace) = self.brace {
            quote_spanned! {brace.span => #new_tokens}
        } else {
            quote! {#new_tokens}
        })
    }
}
