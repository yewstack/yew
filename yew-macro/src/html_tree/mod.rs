pub mod html_block;
pub mod html_component;
pub mod html_dashed_name;
pub mod html_iterable;
pub mod html_list;
pub mod html_node;
pub mod html_prop;
pub mod html_tag;

use crate::PeekValue;
use html_block::HtmlBlock;
use html_component::HtmlComponent;
use html_dashed_name::HtmlDashedName;
use html_iterable::HtmlIterable;
use html_list::HtmlList;
use html_node::HtmlNode;
use html_prop::HtmlProp;
use html_prop::HtmlPropSuffix;
use html_tag::HtmlTag;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};

pub enum HtmlType {
    Block,
    Component,
    List,
    Tag,
    Empty,
}

pub enum HtmlTree {
    Block(Box<HtmlBlock>),
    Component(Box<HtmlComponent>),
    Iterable(Box<HtmlIterable>),
    List(Box<HtmlList>),
    Tag(Box<HtmlTag>),
    Node(Box<HtmlNode>),
    Empty,
}

pub struct HtmlRoot(HtmlTree);
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_root = if HtmlTree::peek(input.cursor()).is_some() {
            HtmlRoot(input.parse()?)
        } else if HtmlIterable::peek(input.cursor()).is_some() {
            HtmlRoot(HtmlTree::Iterable(Box::new(input.parse()?)))
        } else {
            HtmlRoot(HtmlTree::Node(Box::new(input.parse()?)))
        };

        if !input.is_empty() {
            let stream: TokenStream = input.parse()?;
            Err(syn::Error::new_spanned(
                stream,
                "only one root html element allowed",
            ))
        } else {
            Ok(html_root)
        }
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlRoot(html_tree) = self;
        html_tree.to_tokens(tokens);
    }
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_type = HtmlTree::peek(input.cursor())
            .ok_or_else(|| input.error("expected valid html element"))?;
        let html_tree = match html_type {
            HtmlType::Empty => HtmlTree::Empty,
            HtmlType::Component => HtmlTree::Component(Box::new(input.parse()?)),
            HtmlType::Tag => HtmlTree::Tag(Box::new(input.parse()?)),
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
        } else if HtmlComponent::peek(cursor).is_some() {
            Some(HtmlType::Component)
        } else if HtmlTag::peek(cursor).is_some() {
            Some(HtmlType::Tag)
        } else if HtmlBlock::peek(cursor).is_some() {
            Some(HtmlType::Block)
        } else if HtmlList::peek(cursor).is_some() {
            Some(HtmlType::List)
        } else {
            None
        }
    }
}

impl ToTokens for HtmlTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let node = self.token_stream();
        tokens.extend(quote! {
            ::yew::virtual_dom::VNode::from(#node)
        });
    }
}

impl HtmlTree {
    fn token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            HtmlTree::Empty => HtmlList {
                children: Vec::new(),
                key: None,
            }
            .into_token_stream(),
            HtmlTree::Component(comp) => comp.into_token_stream(),
            HtmlTree::Tag(tag) => tag.into_token_stream(),
            HtmlTree::List(list) => list.into_token_stream(),
            HtmlTree::Node(node) => node.into_token_stream(),
            HtmlTree::Iterable(iterable) => iterable.into_token_stream(),
            HtmlTree::Block(block) => block.into_token_stream(),
        }
    }
}

pub struct HtmlRootNested(HtmlTreeNested);
impl Parse for HtmlRootNested {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(HtmlRootNested(HtmlTreeNested::parse(input)?))
    }
}

impl ToTokens for HtmlRootNested {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub struct HtmlTreeNested(HtmlTree);
impl Parse for HtmlTreeNested {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(HtmlTreeNested(HtmlTree::parse(input)?))
    }
}

impl ToTokens for HtmlTreeNested {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.token_stream());
    }
}
