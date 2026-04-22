use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Expr, Stmt};

use super::{
    HtmlChildrenTree, HtmlTree, ToNodeIterator, parse_preamble_stmts, stmts_have_divergent,
};

/// Determines if an expression is guaranteed to always return the same value anywhere.
pub(super) fn is_contextless_pure(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Path(path) => path.path.get_ident().is_none(),
        _ => false,
    }
}

/// Parse leading Rust statements from a loop body, then the remaining children.
/// Also runs duplicate-key detection keyed to `loop_kind` (e.g. "for", "while").
pub(super) fn parse_loop_body(
    body_stream: ParseStream,
    loop_kind: &str,
) -> syn::Result<(Vec<Stmt>, HtmlChildrenTree, TokenStream)> {
    let stmts = parse_preamble_stmts(body_stream)?;

    let body = HtmlChildrenTree::parse_delimited_with_nodes(body_stream)?;
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
                format!(
                    "duplicate key for a node in a `{loop_kind}`-loop\nthis will create elements \
                     with duplicate keys if the loop iterates more than once"
                ),
            ));
        }
    }

    Ok((stmts, body, deprecations))
}

/// Emit a loop that accumulates its body children into a `VList`.
///
/// `loop_header` is the native Rust loop syntax without its body, e.g.
/// `for #pat in #iter` or `while #cond`.
pub(super) fn emit_loop(
    loop_header: TokenStream,
    stmts: &[Stmt],
    body: &HtmlChildrenTree,
    deprecations: &TokenStream,
) -> TokenStream {
    let acc = Ident::new("__yew_v", Span::mixed_site());

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

    let body_streams = body.0.iter().map(|child| match child {
        HtmlTree::Break(_) | HtmlTree::Continue(_) | HtmlTree::Return(_) => quote!( #child ),
        _ => match child.to_node_iterator_stream() {
            Some(stream) => quote!( #acc.extend(#stream) ),
            _ => quote!( #acc.push(::std::convert::Into::into(#child)) ),
        },
    });

    let has_top_level_divergent = body.0.iter().any(|c| {
        matches!(
            c,
            HtmlTree::Break(_) | HtmlTree::Continue(_) | HtmlTree::Return(_)
        )
    }) || stmts_have_divergent(stmts);

    // Nest in an inner block when divergent, so `#![allow(unreachable_code)]`
    // lands in an inner expression block (accepted everywhere) rather than in
    // an if-branch or match-arm position where it would be rejected.
    if has_top_level_divergent {
        quote!({
            #deprecations
            {
                #![allow(unreachable_code)]
                let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
                #loop_header {
                    #(#stmts)* #alloc_opt; #(#body_streams);*
                }
                #vlist_gen
            }
        })
    } else {
        quote!({
            #deprecations
            let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
            #loop_header {
                #(#stmts)* #alloc_opt; #(#body_streams);*
            }
            #vlist_gen
        })
    }
}
