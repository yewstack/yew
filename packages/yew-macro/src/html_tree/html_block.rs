use super::{HtmlIterable, HtmlNode, ToNodeIterator};
use crate::PeekValue;
use proc_macro2::Delimiter;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{braced, token, Lit};

pub struct HtmlBlock {
    content: BlockContent,
    /// Note that the type is `Option<_>`. This is because the braces are optional for string literals
    brace: Option<token::Brace>,
}

enum BlockContent {
    Node(Box<HtmlNode>),
    Iterable(Box<HtmlIterable>),
}

impl PeekValue<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor
            .literal()
            .map(|_| ())
            .or_else(|| cursor.group(Delimiter::Brace).map(|_| ()))
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        if input.peek(Brace) {
            let brace = braced!(content in input);
            let content = if HtmlIterable::peek(content.cursor()).is_some() {
                BlockContent::Iterable(Box::new(content.parse()?))
            } else {
                BlockContent::Node(Box::new(content.parse()?))
            };

            Ok(HtmlBlock {
                brace: Some(brace),
                content,
            })
        } else {
            // parse string literal
            let content: Lit = input.parse()?;

            if !matches!(content, Lit::Str(_)) {
                return Err(syn::Error::new_spanned(
                    content,
                    "expected braces (`{...}`) around literal",
                ));
            }

            Ok(HtmlBlock {
                brace: None,
                content: BlockContent::Node(Box::new(HtmlNode::Literal(Box::new(content)))),
            })
        }
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
        if let HtmlBlock {
            content,
            brace: Some(brace),
        } = self
        {
            let new_tokens = match content {
                BlockContent::Iterable(iterable) => iterable.to_node_iterator_stream(),
                BlockContent::Node(node) => node.to_node_iterator_stream(),
            }?;

            Some(quote_spanned! {brace.span=> #new_tokens})
        } else {
            match &self.content {
                BlockContent::Iterable(iterable) => iterable.to_node_iterator_stream(),
                BlockContent::Node(node) => node.to_node_iterator_stream(),
            }
        }
    }
}
