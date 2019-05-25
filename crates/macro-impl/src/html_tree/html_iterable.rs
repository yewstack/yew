use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::Token;

pub struct HtmlIterable(TokenStream);

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
                let mut __yew_vlist = $crate::virtual_dom::VList::new();
                let __yew_nodes: ::std::boxed::Box<::std::iter::Iterator<Item = $crate::virtual_dom::VNode<_>>> = ::std::boxed::Box::new({#stream});
                for __yew_node in __yew_nodes {
                    let __yew_vnode = $crate::virtual_dom::VNode::from(__yew_node);
                    __yew_vlist.add_child(__yew_vnode);
                }
                $crate::virtual_dom::VNode::from(__yew_vlist)
            }
        };

        tokens.extend(new_tokens);
    }
}
