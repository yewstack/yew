use proc_macro2::Delimiter;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::{braced, token};

use super::{HtmlIterable, HtmlNode, ToNodeIterator};
use crate::PeekValue;

pub struct HtmlBlock {
    pub content: BlockContent,
    brace: token::Brace,
}

pub enum BlockContent {
    Node(Box<HtmlNode>),
    Iterable(Box<HtmlIterable>),
}

impl PeekValue<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = braced!(content in input);
        let content = if HtmlIterable::peek(content.cursor()).is_some() {
            BlockContent::Iterable(Box::new(content.parse()?))
        } else {
            BlockContent::Node(Box::new(content.parse()?))
        };

        Ok(HtmlBlock { content, brace })
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlBlock { content, .. } = self;
        let new_tokens = match content {
            BlockContent::Iterable(html_iterable) => quote! {#html_iterable},
            BlockContent::Node(html_node) => quote! {#html_node},
        };

        tokens.extend(quote! {#new_tokens});
    }
}

impl ToNodeIterator for HtmlBlock {
    fn to_node_iterator_stream(&self) -> Option<proc_macro2::TokenStream> {
        let HtmlBlock { content, brace } = self;
        let new_tokens = match content {
            BlockContent::Iterable(iterable) => iterable.to_node_iterator_stream(),
            BlockContent::Node(node) => node.to_node_iterator_stream(),
        }?;

        Some(quote_spanned! {brace.span=> #new_tokens})
    }
}
