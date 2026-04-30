use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::buffer::Cursor;
use syn::ext::IdentExt;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Token, braced, token};

use crate::{PeekValue, is_ide_completion};

mod html_block;
mod html_component;
mod html_dashed_name;
mod html_element;
mod html_for;
mod html_if;
mod html_iterable;
mod html_list;
mod html_loop;
mod html_match;
mod html_node;
mod html_while;
mod lint;
mod tag;

use html_block::HtmlBlock;
use html_component::HtmlComponent;
pub use html_dashed_name::HtmlDashedName;
use html_element::HtmlElement;
use html_if::HtmlIf;
use html_iterable::HtmlIterable;
use html_list::HtmlList;
use html_node::HtmlNode;
use tag::TagTokens;

use self::html_block::BlockContent;
use self::html_for::HtmlFor;
use self::html_match::HtmlMatch;
use self::html_while::HtmlWhile;

pub enum HtmlType {
    Block,
    Component,
    List,
    Element,
    If,
    For,
    While,
    Match,
    Break,
    Continue,
    Return,
    Empty,
}

pub enum HtmlTree {
    Block(Box<HtmlBlock>),
    Component(Box<HtmlComponent>),
    List(Box<HtmlList>),
    Element(Box<HtmlElement>),
    If(Box<HtmlIf>),
    For(Box<HtmlFor>),
    While(Box<HtmlWhile>),
    Match(Box<HtmlMatch>),
    Node(Box<HtmlNode>),
    Break(Box<syn::ExprBreak>),
    Continue(Box<syn::ExprContinue>),
    Return(Box<syn::ExprReturn>),
    Empty,
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html_type = Self::peek_html_type(input)
            .ok_or_else(|| input.error("expected a valid html element"))?;
        Ok(match html_type {
            HtmlType::Empty => Self::Empty,
            HtmlType::Component => Self::Component(Box::new(input.parse()?)),
            HtmlType::Element => Self::Element(Box::new(input.parse()?)),
            HtmlType::Block => Self::Block(Box::new(input.parse()?)),
            HtmlType::List => Self::List(Box::new(input.parse()?)),
            HtmlType::If => Self::If(Box::new(input.parse()?)),
            HtmlType::For => Self::For(Box::new(input.parse()?)),
            HtmlType::While => Self::While(Box::new(input.parse()?)),
            HtmlType::Match => Self::Match(Box::new(input.parse()?)),
            HtmlType::Break => {
                let expr = parse_break(input)?;
                while input.peek(Token![;]) {
                    let _: Token![;] = input.parse()?;
                }
                Self::Break(Box::new(expr))
            }
            HtmlType::Continue => {
                let expr = parse_continue(input)?;
                while input.peek(Token![;]) {
                    let _: Token![;] = input.parse()?;
                }
                Self::Continue(Box::new(expr))
            }
            HtmlType::Return => {
                let expr = parse_return(input)?;
                while input.peek(Token![;]) {
                    let _: Token![;] = input.parse()?;
                }
                Self::Return(Box::new(expr))
            }
        })
    }
}

impl HtmlTree {
    /// Parse an HtmlTree, falling back to HtmlNode for bare literals/expressions.
    pub(crate) fn parse_or_node(input: ParseStream) -> syn::Result<Self> {
        if Self::peek_html_type(input).is_some() {
            input.parse()
        } else {
            let node: HtmlNode = input.parse()?;
            Ok(Self::Node(Box::new(node)))
        }
    }

