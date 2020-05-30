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
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HtmlTree::Empty => HtmlList::empty().to_tokens(tokens),
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    /// Check if the generated code will only create a single node.
    fn single_child(&self) -> bool;

    /// The generated code will produce a value that implements `IntoIter<Item = Into<VNode>>`.
    fn to_children_tokens(&self, tokens: &mut TokenStream);

    fn to_children_token_stream(&self) -> TokenStream {
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

    fn to_children_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HtmlTree::Block(block) => block.to_children_tokens(tokens),
            other => tokens.extend(quote_spanned! {other.span()=> ::std::iter::once(#other)}),
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

    fn only_single_children(&self) -> bool {
        self.0.iter().all(ToChildrenTokens::single_child)
    }

    pub fn to_build_vec_tokens(&self, tokens: &mut TokenStream) {
        if self.only_single_children() {
            let children = &self.0;
            tokens.extend(quote! {
                vec![#((#children).into()),*]
            });
            return;
        }

        let vec_ident = Ident::new("__yew_v", Span::call_site());
        let add_children_streams = (&self.0).iter().map(|child| {
            if child.single_child() {
                quote! {
                    #vec_ident.push((#child).into());
                }
            } else {
                let children_stream = child.to_children_token_stream();
                quote! {
                    #vec_ident.extend((#children_stream).into_iter().map(|n| n.into()));
                }
            }
        });

        tokens.extend(quote! {
            {
                let mut #vec_ident = ::std::vec::Vec::new();
                #(#add_children_streams)*
                #vec_ident
            }
        });
    }
}

impl ToTokens for HtmlChildrenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_build_vec_tokens(tokens);
    }
}
