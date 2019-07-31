use super::html_iterable::HtmlIterable;
use super::html_node::HtmlNode;
use crate::PeekValue;
use proc_macro2::Delimiter;
use quote::{quote, quote_spanned, ToTokens};
use syn::braced;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::token;

pub struct HtmlBlock {
    content: BlockContent,
    brace: token::Brace,
}

enum BlockContent {
    Node(HtmlNode),
    Iterable(HtmlIterable),
}

impl PeekValue<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;
        let brace = braced!(content in input);
        let content = if HtmlIterable::peek(content.cursor()).is_some() {
            BlockContent::Iterable(content.parse()?)
        } else {
            BlockContent::Node(content.parse()?)
        };

        Ok(HtmlBlock { brace, content })
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlBlock { content, brace } = self;
        let new_tokens = match content {
            BlockContent::Iterable(html_iterable) => quote! {#html_iterable},
            BlockContent::Node(html_node) => quote! {#html_node},
        };

        tokens.extend(quote_spanned! {brace.span=> #new_tokens});
    }
}
