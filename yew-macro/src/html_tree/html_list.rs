use super::HtmlTree;
use crate::html_tree::{HtmlProp, HtmlPropSuffix};
use crate::PeekValue;
use boolinator::Boolinator;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, Token};

pub struct HtmlList {
    pub children: Vec<HtmlTree>,
    pub key: Option<Expr>,
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
                    "this close tag has no corresponding open tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlListOpen>()?;
        if !HtmlList::verify_end(input.cursor()) {
            return Err(syn::Error::new_spanned(
                open,
                "this open tag has no corresponding close tag",
            ));
        }

        let mut children: Vec<HtmlTree> = vec![];
        while HtmlListClose::peek(input.cursor()).is_none() {
            children.push(input.parse()?);
        }

        input.parse::<HtmlListClose>()?;

        Ok(HtmlList {
            children,
            key: open.key,
        })
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let children = &self.children;
        let key = if let Some(key) = &self.key {
            quote_spanned! {key.span() => Some(#key)}
        } else {
            quote! {None }
        };
        tokens.extend(quote! {
            ::yew::virtual_dom::VNode::VList(
                ::yew::virtual_dom::VList::new_with_children({
                    let mut v = ::std::vec::Vec::new();
                    #(v.extend(::yew::utils::NodeSeq::from(#children));)*
                    v
                },
                #key)
            )
        });
    }
}

impl HtmlList {
    fn verify_end(mut cursor: Cursor) -> bool {
        let mut list_stack_count = 1;
        loop {
            if HtmlListOpen::peek(cursor).is_some() {
                list_stack_count += 1;
            } else if HtmlListClose::peek(cursor).is_some() {
                list_stack_count -= 1;
                if list_stack_count == 0 {
                    break;
                }
            }
            if let Some((_, next)) = cursor.token_tree() {
                cursor = next;
            } else {
                break;
            }
        }

        list_stack_count == 0
    }
}

struct HtmlListOpen {
    lt: Token![<],
    key: Option<Expr>,
    gt: Token![>],
}

impl PeekValue<()> for HtmlListOpen {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        if let Some((ident, _)) = cursor.ident() {
            (ident == "key").as_option()
        } else {
            let (punct, _) = cursor.punct()?;
            (punct.as_char() == '>').as_option()
        }
    }
}

impl Parse for HtmlListOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse()?;
        if input.cursor().ident().is_some() {
            let HtmlPropSuffix { stream, gt, .. } = input.parse()?;
            let props = parse::<ParseKey>(stream)?;
            Ok(HtmlListOpen {
                lt,
                key: Some(props.key.value),
                gt,
            })
        } else {
            let gt = input.parse()?;
            Ok(HtmlListOpen { lt, key: None, gt })
        }
    }
}

struct ParseKey {
    key: HtmlProp,
}

impl Parse for ParseKey {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let key = input.parse::<HtmlProp>()?;
        if !input.is_empty() {
            input.error("Only a single key element is allowed on a <></>");
        }
        Ok(ParseKey { key })
    }
}

impl ToTokens for HtmlListOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlListOpen { lt, gt, .. } = self;
        tokens.extend(quote! {#lt#gt});
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
