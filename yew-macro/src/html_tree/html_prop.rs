use crate::html_tree::HtmlDashedName as HtmlPropLabel;
use crate::{Peek, PeekValue};
use boolinator::Boolinator;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, Token};

pub struct HtmlProp {
    pub label: HtmlPropLabel,
    pub value: Expr,
}

impl PeekValue<()> for HtmlProp {
    fn peek(cursor: Cursor) -> Option<()> {
        let (_, cursor) = HtmlPropLabel::peek(cursor)?;
        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '=').as_option()
    }
}

impl Parse for HtmlProp {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let label = input.parse::<HtmlPropLabel>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<Expr>()?;
        // backwards compat
        let _ = input.parse::<Token![,]>();
        Ok(HtmlProp { label, value })
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
        let stream: proc_macro2::TokenStream = trees.into_iter().collect();
        let stream = TokenStream::from(stream);

        Ok(HtmlPropSuffix { stream, div, gt })
    }
}
