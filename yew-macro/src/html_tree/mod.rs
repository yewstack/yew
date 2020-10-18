use crate::PeekValue;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

mod html_block;
mod html_component;
mod html_dashed_name;
mod html_element;
mod html_iterable;
mod html_list;
mod html_node;
mod tag;

use html_block::HtmlBlock;
use html_component::HtmlComponent;
pub use html_dashed_name::HtmlDashedName;
use html_element::HtmlElement;
use html_iterable::HtmlIterable;
use html_list::HtmlList;
use html_node::HtmlNode;
use tag::TagTokens;

pub enum HtmlType {
    Block,
    Component,
    List,
    Element,
    Empty,
}

pub enum HtmlTree {
    Block(Box<HtmlBlock>),
    Component(Box<HtmlComponent>),
    List(Box<HtmlList>),
    Element(Box<HtmlElement>),
    Empty,
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_type = HtmlTree::peek(input.cursor())
            .ok_or_else(|| input.error("expected a valid html element"))?;
        let html_tree = match html_type {
            HtmlType::Empty => HtmlTree::Empty,
            HtmlType::Component => HtmlTree::Component(Box::new(input.parse()?)),
            HtmlType::Element => HtmlTree::Element(Box::new(input.parse()?)),
            HtmlType::Block => HtmlTree::Block(Box::new(input.parse()?)),
            HtmlType::List => HtmlTree::List(Box::new(input.parse()?)),
        };
        Ok(html_tree)
    }
}

impl PeekValue<HtmlType> for HtmlTree {
    fn peek(cursor: Cursor) -> Option<HtmlType> {
        if cursor.eof() {
            Some(HtmlType::Empty)
        } else if HtmlList::peek(cursor).is_some() {
            Some(HtmlType::List)
        } else if HtmlComponent::peek(cursor).is_some() {
            Some(HtmlType::Component)
        } else if HtmlElement::peek(cursor).is_some() {
            Some(HtmlType::Element)
        } else if HtmlBlock::peek(cursor).is_some() {
            Some(HtmlType::Block)
        } else {
            None
        }
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HtmlTree::Empty => tokens.extend(quote! {
                ::yew::virtual_dom::VNode::VList(::yew::virtual_dom::VList::new())
            }),
            HtmlTree::Component(comp) => comp.to_tokens(tokens),
            HtmlTree::Element(tag) => tag.to_tokens(tokens),
            HtmlTree::List(list) => list.to_tokens(tokens),
            HtmlTree::Block(block) => block.to_tokens(tokens),
        }
    }
}

pub enum HtmlRoot {
    Tree(HtmlTree),
    Iterable(Box<HtmlIterable>),
    Node(Box<HtmlNode>),
}

impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_root = if HtmlTree::peek(input.cursor()).is_some() {
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
                "only one root html element is allowed (hint: you can wrap multiple html elements in a fragment `<></>`)",
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
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse().map(Self)
    }
}
impl ToTokens for HtmlRootVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_tokens = self.0.to_token_stream();
        tokens.extend(quote! {{
            #[allow(clippy::useless_conversion, unused_braces)]
            ::yew::virtual_dom::VNode::from(#new_tokens)
        }});
    }
}

/// This trait represents a type that can be unfolded into multiple html nodes.
pub trait ToNodeIterator {
    /// Generate a token stream which produces a value that implements IntoIterator<Item=T> where T is inferred by the compiler.
    /// The easiest way to achieve this is to call `.into()` on each element.
    /// If the resulting iterator only ever yields a single item this function should return None instead.
    fn to_node_iterator_stream(&self) -> Option<TokenStream>;
}

impl ToNodeIterator for HtmlTree {
    fn to_node_iterator_stream(&self) -> Option<TokenStream> {
        match self {
            HtmlTree::Block(block) => block.to_node_iterator_stream(),
            // everthing else is just a single node.
            _ => None,
        }
    }
}

struct HtmlChildrenTree(Vec<HtmlTree>);

impl HtmlChildrenTree {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn parse_child(&mut self, input: ParseStream) -> Result<()> {
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
            // optimize for the common case where all children are single nodes (only using literal html).
            let children_into = children
                .iter()
                .map(|child| quote_spanned! {child.span()=> ::std::convert::Into::into(#child) });
            return quote! {
                vec![#(#children_into),*]
            };
        }

        let vec_ident = Ident::new("__yew_v", Span::call_site());
        let add_children_streams = children.iter().map(|child| {
            if let Some(node_iterator_stream) = child.to_node_iterator_stream() {
                quote! {
                    #vec_ident.extend(#node_iterator_stream);
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
}

impl ToTokens for HtmlChildrenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_build_vec_token_stream());
    }
}
