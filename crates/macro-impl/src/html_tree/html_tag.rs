use super::HtmlTree;
use crate::Peek;
use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::token;
use syn::Ident;

pub struct HtmlTag {
    open: HtmlTagOpen,
    children: Vec<HtmlTree>,
}

impl Peek<()> for HtmlTag {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlTagOpen::peek(cursor)
            .or(HtmlTagClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlTag {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if HtmlTagClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlTagClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close,
                    "this close tag has no corresponding open tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlTagOpen>()?;
        if open.div.is_some() {
            return Ok(HtmlTag {
                open,
                children: Vec::new(),
            });
        }

        let mut cursor = input.cursor();
        let mut tag_stack_count = 1;
        loop {
            if let Some(next_open_ident) = HtmlTagOpen::peek(cursor) {
                if open.ident.to_string() == next_open_ident.to_string() {
                    tag_stack_count += 1;
                }
            } else if let Some(next_close_ident) = HtmlTagClose::peek(cursor) {
                if open.ident.to_string() == next_close_ident.to_string() {
                    tag_stack_count -= 1;
                    if tag_stack_count == 0 {
                        break;
                    }
                }
            }
            if let Some((_, next)) = cursor.token_tree() {
                cursor = next;
            } else {
                break;
            }
        }

        if tag_stack_count > 0 {
            return Err(syn::Error::new_spanned(
                open,
                "this open tag has no corresponding close tag",
            ));
        }

        let mut children: Vec<HtmlTree> = vec![];
        loop {
            if let Some(next_close_ident) = HtmlTagClose::peek(input.cursor()) {
                if open.ident.to_string() == next_close_ident.to_string() {
                    break;
                }
            }

            children.push(input.parse()?);
        }

        input.parse::<HtmlTagClose>()?;

        Ok(HtmlTag { open, children })
    }
}

impl ToTokens for HtmlTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTag { open, children } = self;
        let tag_name = open.ident.to_string();
        tokens.extend(quote! {{
            let mut __yew_vtag = $crate::virtual_dom::vtag::VTag::new(#tag_name);
            #(__yew_vtag.add_child(#children);)*
            __yew_vtag
        }});
    }
}

struct HtmlTagOpen {
    lt: token::Lt,
    ident: Ident,
    div: Option<token::Div>,
    gt: token::Gt,
}

impl Peek<Ident> for HtmlTagOpen {
    fn peek(cursor: Cursor) -> Option<Ident> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (ident, cursor) = cursor.ident()?;
        (ident.to_string().to_lowercase() == ident.to_string()).as_option()?;

        let (mut punct, cursor) = cursor.punct()?;
        if punct.as_char() == '/' {
            let extra_punct = cursor.punct()?;
            punct = extra_punct.0;
        }

        (punct.as_char() == '>').as_option()?;

        Some(ident)
    }
}

impl Parse for HtmlTagOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlTagOpen {
            lt: input.parse()?,
            ident: input.parse()?,
            div: input.parse().ok(),
            gt: input.parse()?,
        })
    }
}

impl ToTokens for HtmlTagOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTagOpen { lt, ident, div, gt } = self;
        let open_tag = match div {
            Some(div) => quote! {#lt#ident#div#gt},
            None => quote! {#lt#ident#gt},
        };
        tokens.extend(open_tag);
    }
}

struct HtmlTagClose {
    lt: token::Lt,
    div: Option<token::Div>,
    ident: Ident,
    gt: token::Gt,
}

impl Peek<Ident> for HtmlTagClose {
    fn peek(cursor: Cursor) -> Option<Ident> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (ident, cursor) = cursor.ident()?;
        (ident.to_string().to_lowercase() == ident.to_string()).as_option()?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(ident)
    }
}

impl Parse for HtmlTagClose {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlTagClose {
            lt: input.parse()?,
            div: input.parse()?,
            ident: input.parse()?,
            gt: input.parse()?,
        })
    }
}

impl ToTokens for HtmlTagClose {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTagClose { lt, div, ident, gt } = self;
        tokens.extend(quote! {#lt#div#ident#gt});
    }
}
