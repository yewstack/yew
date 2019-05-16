use super::HtmlTree;
use crate::Peek;
use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::token;

pub struct HtmlList(pub Vec<HtmlTree>);

impl Peek<()> for HtmlList {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlListOpen::peek(cursor)
    }
}

impl Parse for HtmlList {
    fn parse(input: ParseStream) -> Result<Self> {
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
        while let Ok(html_tree) = input.parse::<HtmlTree>() {
            children.push(html_tree);
        }

        input.parse::<HtmlListClose>()?;

        Ok(HtmlList(children))
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlList(html_trees) = self;
        let html_trees = html_trees.iter().map(|html_tree| quote! { #html_tree });
        tokens.extend(quote! {
            ::yew_html_common::html_tree::html_list::HtmlList(
                vec![#(#html_trees,)*]
            )
        });
    }
}

struct HtmlListOpen {
    lt_token: token::Lt,
    gt_token: token::Gt,
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
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(HtmlListOpen {
            lt_token: input.parse()?,
            gt_token: input.parse()?,
        })
    }
}

impl ToTokens for HtmlListOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlListOpen { lt_token, gt_token } = self;
        tokens.extend(quote! {#lt_token#gt_token});
    }
}

struct HtmlListClose {}

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
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<token::Lt>()?;
        input.parse::<token::Div>()?;
        input.parse::<token::Gt>()?;
        Ok(HtmlListClose {})
    }
}