    /// Determine the [`HtmlType`] before actually parsing it.
    /// Even though this method accepts a [`ParseStream`], it is forked and the original stream is
    /// not modified. Once a certain `HtmlType` can be deduced for certain, the function eagerly
    /// returns with the appropriate type. If invalid html tag, returns `None`.
    fn peek_html_type(input: ParseStream) -> Option<HtmlType> {
        let input = input.fork(); // do not modify original ParseStream
        let cursor = input.cursor();

        if input.is_empty() {
            Some(HtmlType::Empty)
        } else if HtmlBlock::peek(cursor).is_some() {
            Some(HtmlType::Block)
        } else if HtmlIf::peek(cursor).is_some() {
            Some(HtmlType::If)
        } else if HtmlFor::peek(cursor).is_some() {
            Some(HtmlType::For)
        } else if HtmlWhile::peek(cursor).is_some() {
            Some(HtmlType::While)
        } else if HtmlMatch::peek(cursor).is_some() {
            Some(HtmlType::Match)
        } else if cursor.ident().map(|(i, _)| i == "break").unwrap_or(false) {
            Some(HtmlType::Break)
        } else if cursor
            .ident()
            .map(|(i, _)| i == "continue")
            .unwrap_or(false)
        {
            Some(HtmlType::Continue)
        } else if cursor.ident().map(|(i, _)| i == "return").unwrap_or(false) {
            Some(HtmlType::Return)
        } else if input.peek(Token![<]) {
            let _lt: Token![<] = input.parse().ok()?;

            // eat '/' character for unmatched closing tag
            let _slash: Option<Token![/]> = input.parse().ok();

            if input.peek(Token![>]) {
                Some(HtmlType::List)
            } else if input.peek(Token![@]) {
                Some(HtmlType::Element) // dynamic element
            } else if input.peek(Token![::]) {
                Some(HtmlType::Component)
            } else if input.peek(Ident::peek_any) {
                let ident = Ident::parse_any(&input).ok()?;
                let ident_str = ident.to_string();

                if input.peek(Token![=]) || (input.peek(Token![?]) && input.peek2(Token![=])) {
                    Some(HtmlType::List)
                } else if ident_str.chars().next().unwrap().is_ascii_uppercase()
                    || input.peek(Token![::])
                    || is_ide_completion() && ident_str.chars().any(|c| c.is_ascii_uppercase())
                {
                    Some(HtmlType::Component)
                } else {
                    Some(HtmlType::Element)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        lint::lint_all(self);
        match self {
            Self::Empty => tokens.extend(quote! {
                <::yew::virtual_dom::VNode as ::std::default::Default>::default()
            }),
            Self::Component(comp) => comp.to_tokens(tokens),
            Self::Element(tag) => tag.to_tokens(tokens),
            Self::List(list) => list.to_tokens(tokens),
            Self::Block(block) => block.to_tokens(tokens),
            Self::If(block) => block.to_tokens(tokens),
            Self::For(block) => block.to_tokens(tokens),
            Self::While(block) => block.to_tokens(tokens),
            Self::Match(block) => block.to_tokens(tokens),
            Self::Node(node) => node.to_tokens(tokens),
            Self::Break(expr) => expr.to_tokens(tokens),
            Self::Continue(expr) => expr.to_tokens(tokens),
            Self::Return(expr) => expr.to_tokens(tokens),
        }
    }
}

pub struct HtmlRoot {
    children: HtmlChildrenTree,
    deprecations: TokenStream,
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let children = HtmlChildrenTree::parse_delimited_with_nodes(input)?;
        let deprecations = check_unnecessary_fragment(&children);
        Ok(Self {
            children,
            deprecations,
        })
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let deprecations = &self.deprecations;
        match &self.children.0[..] {
            [] => tokens.extend(quote! {
                { #deprecations <::yew::virtual_dom::VNode as ::std::default::Default>::default() }
            }),
            [single] => {
                if deprecations.is_empty() {
                    single.to_tokens(tokens);
                } else {
                    tokens.extend(quote! { { #deprecations #single } });
                }
            }
            _ => {
                let children = &self.children;
                let vlist = match children.fully_keyed() {
                    Some(true) => quote! {
                        ::yew::virtual_dom::VList::__macro_new(#children, ::std::option::Option::None, ::yew::virtual_dom::FullyKeyedState::KnownFullyKeyed)
                    },
                    Some(false) => quote! {
                        ::yew::virtual_dom::VList::__macro_new(#children, ::std::option::Option::None, ::yew::virtual_dom::FullyKeyedState::KnownMissingKeys)
                    },
                    None => quote! {
                        ::yew::virtual_dom::VList::with_children(#children, ::std::option::Option::None)
                    },
                };
                tokens.extend(quote! {
                    {
                        #deprecations
                        ::yew::virtual_dom::VNode::VList(::std::rc::Rc::new(#vlist))
                    }
                });
            }
        }
    }
}

/// Same as HtmlRoot but always returns a VNode.
pub struct HtmlRootVNode(HtmlRoot);
impl Parse for HtmlRootVNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for HtmlRootVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_tokens = self.0.to_token_stream();
        let span = new_tokens.span();
        tokens.extend(quote_spanned! {span.resolved_at(Span::mixed_site())=> {
            #[allow(clippy::useless_conversion)]
            <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(#new_tokens)
        }});
    }
}

/// This trait represents a type that can be unfolded into multiple html nodes.
pub trait ToNodeIterator {
    /// Generate a token stream which produces a value that implements IntoIterator<Item=T> where T
    /// is inferred by the compiler. The easiest way to achieve this is to call `.into()` on
    /// each element. If the resulting iterator only ever yields a single item this function
    /// should return None instead.
    fn to_node_iterator_stream(&self) -> Option<TokenStream>;
    /// Returns a boolean indicating whether the node can only ever unfold into 1 node
    /// Same as calling `.to_node_iterator_stream().is_none()`,
    /// but doesn't actually construct any token stream
    fn is_singular(&self) -> bool;
}

impl ToNodeIterator for HtmlTree {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        match self {
            Self::Block(block) => block.to_node_iterator_stream(),
            Self::Node(node) => node.to_node_iterator_stream(),
            _ => None,
        }
    }

    fn is_singular(&self) -> bool {
        match self {
            Self::Block(block) => block.is_singular(),
            Self::Node(node) => node.is_singular(),
            _ => true,
        }
    }
}

pub struct HtmlChildrenTree(pub Vec<HtmlTree>);

impl HtmlChildrenTree {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn parse_child(&mut self, input: ParseStream) -> syn::Result<()> {
        self.0.push(input.parse()?);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // Check if each child represents a single node.
    // This is the case when no expressions are used.
    fn only_single_node_children(&self) -> bool {
        self.0.iter().all(HtmlTree::is_singular)
    }

    pub fn to_build_vec_token_stream(&self) -> TokenStream {
        let Self(children) = self;

        let has_divergent = children.iter().any(|c| {
            matches!(
                c,
                HtmlTree::Break(_) | HtmlTree::Continue(_) | HtmlTree::Return(_)
            )
        });

        if !has_divergent && self.only_single_node_children() {
            // optimize for the common case where all children are single nodes (only using literal
            // html).
            let children_into = children
                .iter()
                .map(|child| quote_spanned! {child.span()=> ::std::convert::Into::into(#child) });
            return quote! {
                [#(#children_into),*].to_vec()
            };
        }

        let vec_ident = Ident::new("__yew_v", Span::mixed_site());
        let add_children_streams = children.iter().map(|child| match child {
            HtmlTree::Break(_) | HtmlTree::Continue(_) | HtmlTree::Return(_) => {
                quote!( #child; )
            }
            _ => match child.to_node_iterator_stream() {
                Some(node_iterator_stream) => {
                    quote! {
                        ::std::iter::Extend::extend(&mut #vec_ident, #node_iterator_stream);
                    }
                }
                _ => {
                    quote_spanned! {child.span()=>
                        #vec_ident.push(::std::convert::Into::into(#child));
                    }
                }
            },
        });

        let allow_unreachable = has_divergent.then(|| quote!( #![allow(unreachable_code)] ));
        quote! {
            {
                #allow_unreachable
                let mut #vec_ident = ::std::vec::Vec::new();
                #(#add_children_streams)*
                #vec_ident
            }
        }
    }

    pub fn parse_delimited_with_nodes(input: ParseStream) -> syn::Result<Self> {
        let mut children = HtmlChildrenTree::new();

        while !input.is_empty() {
            if HtmlTree::peek_html_type(input).is_some() {
                children.parse_child(input)?;
            } else {
                let node: HtmlNode = input.parse()?;
                children.0.push(HtmlTree::Node(Box::new(node)));
            }
        }

        Ok(children)
    }

    pub fn to_children_renderer_tokens(&self) -> Option<TokenStream> {
        match self.0[..] {
            [] => None,
            [HtmlTree::Component(ref children)] => Some(quote! { #children }),
            [HtmlTree::Element(ref children)] => Some(quote! { #children }),
            [HtmlTree::Block(ref m)] => {
                // We only want to process `{vnode}` and not `{for vnodes}`.
                // This should be converted into a if let guard once https://github.com/rust-lang/rust/issues/51114 is stable.
                // Or further nested once deref pattern (https://github.com/rust-lang/rust/issues/87121) is stable.
                if let HtmlBlock {
                    content: BlockContent::Node(children),
                    deprecations,
                    ..
                } = m.as_ref()
                {
                    if deprecations.is_empty() {
                        Some(quote! { #children })
                    } else {
                        Some(quote! { { #deprecations #children } })
                    }
                } else {
                    Some(quote! { ::yew::html::ChildrenRenderer::new(#self) })
                }
            }
            _ => Some(quote! { ::yew::html::ChildrenRenderer::new(#self) }),
        }
    }

    pub fn to_vnode_tokens(&self) -> TokenStream {
        match self.0[..] {
            [] => quote! {::std::default::Default::default() },
            [HtmlTree::Component(ref children)] => {
                quote! { ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(#children) }
            }
            [HtmlTree::Element(ref children)] => {
                quote! { ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(#children) }
            }
            [HtmlTree::Block(ref m)] => {
                // We only want to process `{vnode}` and not `{for vnodes}`.
                // This should be converted into a if let guard once https://github.com/rust-lang/rust/issues/51114 is stable.
                // Or further nested once deref pattern (https://github.com/rust-lang/rust/issues/87121) is stable.
                if let HtmlBlock {
                    content: BlockContent::Node(children),
                    deprecations,
                    ..
                } = m.as_ref()
                {
                    if deprecations.is_empty() {
                        quote! { ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(#children) }
                    } else {
                        quote! {
                            ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(
                                { #deprecations #children }
                            )
                        }
                    }
                } else {
                    quote! {
                        ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(
                            ::yew::html::ChildrenRenderer::new(#self)
                        )
                    }
                }
            }
            _ => quote! {
                ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(
                    ::yew::html::ChildrenRenderer::new(#self)
                )
            },
        }
    }

    pub fn size_hint(&self) -> Option<usize> {
        self.only_single_node_children().then(|| {
            self.0
                .iter()
                .filter(|c| {
                    !matches!(
                        c,
                        HtmlTree::Break(_) | HtmlTree::Continue(_) | HtmlTree::Return(_)
                    )
                })
                .count()
        })
    }

    pub fn fully_keyed(&self) -> Option<bool> {
        for child in self.0.iter() {
            match child {
                HtmlTree::Block(block) => {
                    return match &block.content {
                        BlockContent::Node(node) => {
                            matches!(&**node, HtmlNode::Literal(_)).then_some(false)
                        }
                        _ => None,
                    };
                }
                HtmlTree::Component(comp) => {
                    if comp.props.props.special.key.is_none() {
                        return Some(false);
                    }
                }
                HtmlTree::List(list) => {
                    if list.open.props.key.is_none() {
                        return Some(false);
                    }
                }
                HtmlTree::Element(element) => {
                    if element.props.special.key.is_none() {
                        return Some(false);
                    }
                }
                HtmlTree::Node(node) => {
                    return match node.as_ref() {
                        HtmlNode::Literal(_) => Some(false),
                        HtmlNode::Expression(_) => None,
                    };
                }
                HtmlTree::If(_)
                | HtmlTree::For(_)
                | HtmlTree::While(_)
                | HtmlTree::Match(_)
                | HtmlTree::Break(_)
                | HtmlTree::Continue(_)
                | HtmlTree::Return(_)
                | HtmlTree::Empty => {
                    return Some(false);
                }
            }
        }
        Some(true)
    }
}

impl ToTokens for HtmlChildrenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_build_vec_token_stream());
    }
}

pub struct HtmlRootBraced {
    brace: token::Brace,
    stmts: Vec<syn::Stmt>,
    children: HtmlChildrenTree,
    deprecations: TokenStream,
}

impl PeekValue<()> for HtmlRootBraced {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlRootBraced {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = braced!(content in input);

        let stmts = parse_preamble_stmts(&content)?;

        let children = HtmlChildrenTree::parse_delimited_with_nodes(&content)?;
        let deprecations = check_unnecessary_fragment(&children);

        Ok(HtmlRootBraced {
            brace,
            stmts,
            children,
            deprecations,
        })
    }
}

/// Parse leading Rust statements as a preamble: `let` bindings, items, macro
/// invocations terminated by `;`, and expression statements terminated by `;`
/// (including `break`/`continue`/`return`, with or without labels).
///
/// Bare expressions (no trailing `;`) are left in the stream for html parsing.
/// A forked parse is used to test each candidate statement without committing
/// to it, which lets us fall through cleanly whenever the next token run is
/// not Rust-parseable (e.g. an `<element/>` or `if cond { <node/> }` that the
/// Rust expression grammar rejects).
///
/// `for`, `while`, and `loop` auto-terminate as statements in Rust grammar, so
/// in preamble position they parse as `Stmt::Expr(_, None)`. When the fork
/// succeeds at parsing one as a full Rust `Stmt`, the entire expression
/// (header, body, all nested calls) was Rust-parseable, which means it
/// contains no html elements and cannot be a misread of html-control-flow.
/// We accept it as a preamble statement so users can write side-effect loops
/// without a trailing `;` or a `let _ =` binding.
///
/// `if` and `match` are handled separately by the html-control-flow parser
/// because their bodies routinely contain VNode-convertible expressions.
/// Bare `{...}` blocks stay as html-block-as-child (`{ render(item) }`).
pub(super) fn parse_preamble_stmts(input: ParseStream) -> syn::Result<Vec<syn::Stmt>> {
    let mut stmts = Vec::new();
    loop {
        let fork = input.fork();
        let is_preamble = match fork.parse::<syn::Stmt>() {
            Ok(syn::Stmt::Local(_)) => true,
            Ok(syn::Stmt::Item(_)) => true,
            Ok(syn::Stmt::Expr(_, Some(_))) => true,
            Ok(syn::Stmt::Macro(m)) => m.semi_token.is_some(),
            Ok(syn::Stmt::Expr(expr, None)) => is_imperative_blocklike(&expr),
            _ => false,
        };
        if !is_preamble {
            break;
        }
        let stmt: syn::Stmt = input.parse()?;
        stmts.push(stmt);
    }
    Ok(stmts)
}

/// Whether `expr` is an imperative block-like Rust expression (`for`, `while`,
/// `loop`) that should be accepted as a preamble statement when it appears
/// without a trailing `;`.
fn is_imperative_blocklike(expr: &syn::Expr) -> bool {
    matches!(
        expr,
        syn::Expr::ForLoop(_) | syn::Expr::While(_) | syn::Expr::Loop(_)
    )
}

/// Parse `break [label] [value]` forgivingly: first try syn's full
/// `ExprBreak::parse` (via a fork so we don't commit a bad state), and if that
/// fails because the value position starts with `<` (html) or another
/// non-expression token, fall back to the keyword + optional lifetime label.
/// This lets `break` / `break 'outer` / `break val` / `break 'outer val` all
/// work without also eating html children.
pub(super) fn parse_break(input: syn::parse::ParseStream) -> syn::Result<syn::ExprBreak> {
    let fork = input.fork();
    if let Ok(expr) = fork.parse::<syn::ExprBreak>() {
        input.advance_to(&fork);
        return Ok(expr);
    }
    let break_token: Token![break] = input.parse()?;
    let label: Option<syn::Lifetime> = if input.peek(syn::Lifetime) {
        Some(input.parse()?)
    } else {
        None
    };
    Ok(syn::ExprBreak {
        attrs: Vec::new(),
        break_token,
        label,
        expr: None,
    })
}

/// Parse `continue [label]` with the same fork-and-fallback strategy as
/// [`parse_break`].
pub(super) fn parse_continue(input: syn::parse::ParseStream) -> syn::Result<syn::ExprContinue> {
    let fork = input.fork();
    if let Ok(expr) = fork.parse::<syn::ExprContinue>() {
        input.advance_to(&fork);
        return Ok(expr);
    }
    let continue_token: Token![continue] = input.parse()?;
    let label: Option<syn::Lifetime> = if input.peek(syn::Lifetime) {
        Some(input.parse()?)
    } else {
        None
    };
    Ok(syn::ExprContinue {
        attrs: Vec::new(),
        continue_token,
        label,
    })
}

/// Parse `return [value]` with the same fork-and-fallback strategy as
/// [`parse_break`]. The fallback emits a bare `return` (no value), leaving
/// anything that follows — notably `<span>` or other html — for the children
/// parser.
pub(super) fn parse_return(input: syn::parse::ParseStream) -> syn::Result<syn::ExprReturn> {
    let fork = input.fork();
    if let Ok(expr) = fork.parse::<syn::ExprReturn>() {
        input.advance_to(&fork);
        return Ok(expr);
    }
    let return_token: Token![return] = input.parse()?;
    Ok(syn::ExprReturn {
        attrs: Vec::new(),
        return_token,
        expr: None,
    })
}

/// Whether any statement is a top-level divergent expression (`break`,
/// `continue`, or `return`). Callers that emit code after the statements need
/// an `#[allow(unreachable_code)]` when this is true.
pub(super) fn stmts_have_divergent(stmts: &[syn::Stmt]) -> bool {
    stmts.iter().any(|stmt| {
        matches!(
            stmt,
            syn::Stmt::Expr(
                syn::Expr::Break(_) | syn::Expr::Continue(_) | syn::Expr::Return(_),
                _,
            )
        )
    })
}

pub(super) fn deprecated_call(span: Span, note: &str) -> TokenStream {
    quote_spanned! {span=>
        {
            #[deprecated = #note]
            fn __yew_deprecated() {}
            __yew_deprecated();
        }
    }
}

/// Lint when a braced body contains a single keyless fragment, since the children
/// can be placed directly in the body without the `<>...</>` wrapper.
pub(super) fn check_unnecessary_fragment(children: &HtmlChildrenTree) -> TokenStream {
    let [HtmlTree::List(list)] = &children.0[..] else {
        return TokenStream::new();
    };
    if list.open.props.key.is_some() {
        return TokenStream::new();
    }
    deprecated_call(
        list.open_spanned().to_token_stream().span(),
        "unnecessary `<>...</>`. Children can be placed directly in the body",
    )
}

impl ToTokens for HtmlRootBraced {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            brace,
            stmts,
            children,
            deprecations,
        } = self;

        // Inner attributes are rejected on if-branch and match-arm blocks
        // directly, so we always nest in an inner expression block. The inner
        // attribute goes on that inner block, which Rust accepts everywhere.
        let allow_unreachable =
            stmts_have_divergent(stmts).then(|| quote!(#![allow(unreachable_code)]));
        tokens.extend(quote_spanned! {brace.span.span()=>
            {
                #deprecations
                {
                    #allow_unreachable
                    #(#stmts)*
                    ::yew::virtual_dom::VNode::VList(::std::rc::Rc::new(
                        ::yew::virtual_dom::VList::with_children(
                            #children, ::std::option::Option::None
                        )
                    ))
                }
            }
        });
    }
}
