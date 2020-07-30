use super::{html_dashed_name::HtmlDashedName, HtmlChildrenTree};
use crate::html_tree::{HtmlProp, HtmlPropSuffix};
use crate::{Peek, PeekValue};
use boolinator::Boolinator;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, Token};

pub struct HtmlList {
    children: HtmlChildrenTree,
    key: Option<Expr>,
}

impl HtmlList {
    pub fn empty() -> Self {
        Self {
            children: HtmlChildrenTree::new(),
            key: None,
        }
    }
}

impl PeekValue<()> for HtmlList {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlListOpen::peek(cursor)
            .or_else(|| HtmlListClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlList {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if HtmlListClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlListClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close,
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
                    open,
                    "this opening fragment has no corresponding closing fragment",
                ));
            }
        }

        input.parse::<HtmlListClose>()?;

        Ok(HtmlList {
            children,
            key: open.props.key,
        })
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let children = &self.children;
        let key = if let Some(key) = &self.key {
            quote_spanned! {key.span()=> Some(::yew::virtual_dom::Key::from(#key))}
        } else {
            quote! {None}
        };
        tokens.extend(quote! {
            ::yew::virtual_dom::VNode::VList(
                ::yew::virtual_dom::VList::new_with_children(#children, #key)
            )
        });
    }
}

struct HtmlListOpen {
    lt: Token![<],
    props: HtmlListProps,
    gt: Token![>],
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse()?;
        let HtmlPropSuffix { stream, gt, .. } = input.parse()?;
        let props = syn::parse2(stream)?;
        Ok(Self { lt, props, gt })
    }
}

impl ToTokens for HtmlListOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlListOpen { lt, gt, .. } = self;
        tokens.extend(quote! {#lt#gt});
    }
}

struct HtmlListProps {
    key: Option<Expr>,
}
impl Parse for HtmlListProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let key = if input.is_empty() {
            None
        } else {
            let prop: HtmlProp = input.parse()?;
            if !input.is_empty() {
                return Err(input.error("only a single `key` prop is allowed on a fragment"));
            }

            if prop.label.to_ascii_lowercase_string() != "key" {
                return Err(syn::Error::new_spanned(
                    prop.label,
                    "fragments only accept the `key` prop",
                ));
            }

            if prop.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    prop.label,
                    "the 'key' attribute does not support being used as an optional attribute",
                ));
            }

            Some(prop.value)
        };

        Ok(Self { key })
    }
}

struct HtmlListClose {
    lt: Token![<],
    div: Token![/],
    gt: Token![>],
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlListClose {
            lt: input.parse()?,
            div: input.parse()?,
            gt: input.parse()?,
        })
    }
}

impl ToTokens for HtmlListClose {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlListClose { lt, div, gt } = self;
        tokens.extend(quote! {#lt#div#gt});
    }
}
