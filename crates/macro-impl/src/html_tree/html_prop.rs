use crate::Peek;
use boolinator::Boolinator;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{quote, ToTokens};
use std::fmt;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, Token};

pub struct HtmlProp {
    pub label: HtmlPropLabel,
    pub value: Expr,
}

impl Peek<()> for HtmlProp {
    fn peek(mut cursor: Cursor) -> Option<()> {
        loop {
            let (_, c) = cursor.ident()?;
            let (punct, c) = c.punct()?;
            if punct.as_char() == '-' {
                cursor = c;
                continue;
            }
            return (punct.as_char() == '=').as_option();
        }
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

        Ok(HtmlPropSuffix { div, gt, stream })
    }
}

pub struct HtmlPropLabel {
    pub name: Ident,
    pub extended: Vec<(Token![-], Ident)>,
}

impl HtmlPropLabel {
    pub fn new(name: Ident) -> Self {
        HtmlPropLabel {
            name,
            extended: Vec::new(),
        }
    }
}

impl fmt::Display for HtmlPropLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for (_, ident) in &self.extended {
            write!(f, "-{}", ident)?;
        }
        Ok(())
    }
}

impl Parse for HtmlPropLabel {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = if let Ok(token) = input.parse::<Token![type]>() {
            Ident::new("type", token.span).into()
        } else if let Ok(token) = input.parse::<Token![for]>() {
            Ident::new("for", token.span).into()
        } else {
            input.parse::<Ident>()?.into()
        };

        let mut extended = Vec::new();
        while input.peek(Token![-]) {
            extended.push((input.parse::<Token![-]>()?, input.parse::<Ident>()?));
        }

        Ok(HtmlPropLabel { name, extended })
    }
}

impl ToTokens for HtmlPropLabel {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlPropLabel { name, extended } = self;
        let dashes = extended.iter().map(|(dash, _)| quote! {#dash});
        let idents = extended.iter().map(|(_, ident)| quote! {#ident});
        let extended = quote! { #(#dashes#idents)* };
        tokens.extend(quote! {#name#extended});
    }
}
