use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{For, In};
use syn::{braced, Expr, Pat};

use super::{HtmlChildrenTree, ToNodeIterator};
use crate::html_tree::HtmlTree;
use crate::PeekValue;

/// Determines if an expression is guaranteed to always return the same value anywhere.
fn is_contextless_pure(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Path(path) => path.path.get_ident().is_none(),
        _ => false,
    }
}

pub struct HtmlFor {
    pat: Pat,
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
        For::parse(input)?;
        let pat = Pat::parse_single(input)?;
        In::parse(input)?;
        let iter = Expr::parse_without_eager_brace(input)?;

        let body_stream;
        braced!(body_stream in input);

        let body = HtmlChildrenTree::parse_delimited(&body_stream)?;
        // TODO: more concise code by using if-let guards once MSRV is raised
        for child in body.0.iter() {
            let HtmlTree::Element(element) = child else {
                continue;
            };

            let Some(key) = &element.props.special.key else {
                continue;
            };

            if is_contextless_pure(&key.value) {
                return Err(syn::Error::new(
                    key.value.span(),
                    "duplicate key for a node in a `for`-loop\nthis will create elements with \
                     duplicate keys if the loop iterates more than once",
                ));
            }
        }
        Ok(Self { pat, iter, body })
    }
}

impl ToTokens for HtmlFor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { pat, iter, body } = self;
        let acc = Ident::new("__yew_v", iter.span());

        let alloc_opt = body
            .size_hint()
            .filter(|&size| size > 1) // explicitly reserving space for 1 more element is redundant
            .map(|size| quote!( #acc.reserve(#size) ));

        let vlist_gen = match body.fully_keyed() {
            Some(true) => quote! {
                ::yew::virtual_dom::VList::__macro_new(
                    #acc,
                    ::std::option::Option::None,
                    ::yew::virtual_dom::FullyKeyedState::KnownFullyKeyed
                )
            },
            Some(false) => quote! {
                ::yew::virtual_dom::VList::__macro_new(
                    #acc,
                    ::std::option::Option::None,
                    ::yew::virtual_dom::FullyKeyedState::KnownMissingKeys
                )
            },
            None => quote! {
                ::yew::virtual_dom::VList::with_children(#acc, ::std::option::Option::None)
            },
        };

        let body = body.0.iter().map(|child| {
            if let Some(child) = child.to_node_iterator_stream() {
                quote!( #acc.extend(#child) )
            } else {
                quote!( #acc.push(::std::convert::Into::into(#child)) )
            }
        });

        tokens.extend(quote!({
            let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
            ::std::iter::Iterator::for_each(
                ::std::iter::IntoIterator::into_iter(#iter),
                |#pat| { #alloc_opt; #(#body);* }
            );
            #vlist_gen
        }))
    }
}
