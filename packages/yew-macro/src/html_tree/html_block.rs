use proc_macro2::Delimiter;
use quote::{ToTokens, quote, quote_spanned};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Stmt, braced, token};

use super::{HtmlIterable, HtmlNode, ToNodeIterator};
use crate::PeekValue;

pub struct HtmlBlock {
    pub content: BlockContent,
    brace: token::Brace,
}

pub enum BlockContent {
    Node(Box<HtmlNode>),
    Iterable(Box<HtmlIterable>),
}

impl PeekValue<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = braced!(content in input);
        let content = if HtmlIterable::peek(content.cursor()).is_some() {
            BlockContent::Iterable(Box::new(content.parse()?))
        } else {
            let node: HtmlNode = content.parse()?;
            if let HtmlNode::Expression(ref expr) = node {
                check_deprecated_html_call(expr);
            }
            BlockContent::Node(Box::new(node))
        };

        Ok(HtmlBlock { content, brace })
    }
}

/// Check for deprecated `html!` usage patterns inside expression blocks.
fn check_deprecated_html_call(expr: &Expr) {
    // Pattern 1: { match expr { arm => html! { ... }, ... } }
    if let Expr::Match(match_expr) = expr {
        for arm in &match_expr.arms {
            if let Some(span) = html_macro_call_span(&arm.body) {
                super::emit_deprecated!(
                    span,
                    "Use bare elements in arms directly \n\nmatch value {\n    pattern => \
                     <Element />,\n}"
                );
                return;
            }
        }
    }

    // Pattern 2: {{ let x = ...; html! { ... } }}
    if let Expr::Block(block_expr) = expr {
        if let Some(span) = block_expr
            .block
            .stmts
            .last()
            .and_then(stmt_tail_html_macro_span)
        {
            super::emit_deprecated!(
                span,
                "`html!` is not needed inside expression blocks. Use `let` bindings and bare \
                 elements directly"
            );
        }
    }
}

/// Check if a statement is a tail `html!`/`html_nested!` macro call (no trailing semicolon).
fn stmt_tail_html_macro_span(stmt: &Stmt) -> Option<proc_macro2::Span> {
    match stmt {
        Stmt::Expr(expr, None) => html_macro_call_span(expr),
        Stmt::Macro(stmt_mac) if stmt_mac.semi_token.is_none() => {
            macro_path_html_span(&stmt_mac.mac.path)
        }
        _ => None,
    }
}

pub(super) fn html_macro_call_span(expr: &Expr) -> Option<proc_macro2::Span> {
    let m = match expr {
        Expr::Macro(m) => m,
        _ => return None,
    };
    macro_path_html_span(&m.mac.path)
}

fn macro_path_html_span(path: &syn::Path) -> Option<proc_macro2::Span> {
    let ident = &path.segments.last()?.ident;
    (ident == "html" || ident == "html_nested").then(|| path.span())
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlBlock { content, .. } = self;
        let new_tokens = match content {
            BlockContent::Iterable(html_iterable) => quote! {#html_iterable},
            BlockContent::Node(html_node) => quote! {#html_node},
        };

        tokens.extend(quote! {#new_tokens});
    }
}

impl ToNodeIterator for HtmlBlock {
    fn to_node_iterator_stream(&self) -> Option<proc_macro2::TokenStream> {
        let HtmlBlock { content, brace } = self;
        let new_tokens = match content {
            BlockContent::Iterable(iterable) => iterable.to_node_iterator_stream(),
            BlockContent::Node(node) => node.to_node_iterator_stream(),
        }?;

        Some(quote_spanned! {brace.span=> #new_tokens})
    }

    fn is_singular(&self) -> bool {
        match &self.content {
            BlockContent::Node(node) => node.is_singular(),
            BlockContent::Iterable(_) => false,
        }
    }
}
