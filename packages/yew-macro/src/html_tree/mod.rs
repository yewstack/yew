use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, token, Token};

use crate::{is_ide_completion, PeekValue};

mod html_block;
mod html_component;
mod html_dashed_name;
mod html_element;
mod html_if;
mod html_iterable;
mod html_list;
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

pub enum HtmlType {
    Block,
    Component,
    List,
    Element,
    If,
    Empty,
}

pub enum HtmlTree {
    Block(Box<HtmlBlock>),
    Component(Box<HtmlComponent>),
    List(Box<HtmlList>),
    Element(Box<HtmlElement>),
    If(Box<HtmlIf>),
    Empty,
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html_type = Self::peek_html_type(input)
            .ok_or_else(|| input.error("expected a valid html element"))?;
        let html_tree = match html_type {
            HtmlType::Empty => HtmlTree::Empty,
            HtmlType::Component => HtmlTree::Component(Box::new(input.parse()?)),
            HtmlType::Element => HtmlTree::Element(Box::new(input.parse()?)),
            HtmlType::Block => HtmlTree::Block(Box::new(input.parse()?)),
            HtmlType::List => HtmlTree::List(Box::new(input.parse()?)),
            HtmlType::If => HtmlTree::If(Box::new(input.parse()?)),
        };
        Ok(html_tree)
    }
}

impl HtmlTree {
    /// Determine the [`HtmlType`] before actually parsing it.
    /// Even though this method accepts a [`ParseStream`], it is forked and the original stream is
    /// not modified. Once a certain `HtmlType` can be deduced for certain, the function eagerly
    /// returns with the appropriate type. If invalid html tag, returns `None`.
    fn peek_html_type(input: ParseStream) -> Option<HtmlType> {
        let input = input.fork(); // do not modify original ParseStream

        if input.is_empty() {
            Some(HtmlType::Empty)
        } else if input
            .cursor()
            .group(proc_macro2::Delimiter::Brace)
            .is_some()
        {
            Some(HtmlType::Block)
        } else if HtmlIf::peek(input.cursor()).is_some() {
            Some(HtmlType::If)
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
            HtmlTree::Empty => tokens.extend(quote! {
                <::yew::virtual_dom::VNode as ::std::default::Default>::default()
            }),
            HtmlTree::Component(comp) => comp.to_tokens(tokens),
            HtmlTree::Element(tag) => tag.to_tokens(tokens),
            HtmlTree::List(list) => list.to_tokens(tokens),
            HtmlTree::Block(block) => block.to_tokens(tokens),
            HtmlTree::If(block) => block.to_tokens(tokens),
        }
    }
}

pub enum HtmlRoot {
    Tree(HtmlTree),
    Iterable(Box<HtmlIterable>),
    Node(Box<HtmlNode>),
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let html_root = if HtmlTree::peek_html_type(input).is_some() {
            Self::Tree(input.parse()?)
        } else if HtmlIterable::peek(input.cursor()).is_some() {
            Self::Iterable(Box::new(input.parse()?))
        } else {
            Self::Node(Box::new(input.parse()?))
        };

        if !input.is_empty() {
            let stream: TokenStream = input.parse()?;
            Err(syn::Error::new_spanned(
                stream,
                "only one root html element is allowed (hint: you can wrap multiple html elements \
                 in a fragment `<></>`)",
            ))
        } else {
            Ok(html_root)
        }
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Tree(tree) => tree.to_tokens(tokens),
            Self::Node(node) => node.to_tokens(tokens),
            Self::Iterable(iterable) => iterable.to_tokens(tokens),
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
        tokens.extend(
            quote_spanned! {self.0.span().resolved_at(Span::mixed_site())=> {
                #[allow(clippy::useless_conversion)]
                <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(#new_tokens)
            }},
        );
    }
}

/// This trait represents a type that can be unfolded into multiple html nodes.
pub trait ToNodeIterator {
    /// Generate a token stream which produces a value that implements IntoIterator<Item=T> where T
    /// is inferred by the compiler. The easiest way to achieve this is to call `.into()` on
    /// each element. If the resulting iterator only ever yields a single item this function
    /// should return None instead.
    fn to_node_iterator_stream(&self) -> Option<TokenStream>;
}

impl ToNodeIterator for HtmlTree {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        match self {
            HtmlTree::Block(block) => block.to_node_iterator_stream(),
            // everything else is just a single node.
            _ => None,
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
        self.0
            .iter()
            .map(ToNodeIterator::to_node_iterator_stream)
            .all(|s| s.is_none())
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
                ::std::vec![#(#children_into),*]
            };
        }

        let vec_ident = Ident::new("__yew_v", Span::mixed_site());
        let add_children_streams = children.iter().map(|child| {
            if let Some(node_iterator_stream) = child.to_node_iterator_stream() {
                quote! {
                    ::std::iter::Extend::extend(&mut #vec_ident, #node_iterator_stream);
                }
            } else {
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

    fn parse_delimited(input: ParseStream) -> syn::Result<Self> {
        let mut children = HtmlChildrenTree::new();

        while !input.is_empty() {
            children.parse_child(input)?;
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
}

impl ToTokens for HtmlChildrenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_build_vec_token_stream());
    }
}

pub struct HtmlRootBraced {
    brace: token::Brace,
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
        let children = HtmlChildrenTree::parse_delimited(&content)?;

        Ok(HtmlRootBraced { brace, children })
    }
}

impl ToTokens for HtmlRootBraced {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { brace, children } = self;

        tokens.extend(quote_spanned! {brace.span.span()=>
            {
                ::yew::virtual_dom::VNode::VList(::std::rc::Rc::new(
                    ::yew::virtual_dom::VList::with_children(#children, ::std::option::Option::None)
                ))
            }
        });
    }
}
