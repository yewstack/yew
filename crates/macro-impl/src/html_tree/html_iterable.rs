use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Token};

pub struct HtmlIterable(Expr);

impl Peek<()> for HtmlIterable {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident.to_string() == "for").as_option()
    }
}

impl Parse for HtmlIterable {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let for_token = input.parse::<Token![for]>()?;

        match input.parse() {
            Ok(expr) => Ok(HtmlIterable(expr)),
            Err(err) => {
                if err.to_string().starts_with("unexpected end of input") {
                    Err(syn::Error::new_spanned(
                        for_token,
                        "expected expression after `for`",
                    ))
                } else {
                    Err(err)
                }
            }
        }
    }
}

impl ToTokens for HtmlIterable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.0;
        let vlist = Ident::new("__yew_vlist", expr.span());
        let add_children = quote_spanned! {expr.span()=>
            let __yew_nodes: ::std::boxed::Box<::std::iter::Iterator<Item = _>> = ::std::boxed::Box::new(#expr);
            for __yew_node in __yew_nodes.into_iter() {
                #vlist.add_child(__yew_node);
            }
        };

        let new_tokens = quote! {
            {
                let mut #vlist = $crate::virtual_dom::VList::new();
                #add_children
                $crate::virtual_dom::VNode::from(#vlist)
            }
        };

        tokens.extend(new_tokens);
    }
}
