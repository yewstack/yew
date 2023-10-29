use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, Expr, Pat, Token};

use super::{HtmlChildrenTree, ToNodeIterator};
use crate::PeekValue;

pub struct HtmlFor {
    for_token: Token![for],
    pat: Pat,
    in_token: Token![in],
    iter: Expr,
    body: HtmlChildrenTree,
}

impl PeekValue<()> for HtmlFor {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "for").then_some(())
    }
}

impl Parse for HtmlFor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let for_token = input.parse()?;
        let pat = Pat::parse_single(input)?;
        let in_token = input.parse()?;
        let iter = Expr::parse_without_eager_brace(input)?;

        let body_stream;
        braced!(body_stream in input);
        let body = HtmlChildrenTree::parse_delimited(&body_stream)?;
        Ok(Self {
            for_token,
            pat,
            in_token,
            iter,
            body,
        })
    }
}

impl ToTokens for HtmlFor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            for_token,
            pat,
            in_token,
            iter,
            body,
        } = self;
        // TODO: call `__yew_v.reserve` if the amount of elements added per iteration can be
        // pre-determined
        let acc = Ident::new("__yew_v", iter.span());
        let optimisation = body.size_hint().map(|size| {
            quote!(
                #acc.reserve(#size);
            )
        });
        let body = body.0.iter().map(|child| {
            if let Some(child) = child.to_node_iterator_stream() {
                quote!(
                    #acc.extend(#child);
                )
            } else {
                quote!(
                    #acc.push(::std::convert::Into::into(#child));
                )
            }
        });
        tokens.extend(quote!({
            let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
            #for_token #pat #in_token #iter {
                #optimisation
                #(#body)*
            }
            ::yew::virtual_dom::VList::with_children(#acc, ::std::option::Option::None)
        }))
    }
}
