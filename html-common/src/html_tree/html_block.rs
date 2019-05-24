use super::html_text::HtmlText;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::{Delimiter, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::braced;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::token;
use syn::Token;

pub struct HtmlBlock {
    content: BlockContent,
    brace: token::Brace,
}

enum BlockContent {
    Text(HtmlText),
    Iterable(HtmlIterable),
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
        } else if HtmlIterable::peek(content.cursor()).is_some() {
            BlockContent::Iterable(content.parse()?)
        } else {
            BlockContent::Stream(content.parse()?)
        };

        Ok(HtmlBlock { brace, content })
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlBlock { content, brace } = self;
        let new_tokens = match content {
            BlockContent::Text(html_text) => quote! {#html_text},
            BlockContent::Iterable(html_iterable) => quote! {#html_iterable},
            BlockContent::Stream(stream) => quote! {
                ::yew::virtual_dom::VNode::from({#stream})
            },
        };

        tokens.extend(quote_spanned! {brace.span=> #new_tokens});
    }
}

struct HtmlIterable(TokenStream);

impl Peek<()> for HtmlIterable {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident.to_string() == "for").as_option()
    }
}

impl Parse for HtmlIterable {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        input.parse::<Token![for]>()?;
        Ok(HtmlIterable(input.parse()?))
    }
}

impl ToTokens for HtmlIterable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let stream = &self.0;
        let new_tokens = quote! {
            {
                let mut __yew_vlist = ::yew::virtual_dom::VList::new();
                for __yew_node in {#stream} {
                    let __yew_vnode = ::yew::virtual_dom::VNode::from(__yew_node);
                    __yew_vlist.add_child(__yew_vnode);
                }
                ::yew::virtual_dom::VNode::from(__yew_vlist)
            }
        };

        tokens.extend(new_tokens);
    }
}
