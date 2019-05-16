use super::HtmlTree;
use crate::Peek;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::token;

pub struct HtmlListChildren(pub Vec<HtmlTree>);
impl ToTokens for HtmlListChildren {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlListChildren(html_trees) = self;
        let html_trees = html_trees.iter().map(|html_tree| quote! { #html_tree });
        tokens.extend(quote! {
            ::yew_html_common::html_tree::html_list::HtmlListChildren(vec![#(#html_trees,)*])
        });
    }
}

pub struct HtmlList {
    pub children: HtmlListChildren,
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

impl Peek for HtmlListClose {
    fn peek(input: &ParseStream) -> bool {
        input.peek(token::Lt) && input.peek2(token::Div) && input.peek3(token::Gt)
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
        let mut list_stack_count = 0;
        while !input.is_empty() {
            if HtmlListOpen::peek(&input) {
                list_stack_count += 1;
            } else if HtmlListClose::peek(&input) {
                if list_stack_count == 0 {
                    break;
                } else {
                    list_stack_count -= 1;
                }
            }
            content.push(input.parse::<TokenTree>()?);
        }

        input.parse::<HtmlListClose>().map_err(|_| {
            syn::Error::new_spanned(open, "this open tag has no corresponding close tag")
        })?;

        let token_stream: proc_macro2::TokenStream = content.into_iter().collect();
        let children = syn::parse::<HtmlListChildren>(token_stream.into())?;

        Ok(HtmlList { children })
    }
}

impl ToTokens for HtmlList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlList { children } = self;
        tokens.extend(quote! {
            ::yew_html_common::html_tree::html_list::HtmlList {
                children: #children,
            }
        });
    }
}
