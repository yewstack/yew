mod html_block;
mod html_component;
mod html_dashed_name;
mod html_iterable;
mod html_list;
mod html_node;
mod html_prop;
mod html_tag;

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
    List(Box<HtmlList>),
    Tag(Box<HtmlTag>),
    Empty,
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_type = HtmlTree::peek(input.cursor())
            .ok_or_else(|| input.error("expected a valid html element"))?;
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
        match self {
            HtmlTree::Empty => HtmlList {
                children: Vec::new(),
                key: None,
            }
            .to_tokens(tokens),
            HtmlTree::Component(comp) => comp.to_tokens(tokens),
            HtmlTree::Tag(tag) => tag.to_tokens(tokens),
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
            HtmlRoot::Tree(input.parse()?)
        } else if HtmlIterable::peek(input.cursor()).is_some() {
            HtmlRoot::Iterable(Box::new(input.parse()?))
        } else {
            HtmlRoot::Node(Box::new(input.parse()?))
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let new_tokens = match self {
            HtmlRoot::Tree(tree) => tree.to_token_stream(),
            HtmlRoot::Node(node) => node.to_token_stream(),
            HtmlRoot::Iterable(iterable) => iterable.to_token_stream(),
        };
        tokens.extend(quote! {
            ::yew::virtual_dom::VNode::from(#new_tokens)
        });
    }
}

pub trait ToChildrenTokens {
    fn single_child(&self) -> bool;

    fn to_children_tokens(&self, tokens: &mut proc_macro2::TokenStream);

    fn to_children_token_stream(&self) -> proc_macro2::TokenStream {
        let mut tokens = TokenStream::new();
        self.to_children_tokens(&mut tokens);
        tokens
    }
}

impl ToChildrenTokens for HtmlTree {
    fn single_child(&self) -> bool {
        match self {
            HtmlTree::Block(block) => block.single_child(),
            _ => true,
        }
    }

    fn to_children_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlTree::Block(block) => block.to_children_tokens(tokens),
            other => tokens.extend(quote! {vec![#other]}),
        }
    }
}

struct HtmlChildrenTree(Vec<HtmlTree>);

impl HtmlChildrenTree {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn parse_child(&mut self, input: ParseStream) -> Result<()> {
        self.0.push(input.parse()?);
        Ok(())
    }

    fn only_single_children(&self) -> bool {
        self.0.iter().all(ToChildrenTokens::single_child)
    }
}

impl ToTokens for HtmlChildrenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let children = &self.0;
        if self.only_single_children() {
            tokens.extend(quote! {vec![#(#children),*]});
        } else {
            let mut children = children.iter().map(|c| c.to_children_token_stream());
            // can't fail because otherwise 'only_single_children' would be true.
            let first = children.next().unwrap();
            tokens.extend(quote! {
                (#first).into_iter()#(.chain(#children))*
            })
        }
    }
}
