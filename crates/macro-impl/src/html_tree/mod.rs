pub mod html_block;
pub mod html_component;
pub mod html_iterable;
pub mod html_list;
pub mod html_node;
pub mod html_prop;
pub mod html_tag;

use crate::Peek;
use html_block::HtmlBlock;
use html_component::HtmlComponent;
use html_iterable::HtmlIterable;
use html_list::HtmlList;
use html_node::HtmlNode;
use html_prop::HtmlProp;
use html_prop::HtmlPropSuffix;
use html_tag::HtmlTag;
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
            Err(input.error("only one root html element allowed"))
        } else {
            Ok(html_root)
        }
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlRoot(html_tree) = self;
        tokens.extend(quote! { #html_tree });
    }
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_type =
            HtmlTree::peek(input.cursor()).ok_or(input.error("expected valid html element"))?;
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

impl Peek<HtmlType> for HtmlTree {
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
        let token_stream = match self {
            HtmlTree::Empty => quote! {
                $crate::virtual_dom::VNode::VList(
                    $crate::virtual_dom::vlist::VList::new()
                )
            },
            HtmlTree::Component(comp) => quote! {
                $crate::virtual_dom::VNode::VComp(#comp)
            },
            HtmlTree::Tag(tag) => quote! {
                $crate::virtual_dom::VNode::VTag(#tag)
            },
            HtmlTree::List(list) => quote! {
                $crate::virtual_dom::VNode::VList(#list)
            },
            HtmlTree::Node(node) => quote! {#node},
            HtmlTree::Iterable(iterable) => quote! {#iterable},
            HtmlTree::Block(block) => quote! {#block},
        };

        tokens.extend(token_stream);
    }
}
