use crate::Peek;
use boolinator::Boolinator;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Attribute, Expr, Ident, Token};

pub struct HtmlProp {
    pub name: Ident,
    pub value: Expr,
}

impl Peek<()> for HtmlProp {
    fn peek(cursor: Cursor) -> Option<()> {
        let (_, cursor) = cursor.ident()?;
        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '=').as_option()
    }
}

struct ExprBlock(syn::ExprBlock);

impl Parse for ExprBlock {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(ExprBlock(syn::ExprBlock {
            attrs: input.call(Attribute::parse_outer)?,
            label: input.parse().ok(),
            block: input.parse()?,
        }))
    }
}

impl Parse for HtmlProp {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = if let Ok(ty) = input.parse::<Token![type]>() {
            Ident::new("type", ty.span)
        } else {
            input.parse::<Ident>()?
        };

        input.parse::<Token![=]>()?;
        let value = input.parse::<Expr>()?;
        let _ = input.parse::<Token![,]>();
        Ok(HtmlProp { name, value })
    }
}

pub struct HtmlPropSuffix {
    pub div: Option<Token![/]>,
    pub gt: Token![>],
    pub stream: TokenStream,
}

impl Parse for HtmlPropSuffix {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut trees: Vec<TokenTree> = vec![];
        let mut div: Option<Token![/]> = None;
        let mut angle_count = 1;
        let gt: Option<Token![>]>;

        loop {
            let next = input.parse()?;
            match &next {
                TokenTree::Punct(punct) => match punct.as_char() {
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
                },
                _ => {}
            }
            trees.push(next);
        }

        let gt: Token![>] = gt.ok_or(input.error("missing tag close"))?;
        let stream: proc_macro2::TokenStream = trees.into_iter().collect();
        let stream = TokenStream::from(stream);

        Ok(HtmlPropSuffix { div, gt, stream })
    }
}
