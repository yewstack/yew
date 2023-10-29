use std::iter::successors;

use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{For, In};
use syn::{braced, Expr, Pat};

use super::{HtmlChildrenTree, ToNodeIterator};
use crate::PeekValue;

/// Returns the location of a `break` or `continue` token, if found
fn find_divergence(cursor: Cursor) -> Option<Span> {
    fn inner(stream: TokenStream) -> Option<Span> {
        for token in stream {
            match token {
                TokenTree::Group(group) => {
                    if let res @ Some(_) = inner(group.stream()) {
                        return res;
                    }
                }
                TokenTree::Ident(ident) => {
                    if ident == "break" || ident == "continue" {
                        return Some(ident.span());
                    }
                }
                TokenTree::Punct(_) | TokenTree::Literal(_) => (),
            }
        }
        None
    }

    for (token, _) in successors(cursor.token_tree(), |(_, cursor)| cursor.token_tree()) {
        if let TokenTree::Group(group) = token {
            if group.delimiter() == Delimiter::Brace {
                if let res @ Some(_) = inner(group.stream()) {
                    return res;
                }
            }
        }
    }

    None
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

        if let Some(span) = find_divergence(body_stream.cursor()) {
            return Err(syn::Error::new(
                span,
                "diverging expression in the body of a for loop\n`break` or `continue` are not \
                 allowed in `html!` for loops",
            ));
        }

        let body = HtmlChildrenTree::parse_delimited(&body_stream)?;
        Ok(Self { pat, iter, body })
    }
}

impl ToTokens for HtmlFor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { pat, iter, body } = self;
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
            for #pat in #iter {
                #optimisation
                #(#body)*
            }
            ::yew::virtual_dom::VList::with_children(#acc, ::std::option::Option::None)
        }))
    }
}
