use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Expr, Local, Stmt, Token};

use super::{HtmlChildrenTree, HtmlTree, ToNodeIterator};

/// Determines if an expression is guaranteed to always return the same value anywhere.
pub(super) fn is_contextless_pure(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Path(path) => path.path.get_ident().is_none(),
        _ => false,
    }
}

/// Parse leading `let` bindings from a loop body, then the remaining children.
/// Also runs duplicate-key detection keyed to `loop_kind` (e.g. "for", "while").
pub(super) fn parse_loop_body(
    body_stream: ParseStream,
    loop_kind: &str,
) -> syn::Result<(Vec<Local>, HtmlChildrenTree, TokenStream)> {
    let mut let_stmts = Vec::new();
    while body_stream.peek(Token![let]) {
        let stmt: Stmt = body_stream.parse()?;
        match stmt {
            Stmt::Local(local) => let_stmts.push(local),
            _ => unreachable!("peeked Token![let] but parsed non-local statement"),
        }
    }

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

    Ok((let_stmts, body, deprecations))
}

/// Emit a loop that accumulates its body children into a `VList`.
///
/// `loop_header` is the native Rust loop syntax without its body, e.g.
/// `for #pat in #iter` or `while #cond`. `span` is used to place the internal
/// accumulator identifier.
pub(super) fn emit_loop(
    loop_header: TokenStream,
    span: Span,
    let_stmts: &[Local],
    body: &HtmlChildrenTree,
    deprecations: &TokenStream,
) -> TokenStream {
    let acc = Ident::new("__yew_v", span);

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
        HtmlTree::Break(_) | HtmlTree::Continue(_) => quote!( #child ),
        _ => match child.to_node_iterator_stream() {
            Some(stream) => quote!( #acc.extend(#stream) ),
            _ => quote!( #acc.push(::std::convert::Into::into(#child)) ),
        },
    });

    quote!({
        #deprecations
        let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
        #loop_header {
            #(#let_stmts)* #alloc_opt; #(#body_streams);*
        }
        #vlist_gen
    })
}
