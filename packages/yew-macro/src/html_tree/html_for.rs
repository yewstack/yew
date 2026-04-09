use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{For, In};
use syn::{Expr, Local, Pat, Stmt, Token, braced};

use super::{HtmlChildrenTree, ToNodeIterator};
use crate::PeekValue;
use crate::html_tree::HtmlTree;

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
    let_stmts: Vec<Local>,
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

        let mut let_stmts = Vec::new();
        while body_stream.peek(Token![let]) {
            let stmt: Stmt = body_stream.parse()?;
            match stmt {
                Stmt::Local(local) => let_stmts.push(local),
                _ => unreachable!("peeked Token![let] but parsed non-local statement"),
            }
        }

        let body = HtmlChildrenTree::parse_delimited_with_nodes(&body_stream)?;
        super::check_unnecessary_fragment(&body);
        // TODO: more concise code by using if-let guards (MSRV 1.95)
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
        Ok(Self {
            pat,
            iter,
            let_stmts,
            body,
        })
    }
}

impl ToTokens for HtmlFor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            pat,
            iter,
            let_stmts,
            body,
        } = self;
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

        let body = body
            .0
            .iter()
            .map(|child| match child.to_node_iterator_stream() {
                Some(child) => {
                    quote!( #acc.extend(#child) )
                }
                _ => {
                    quote!( #acc.push(::std::convert::Into::into(#child)) )
                }
            });

        tokens.extend(quote!({
            let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
            ::std::iter::Iterator::for_each(
                ::std::iter::IntoIterator::into_iter(#iter),
                |#pat| { #(#let_stmts)* #alloc_opt; #(#body);* }
            );
            #vlist_gen
        }))
    }
}
