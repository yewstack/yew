use crate::html_tree::HtmlDashedName as HtmlPropLabel;
use crate::{Peek, PeekValue};
use proc_macro2::{TokenStream, TokenTree};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, Token};

pub struct HtmlProp {
    pub label: HtmlPropLabel,
    pub question_mark: Option<Token![?]>,
    pub value: Expr,
}

impl PeekValue<()> for HtmlProp {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlPropLabel::peek(cursor).map(|_| ())
    }
}

impl Parse for HtmlProp {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let label = input.parse::<HtmlPropLabel>()?;
        let question_mark = if input.peek(Token![?]) {
            Some(input.parse::<Token![?]>()?)
        } else {
            None
        };
        let equals = input
            .parse::<Token![=]>()
            .map_err(|_| syn::Error::new_spanned(&label, "this prop doesn't have a value"))?;
        if input.is_empty() {
            return Err(syn::Error::new_spanned(
                equals,
                "expected an expression following this equals sign",
            ));
        }
        let value = input.parse::<Expr>()?;
        // backwards compat
        let _ = input.parse::<Token![,]>();
        Ok(HtmlProp {
            label,
            question_mark,
            value,
        })
    }
}

pub struct HtmlPropSuffix {
    pub stream: TokenStream,
    pub div: Option<Token![/]>,
    pub gt: Token![>],
}

impl Parse for HtmlPropSuffix {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut trees: Vec<TokenTree> = vec![];
        let mut div: Option<Token![/]> = None;
        let mut angle_count = 1;
        let gt: Option<Token![>]>;

        loop {
            let next = input.parse()?;
            if let TokenTree::Punct(punct) = &next {
                match punct.as_char() {
                    '>' => {
                        angle_count -= 1;
                        if angle_count == 0 {
                            gt = Some(syn::token::Gt {
                                spans: [punct.span()],
                            });
                            break;
                        }
                    }
                    '<' => angle_count += 1,
                    '/' => {
                        if angle_count == 1 && input.peek(Token![>]) {
                            div = Some(syn::token::Div {
                                spans: [punct.span()],
                            });
                            gt = Some(input.parse()?);
                            break;
                        }
                    }
                    _ => {}
                };
            }
            trees.push(next);
        }

        let gt: Token![>] = gt.ok_or_else(|| input.error("missing tag close"))?;
        let stream: TokenStream = trees.into_iter().collect();

        Ok(HtmlPropSuffix { stream, div, gt })
    }
}
