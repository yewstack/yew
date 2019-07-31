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
use quote::ToTokens;
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
    Block(HtmlBlock),
    Component(HtmlComponent),
    Iterable(HtmlIterable),
    List(HtmlList),
    Tag(HtmlTag),
    Node(HtmlNode),
    Empty,
}

pub struct HtmlRoot(HtmlTree);
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_root = if HtmlTree::peek(input.cursor()).is_some() {
            HtmlRoot(input.parse()?)
        } else if HtmlIterable::peek(input.cursor()).is_some() {
            HtmlRoot(HtmlTree::Iterable(input.parse()?))
        } else {
            HtmlRoot(HtmlTree::Node(input.parse()?))
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
            HtmlType::Component => HtmlTree::Component(input.parse()?),
            HtmlType::Tag => HtmlTree::Tag(input.parse()?),
            HtmlType::Block => HtmlTree::Block(input.parse()?),
            HtmlType::List => HtmlTree::List(input.parse()?),
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
        let empty_html_el = HtmlList(Vec::new());
        let html_tree_el: &dyn ToTokens = match self {
            HtmlTree::Empty => &empty_html_el,
            HtmlTree::Component(comp) => comp,
            HtmlTree::Tag(tag) => tag,
            HtmlTree::List(list) => list,
            HtmlTree::Node(node) => node,
            HtmlTree::Iterable(iterable) => iterable,
            HtmlTree::Block(block) => block,
        };

        html_tree_el.to_tokens(tokens);
    }
}
