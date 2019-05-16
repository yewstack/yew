use super::HtmlTree;
use crate::Peek;
use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::braced;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

pub struct HtmlBlock {
    pub tree: Box<HtmlTree>,
    content: TokenStream,
}

impl HtmlBlock {
    pub fn new(tree: HtmlTree) -> Self {
        HtmlBlock {
            tree: Box::new(tree),
            content: TokenStream::new(),
        }
    }
}

impl Peek for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        Ok(HtmlBlock {
            tree: Box::new(HtmlTree::Empty),
            content: content.parse()?,
        })
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlBlock { content, .. } = self;
        let tree = Ident::new("__yew_html_tree", Span::call_site());
        let init_tree = quote_spanned! {content.span()=>
            let #tree: ::yew_html_common::html_tree::HtmlTree = {#content};
        };
        tokens.extend(quote! {{
            #init_tree
            ::yew_html_common::html_tree::html_block::HtmlBlock::new(#tree)
        }});
    }
}
