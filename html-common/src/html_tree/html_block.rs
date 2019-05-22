use super::html_text::HtmlText;
use super::HtmlTree;
use crate::Peek;
use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::braced;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::token;

pub struct HtmlBlock {
    tree: Box<HtmlTree>,
    content: BlockContent,
    brace: Option<token::Brace>,
}

enum BlockContent {
    Text(HtmlText),
    Stream(TokenStream),
}

impl Peek<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;
        let brace = braced!(content in input);
        let content = if HtmlText::peek(content.cursor()).is_some() {
            BlockContent::Text(content.parse()?)
        } else {
            BlockContent::Stream(content.parse()?)
        };

        Ok(HtmlBlock {
            tree: Box::new(HtmlTree::Empty),
            brace: Some(brace),
            content,
        })
    }
}

impl HtmlBlock {
    pub fn new(tree: HtmlTree) -> Self {
        HtmlBlock {
            tree: Box::new(tree),
            content: BlockContent::Stream(TokenStream::new()),
            brace: None,
        }
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlBlock { content, brace, .. } = self;
        let tree = Ident::new("__yew_html_tree", Span::call_site());
        let content: Box<dyn ToTokens> = match content {
            BlockContent::Text(html_text) => Box::new(quote! {
                ::yew_html_common::html_tree::HtmlTree::Text(#html_text)
            }),
            BlockContent::Stream(stream) => Box::new(stream),
        };

        let init_tree = quote_spanned! {brace.unwrap().span=>
            let #tree: ::yew_html_common::html_tree::HtmlTree = {#content};
        };

        tokens.extend(quote! {{
            #init_tree
            ::yew_html_common::html_tree::html_block::HtmlBlock::new(#tree)
        }});
    }
}
