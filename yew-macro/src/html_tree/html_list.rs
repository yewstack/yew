use super::HtmlChildrenTree;
use crate::html_tree::HtmlPropSuffix;
use crate::PeekValue;
use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::Token;

pub struct HtmlList {
    children: HtmlChildrenTree,
}

impl HtmlList {
    pub fn empty() -> Self {
        Self {
            children: HtmlChildrenTree::new(),
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
        if !HtmlList::verify_end(input.cursor()) {
            return Err(syn::Error::new_spanned(
                open,
                "this opening fragment has no corresponding closing fragment",
            ));
        }

        let mut children = HtmlChildrenTree::new();
        while HtmlListClose::peek(input.cursor()).is_none() {
            children.parse_child(input)?;
        }

        input.parse::<HtmlListClose>()?;

        Ok(HtmlList { children })
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let children = &self.children;
        tokens.extend(quote! {
            ::yew::virtual_dom::VNode::VList(
                ::yew::virtual_dom::VList::new_with_children(#children)
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
            let HtmlPropSuffix { gt, .. } = input.parse()?;
            Ok(HtmlListOpen { lt, gt })
        } else {
            let gt = input.parse()?;
            Ok(HtmlListOpen { lt, gt })
        }
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
