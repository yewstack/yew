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

        let (type_str, _) = HtmlComponent::type_str(cursor)?;
        (type_str.to_lowercase() != type_str).as_option()
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
        let override_props = match props {
            Props::List(vec_props) => {
                let check_props = vec_props.0.iter().map(|HtmlProp { name, .. }| {
                    quote_spanned! { name.span()=> let _ = #vcomp_props.#name; }
                });

                let set_props = vec_props.0.iter().map(|HtmlProp { name, value }| {
                    quote_spanned! { value.span()=> #vcomp_props.#name = #value.into(); }
                });

                quote! {
                    #(#check_props#set_props)*
                }
            }
            Props::With(WithProps(props)) => {
                quote_spanned! { props.span()=> #vcomp_props = #props; }
            }
            Props::None => quote! {},
        };

        // hack because span breaks with $crate inline
        let alias_virtual_dom = quote! { use $crate::virtual_dom as _virtual_dom; };
        let lazy_init = quote_spanned! { ty.span()=>
            #alias_virtual_dom
            let (mut #vcomp_props, mut #vcomp) = _virtual_dom::VComp::lazy::<#ty>();
        };

        tokens.extend(quote! {{
            #lazy_init
            #override_props
            #vcomp.set_props(#vcomp_props);
            #vcomp
        }});
    }
}

impl HtmlComponent {
    fn double_colon(cursor: Cursor) -> Option<Cursor> {
        let mut cursor = cursor;
        for _ in 0..2 {
            let (punct, c) = cursor.punct()?;
            (punct.as_char() == ':').as_option()?;
            cursor = c;
        }

        Some(cursor)
    }

    fn type_str(cursor: Cursor) -> Option<(String, Cursor)> {
        let mut cursor = cursor;
        let mut type_str: String = "".to_owned();
        let mut parse_ident_ok = true;
        let mut parse_colons_ok = true;

        while parse_ident_ok {
            if let Some((ident, c)) = cursor.ident() {
                if parse_ident_ok {
                    cursor = c;
                    type_str += &ident.to_string();
                    parse_colons_ok = true;
                } else {
                    break;
                }
            }
            parse_ident_ok = false;

            if let Some(c) = Self::double_colon(cursor) {
                if parse_colons_ok {
                    cursor = c;
                    type_str += "::";
                    parse_ident_ok = true;
                } else {
                    break;
                }
            }
            parse_colons_ok = false;
        }

        Some((type_str, cursor))
    }
}

pub struct HtmlComponentInner {
    ty: Type,
    props: Props,
}

impl Parse for HtmlComponentInner {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let ty = input.parse()?;
        // backwards compatibility
        let _ = input.parse::<Token![:]>();
        let props = input.parse()?;
        Ok(HtmlComponentInner { ty, props })
    }
}

struct HtmlComponentTag {
    lt: Token![<],
    gt: Token![>],
}

impl ToTokens for HtmlComponentTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponentTag { lt, gt, .. } = self;
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
    None,
}

impl Peek<PropType> for Props {
    fn peek(cursor: Cursor) -> Option<PropType> {
        let (ident, _) = cursor.ident()?;
        if ident.to_string() == "with" {
            Some(PropType::With)
        } else {
            Some(PropType::List)
        }
    }
}

impl Parse for Props {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let props = if let Some(prop_type) = Props::peek(input.cursor()) {
            match prop_type {
                PropType::List => input.parse().map(Props::List)?,
                PropType::With => input.parse().map(Props::With)?,
            }
        } else {
            Props::None
        };

        Ok(props)
    }
}

struct ListProps(Vec<HtmlProp>);
impl Parse for ListProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut props: Vec<HtmlProp> = Vec::new();
        while HtmlProp::peek(input.cursor()).is_some() {
            props.push(input.parse::<HtmlProp>()?);
        }
        Ok(ListProps(props))
    }
}

struct WithProps(Ident);
impl Parse for WithProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let with = input.parse::<Ident>()?;
        if with.to_string() != "with" {
            return Err(input.error("expected to find with token"));
        }
        let props = input.parse::<Ident>()?;
        let _ = input.parse::<Token![,]>();
        Ok(WithProps(props))
    }
}
