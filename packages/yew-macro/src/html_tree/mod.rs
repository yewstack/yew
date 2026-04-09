macro_rules! emit_deprecated {
    ($($tt:tt)*) => {{
        #[cfg(yew_macro_nightly)]
        proc_macro_error::emit_warning!($($tt)*);
        #[cfg(not(yew_macro_nightly))]
        proc_macro_error::emit_error!($($tt)*);
    }};
}
pub(crate) use emit_deprecated;
use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::buffer::Cursor;
use syn::ext::IdentExt;
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
mod html_match;
mod html_node;
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

pub enum HtmlType {
    Block,
    Component,
    List,
    Element,
    If,
    For,
    Match,
    Empty,
}

pub enum HtmlTree {
    Block(Box<HtmlBlock>),
    Component(Box<HtmlComponent>),
    List(Box<HtmlList>),
    Element(Box<HtmlElement>),
    If(Box<HtmlIf>),
    For(Box<HtmlFor>),
    Match(Box<HtmlMatch>),
    Node(Box<HtmlNode>),
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
            HtmlType::Match => Self::Match(Box::new(input.parse()?)),
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
        } else if HtmlMatch::peek(cursor).is_some() {
            Some(HtmlType::Match)
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
            Self::Match(block) => block.to_tokens(tokens),
            Self::Node(node) => node.to_tokens(tokens),
        }
    }
}

pub struct HtmlRoot {
    children: HtmlChildrenTree,
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let children = HtmlChildrenTree::parse_delimited_with_nodes(input)?;
        check_unnecessary_fragment(&children);
        Ok(Self { children })
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.children.0[..] {
            [] => tokens.extend(quote! {
                <::yew::virtual_dom::VNode as ::std::default::Default>::default()
            }),
            [single] => single.to_tokens(tokens),
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
                    ::yew::virtual_dom::VNode::VList(::std::rc::Rc::new(#vlist))
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

        if self.only_single_node_children() {
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
        let add_children_streams =
            children
                .iter()
                .map(|child| match child.to_node_iterator_stream() {
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
                });

        quote! {
            {
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
                    ..
                } = m.as_ref()
                {
                    Some(quote! { #children })
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
                    ..
                } = m.as_ref()
                {
                    quote! { ::yew::html::IntoPropValue::<::yew::virtual_dom::VNode>::into_prop_value(#children) }
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
        self.only_single_node_children().then_some(self.0.len())
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
                HtmlTree::If(_) | HtmlTree::For(_) | HtmlTree::Match(_) | HtmlTree::Empty => {
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
    let_stmts: Vec<syn::Local>,
    children: HtmlChildrenTree,
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

        let mut let_stmts = Vec::new();
        while content.peek(Token![let]) {
            let stmt: syn::Stmt = content.parse()?;
            match stmt {
                syn::Stmt::Local(local) => let_stmts.push(local),
                _ => unreachable!("peeked Token![let] but parsed non-local statement"),
            }
        }

        let children = HtmlChildrenTree::parse_delimited_with_nodes(&content)?;
        check_unnecessary_fragment(&children);

        Ok(HtmlRootBraced {
            brace,
            let_stmts,
            children,
        })
    }
}

/// Lint when a braced body contains a single keyless fragment, since the children
/// can be placed directly in the body without the `<>...</>` wrapper.
pub(super) fn check_unnecessary_fragment(children: &HtmlChildrenTree) {
    if let [HtmlTree::List(list)] = &children.0[..] {
        if list.open.props.key.is_none() {
            emit_deprecated!(
                list.open_spanned(),
                "unnecessary `<>...</>`. Children can be placed directly in the body"
            );
        }
    }
}

impl ToTokens for HtmlRootBraced {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            brace,
            let_stmts,
            children,
        } = self;

        tokens.extend(quote_spanned! {brace.span.span()=>
            {
                #(#let_stmts)*
                ::yew::virtual_dom::VNode::VList(::std::rc::Rc::new(
                    ::yew::virtual_dom::VList::with_children(#children, ::std::option::Option::None)
                ))
            }
        });
    }
}
