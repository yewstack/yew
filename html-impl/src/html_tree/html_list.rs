use super::HtmlTree;
use crate::Peek;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::token;

struct HtmlListChildren(Vec<HtmlTree>);
pub struct HtmlList {
    children: HtmlListChildren,
}

struct HtmlListOpen {
    lt_token: token::Lt,
    gt_token: token::Gt,
}

impl Peek for HtmlListOpen {
    fn peek(input: &ParseStream) -> bool {
        input.peek(token::Lt) && input.peek2(token::Gt)
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

impl Parse for HtmlListClose {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<token::Lt>()?;
        input.parse::<token::Div>()?;
        input.parse::<token::Gt>()?;
        Ok(HtmlListClose {})
    }
}

impl Parse for HtmlListChildren {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut children: Vec<HtmlTree> = vec![];
        while !input.is_empty() {
            children.push(input.parse::<HtmlTree>()?);
        }

        Ok(HtmlListChildren(children))
    }
}

impl Peek for HtmlList {
    fn peek(input: &ParseStream) -> bool {
        HtmlListOpen::peek(input)
    }
}

impl Parse for HtmlList {
    fn parse(input: ParseStream) -> Result<Self> {
        let open = input.parse::<HtmlListOpen>()?;

        let mut content: Vec<TokenTree> = vec![];
        while !input.is_empty() {
            content.push(input.parse::<TokenTree>()?);
        }

        let split = if content.len() < 3 {
            0
        } else {
            content.len() - 3
        };

        let last_tokens = content.split_off(split);
        let token_stream: proc_macro2::TokenStream = last_tokens.into_iter().collect();
        syn::parse::<HtmlListClose>(token_stream.into()).map_err(|_| {
            syn::Error::new_spanned(open, "this open tag has no corresponding close tag")
        })?;

        let token_stream: proc_macro2::TokenStream = content.into_iter().collect();
        let children = syn::parse::<HtmlListChildren>(token_stream.into())?;

        Ok(HtmlList { children })
    }
}
