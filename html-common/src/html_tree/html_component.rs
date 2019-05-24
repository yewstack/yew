use crate::Peek;
use boolinator::Boolinator;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::Token;
use syn::Type;

pub struct HtmlComponent {
    ty: Type,
}

impl Peek<()> for HtmlComponent {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (type_str, cursor) = HtmlComponent::type_str(cursor)?;
        (type_str.to_lowercase() != type_str).as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()
    }
}

impl Parse for HtmlComponent {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        input.parse::<Token![<]>()?;
        let comp = HtmlComponent { ty: input.parse()? };
        // backwards compatibility
        let _ = input.parse::<Token![:]>();
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        Ok(comp)
    }
}

impl ToTokens for HtmlComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponent { ty } = self;
        tokens.extend(quote! {{
            ::yew::virtual_dom::VComp::lazy::<#ty>().1
        }});
    }
}

impl HtmlComponent {
    fn type_str(cursor: Cursor) -> Option<(String, Cursor)> {
        let mut cursor = cursor;
        let mut type_str: String = "".to_owned();
        let mut progress = true;
        while progress {
            progress = false;
            match cursor.ident() {
                Some((ident, c)) => {
                    type_str += &ident.to_string();
                    cursor = c;
                    progress = true;
                }
                None => {}
            }

            match cursor.punct() {
                Some((punct, c)) => match punct.as_char() {
                    ':' => {
                        type_str += ":";
                        cursor = c;
                        progress = true;
                    }
                    '/' => {}
                    _ => return None,
                },
                None => {}
            }
        }

        Some((type_str, cursor))
    }
}
