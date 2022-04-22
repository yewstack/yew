use boolinator::Boolinator;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Expr;

use super::html_dashed_name::HtmlDashedName;
use super::{HtmlChildrenTree, TagTokens};
use crate::props::Prop;
use crate::{Peek, PeekValue};

pub struct HtmlList {
    open: HtmlListOpen,
    pub children: HtmlChildrenTree,
    close: HtmlListClose,
}

impl PeekValue<()> for HtmlList {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlListOpen::peek(cursor)
            .or_else(|| HtmlListClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlListClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlListClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close.to_spanned(),
                    "this closing fragment has no corresponding opening fragment",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlListOpen>()?;
        let mut children = HtmlChildrenTree::new();
        while HtmlListClose::peek(input.cursor()).is_none() {
            children.parse_child(input)?;
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open.to_spanned(),
                    "this opening fragment has no corresponding closing fragment",
                ));
            }
        }

        let close = input.parse::<HtmlListClose>()?;

        Ok(Self {
            open,
            children,
            close,
        })
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            open,
            children,
            close,
        } = &self;

        let key = if let Some(key) = &open.props.key {
            quote_spanned! {key.span()=> ::std::option::Option::Some(::std::convert::Into::<::yew::virtual_dom::Key>::into(#key))}
        } else {
            quote! { ::std::option::Option::None }
        };

        let spanned = {
            let open = open.to_spanned();
            let close = close.to_spanned();
            quote! { #open #close }
        };

        tokens.extend(quote_spanned! {spanned.span()=>
            ::yew::virtual_dom::VNode::VList(
                ::yew::virtual_dom::VList::with_children(#children, #key)
            )
        });
    }
}

struct HtmlListOpen {
    tag: TagTokens,
    props: HtmlListProps,
}
impl HtmlListOpen {
    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<()> for HtmlListOpen {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        // make sure it's either a property (key=value) or it's immediately closed
        if let Some((_, cursor)) = HtmlDashedName::peek(cursor) {
            let (punct, _) = cursor.punct()?;
            (punct.as_char() == '=' || punct.as_char() == '?').as_option()
        } else {
            let (punct, _) = cursor.punct()?;
            (punct.as_char() == '>').as_option()
        }
    }
}

impl Parse for HtmlListOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            let props = input.parse()?;
            Ok(Self { tag, props })
        })
    }
}

struct HtmlListProps {
    key: Option<Expr>,
}
impl Parse for HtmlListProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = if input.is_empty() {
            None
        } else {
            let prop: Prop = input.parse()?;
            if !input.is_empty() {
                return Err(input.error("only a single `key` prop is allowed on a fragment"));
            }

            if prop.label.to_ascii_lowercase_string() != "key" {
                return Err(syn::Error::new_spanned(
                    prop.label,
                    "fragments only accept the `key` prop",
                ));
            }

            Some(prop.value)
        };

        Ok(Self { key })
    }
}

struct HtmlListClose(TagTokens);
impl HtmlListClose {
    fn to_spanned(&self) -> impl ToTokens {
        self.0.to_spanned()
    }
}
impl PeekValue<()> for HtmlListClose {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()
    }
}
impl Parse for HtmlListClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            if !input.is_empty() {
                Err(input.error("unexpected content in list close"))
            } else {
                Ok(Self(tag))
            }
        })
    }
}
