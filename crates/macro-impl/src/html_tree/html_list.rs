use super::HtmlTree;
use crate::Peek;
use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::Token;

pub struct HtmlList(pub Vec<HtmlTree>);

impl Peek<()> for HtmlList {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlListOpen::peek(cursor)
            .or(HtmlListClose::peek(cursor))
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

        let mut cursor = input.cursor();
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

        if list_stack_count > 0 {
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

        Ok(HtmlList(children))
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let html_trees = &self.0;
        tokens.extend(quote! {
            $crate::virtual_dom::vlist::VList {
                childs: vec![#(#html_trees,)*],
            }
        });
    }
}

struct HtmlListOpen {
    lt: Token![<],
    gt: Token![>],
}

impl Peek<()> for HtmlListOpen {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()
    }
}

impl Parse for HtmlListOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlListOpen {
            lt: input.parse()?,
            gt: input.parse()?,
        })
    }
}

impl ToTokens for HtmlListOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlListOpen { lt, gt } = self;
        tokens.extend(quote! {#lt#gt});
    }
}

struct HtmlListClose {
    lt: Token![<],
    div: Token![/],
    gt: Token![>],
}

impl Peek<()> for HtmlListClose {
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
