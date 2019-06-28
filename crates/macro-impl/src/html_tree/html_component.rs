use super::HtmlProp;
use super::HtmlPropSuffix;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Ident, Token, Type};

pub struct HtmlComponent(HtmlComponentInner);

impl Peek<()> for HtmlComponent {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        HtmlComponent::peek_type(cursor)
    }
}

impl Parse for HtmlComponent {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse::<Token![<]>()?;
        let HtmlPropSuffix { stream, div, gt } = input.parse()?;
        if div.is_none() {
            return Err(syn::Error::new_spanned(
                HtmlComponentTag { lt, gt },
                "expected component tag be of form `< .. />`",
            ));
        }

        match parse(stream) {
            Ok(comp) => Ok(HtmlComponent(comp)),
            Err(err) => {
                if err.to_string().starts_with("unexpected end of input") {
                    Err(syn::Error::new_spanned(div, err.to_string()))
                } else {
                    Err(err)
                }
            }
        }
    }
}

impl ToTokens for HtmlComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponentInner { ty, props } = &self.0;
        let vcomp = Ident::new("__yew_vcomp", Span::call_site());
        let vcomp_props = Ident::new("__yew_vcomp_props", Span::call_site());
        let override_props = props.iter().map(|props| match props {
            Props::List(ListProps(vec_props)) => {
                let check_props = vec_props.iter().map(|HtmlProp { name, .. }| {
                    quote_spanned! { name.span()=> #vcomp_props.#name; }
                });

                let set_props = vec_props.iter().map(|HtmlProp { name, value }| {
                    quote_spanned! { value.span()=>
                        #vcomp_props.#name = ::yew::virtual_dom::vcomp::Transformer::transform(&mut #vcomp, #value);
                    }
                });

                quote! {
                    #(#check_props#set_props)*
                }
            }
            Props::With(WithProps(props)) => {
                quote_spanned! { props.span()=> #vcomp_props = #props; }
            }
        });

        tokens.extend(quote_spanned! { ty.span()=> {
            let (mut #vcomp_props, mut #vcomp) = ::yew::virtual_dom::VComp::lazy::<#ty>();
            #(#override_props)*
            #vcomp.set_props(#vcomp_props);
            ::yew::virtual_dom::VNode::VComp(#vcomp)
        }});
    }
}

impl HtmlComponent {
    fn double_colon(mut cursor: Cursor) -> Option<Cursor> {
        for _ in 0..2 {
            let (punct, c) = cursor.punct()?;
            (punct.as_char() == ':').as_option()?;
            cursor = c;
        }

        Some(cursor)
    }

    fn peek_type(mut cursor: Cursor) -> Option<()> {
        let mut type_str: String = "".to_owned();
        let mut colons_optional = true;

        loop {
            let mut found_colons = false;
            let mut post_colons_cursor = cursor;
            if let Some(c) = Self::double_colon(post_colons_cursor) {
                found_colons = true;
                post_colons_cursor = c;
            } else if !colons_optional {
                break;
            }

            if let Some((ident, c)) = post_colons_cursor.ident() {
                cursor = c;
                if found_colons {
                    type_str += "::";
                }
                type_str += &ident.to_string();
            } else {
                break;
            }

            // only first `::` is optional
            colons_optional = false;
        }

        (!type_str.is_empty()).as_option()?;
        (type_str.to_lowercase() != type_str).as_option()
    }
}

pub struct HtmlComponentInner {
    ty: Type,
    props: Option<Props>,
}

impl Parse for HtmlComponentInner {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let ty = input.parse()?;
        // backwards compat
        let _ = input.parse::<Token![:]>();

        let props = if let Some(prop_type) = Props::peek(input.cursor()) {
            match prop_type {
                PropType::List => input.parse().map(Props::List).map(Some)?,
                PropType::With => input.parse().map(Props::With).map(Some)?,
            }
        } else {
            None
        };

        Ok(HtmlComponentInner { ty, props })
    }
}

struct HtmlComponentTag {
    lt: Token![<],
    gt: Token![>],
}

impl ToTokens for HtmlComponentTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponentTag { lt, gt } = self;
        tokens.extend(quote! {#lt#gt});
    }
}

enum PropType {
    List,
    With,
}

enum Props {
    List(ListProps),
    With(WithProps),
}

impl Peek<PropType> for Props {
    fn peek(cursor: Cursor) -> Option<PropType> {
        let (ident, _) = cursor.ident()?;
        let prop_type = if ident.to_string() == "with" {
            PropType::With
        } else {
            PropType::List
        };

        Some(prop_type)
    }
}

struct ListProps(Vec<HtmlProp>);
impl Parse for ListProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut props: Vec<HtmlProp> = Vec::new();
        while HtmlProp::peek(input.cursor()).is_some() {
            props.push(input.parse::<HtmlProp>()?);
        }

        for prop in &props {
            if prop.name.to_string() == "type" {
                return Err(syn::Error::new_spanned(&prop.name, "expected identifier"));
            }
        }

        Ok(ListProps(props))
    }
}

struct WithProps(Ident);
impl Parse for WithProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let with = input.parse::<Ident>()?;
        if with.to_string() != "with" {
            return Err(input.error("expected to find `with` token"));
        }
        let props = input.parse::<Ident>()?;
        let _ = input.parse::<Token![,]>();
        Ok(WithProps(props))
    }
}
