use super::{html_dashed_name::HtmlDashedName, HtmlChildrenTree};
use crate::html_tree::{HtmlProp, HtmlPropSuffix};
use crate::{Peek, PeekValue};
use boolinator::Boolinator;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Token};

pub struct HtmlList {
    open: HtmlListOpen,
    children: HtmlChildrenTree,
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

        let close = input.parse::<HtmlListClose>()?;

        Ok(HtmlList {
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
            quote_spanned! {key.span()=> Some(::std::convert::Into::<::yew::virtual_dom::Key>::into(#key))}
        } else {
            quote! {None}
        };

        let open_close_tokens = quote! {#open#close};
        tokens.extend(quote_spanned! {open_close_tokens.span()=>
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
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

            prop.ensure_not_optional()?;

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
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
