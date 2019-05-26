use crate::Peek;
use boolinator::Boolinator;
use quote::{quote, quote_spanned, ToTokens};
use proc_macro2::Span;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Attribute, Ident, Token, Type, Expr, Lit, ExprLit};
use syn::spanned::Spanned;

pub struct HtmlComponent {
    ty: Type,
    props: Option<Props>,
}

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
        input.parse::<Token![<]>()?;
        let ty = input.parse::<Type>()?;

        // backwards compatibility
        let _ = input.parse::<Token![:]>();

        let props = if let Some(prop_type) = Props::peek(input.cursor()) {
            match prop_type {
                PropType::List => {
                    let mut props: Vec<Prop> = Vec::new();
                    while Prop::peek(input.cursor()).is_some() {
                        props.push(input.parse::<Prop>()?);
                    }
                    Some(Props::List(props))
                }
                PropType::With => {
                    Some(Props::With(input.parse::<WithProps>()?))
                }
            }
        } else {
            None
        };

        let comp = HtmlComponent { ty, props};

        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        Ok(comp)
    }
}

impl ToTokens for HtmlComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponent { ty, props } = self;
        let vcomp = Ident::new("__yew_vcomp", Span::call_site());
        let vcomp_props = Ident::new("__yew_vcomp_props", Span::call_site());
        let override_props = if let Some(props) = props {
            match props {
                Props::List(vec_props) => {
                    let check_props = vec_props.iter().map(|Prop{attr, ..}| {
                        quote_spanned!{ attr.span()=> let _ = #vcomp_props.#attr; }
                    });

                    let set_props = vec_props.iter().map(|Prop{attr, value}| {
                        quote_spanned!{ value.span()=> #vcomp_props.#attr = #value.into(); }
                    });

                    quote! {
                        #(#check_props#set_props)*
                    }
                }
                Props::With(WithProps(props)) => {
                    quote_spanned!{ props.span()=> #vcomp_props = #props; }
                }
            }
        } else {
            quote!{}
        };


        // hack because span breaks with $crate inline
        let alias_virtual_dom = quote!{ use $crate::virtual_dom as _virtual_dom; };
        let lazy_init = quote_spanned!{ ty.span()=>
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

enum PropType {
    List,
    With,
}

enum Props {
    List(Vec<Prop>),
    With(WithProps),
}

struct Prop {
    attr: Ident,
    value: Expr,
}

struct WithProps(Ident);

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


impl Peek<()> for Prop {
    fn peek(cursor: Cursor) -> Option<()> {
        let (_, cursor) = cursor.ident()?;
        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '=').as_option()
    }
}

struct ExprBlock(syn::ExprBlock);

impl Parse for ExprBlock {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(ExprBlock(syn::ExprBlock{
            attrs: input.call(Attribute::parse_outer)?,
            label: input.parse().ok(),
            block: input.parse()?,
        }))
    }
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let attr = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;

        let err = input.error("expected literal or expression block");
        let value = if let Ok(lit) = input.parse::<Lit>() {
            Expr::Lit(ExprLit{
                attrs: Vec::new(),
                lit,
            })
        } else if let Ok(ExprBlock(block)) = input.parse::<ExprBlock>() {
            Expr::Block(block)
        } else {
            return Err(err);
        };
        let _ = input.parse::<Token![,]>();
        Ok(Prop{attr, value})
    }
}

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
