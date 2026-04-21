use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::While;
use syn::{Expr, Local, Stmt, Token, braced};

use super::{HtmlChildrenTree, ToNodeIterator};
use crate::PeekValue;
use crate::html_tree::HtmlTree;
use crate::html_tree::html_for::is_contextless_pure;

pub struct HtmlWhile {
    cond: Box<Expr>,
    let_stmts: Vec<Local>,
    body: HtmlChildrenTree,
    deprecations: TokenStream,
}

impl PeekValue<()> for HtmlWhile {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "while").then_some(())
    }
}

impl Parse for HtmlWhile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        While::parse(input)?;
        let cond = Box::new(input.call(Expr::parse_without_eager_brace)?);
        match &*cond {
            Expr::Block(syn::ExprBlock { block, .. }) if block.stmts.is_empty() => {
                return Err(syn::Error::new(
                    cond.span(),
                    "missing condition for `while` expression",
                ));
            }
            _ => {}
        }
        if input.is_empty() {
            return Err(syn::Error::new(
                cond.span(),
                "this `while` expression has a condition, but no block",
            ));
        }

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
        let deprecations = super::check_unnecessary_fragment(&body);
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
                    "duplicate key for a node in a `while`-loop\nthis will create elements with \
                     duplicate keys if the loop iterates more than once",
                ));
            }
        }
        Ok(Self {
            cond,
            let_stmts,
            body,
            deprecations,
        })
    }
}

impl ToTokens for HtmlWhile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            cond,
            let_stmts,
            body,
            deprecations,
        } = self;
        let acc = Ident::new("__yew_v", cond.span());

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

        let body = body.0.iter().map(|child| match child {
            HtmlTree::Break(_) | HtmlTree::Continue(_) => quote!( #child ),
            _ => match child.to_node_iterator_stream() {
                Some(stream) => quote!( #acc.extend(#stream) ),
                _ => quote!( #acc.push(::std::convert::Into::into(#child)) ),
            },
        });

        tokens.extend(quote!({
            #deprecations
            let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
            while #cond {
                #(#let_stmts)* #alloc_opt; #(#body);*
            }
            #vlist_gen
        }))
    }
}
