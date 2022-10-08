use std::convert::TryInto;

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Brace;
use syn::{Expr, Token, TypePath};

use super::{ComponentProps, Prop, PropList, Props};
use crate::html_tree::HtmlDashedName;

/// Pop from `Punctuated` without leaving it in a state where it has trailing punctuation.
fn pop_last_punctuated<T, P>(punctuated: &mut Punctuated<T, P>) -> Option<T> {
    let value = punctuated.pop().map(|pair| pair.into_value());
    // remove the 2nd last value and push it right back to remove the trailing punctuation
    if let Some(pair) = punctuated.pop() {
        punctuated.push_value(pair.into_value());
    }
    value
}

/// Check if the given type path looks like an associated `Properties` type.
fn is_associated_properties(ty: &TypePath) -> bool {
    let mut segments_it = ty.path.segments.iter();
    if let Some(seg) = segments_it.next_back() {
        // if the last segment is `Properties` ...
        if seg.ident == "Properties" {
            if let Some(seg) = segments_it.next_back() {
                // ... and we can be reasonably sure that the previous segment is a component ...
                if !crate::non_capitalized_ascii(&seg.ident.to_string()) {
                    // ... then we assume that this is an associated type like
                    // `Component::Properties`
                    return true;
                }
            }
        }
    }

    false
}

struct PropValue {
    label: HtmlDashedName,
    value: Expr,
}
impl Parse for PropValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let label = input.parse()?;
        let value = if input.peek(Token![:]) {
            let _colon_token: Token![:] = input.parse()?;
            input.parse()?
        } else {
            syn::parse_quote!(#label)
        };
        Ok(Self { label, value })
    }
}

impl From<PropValue> for Prop {
    fn from(prop_value: PropValue) -> Prop {
        let PropValue { label, value } = prop_value;
        Prop {
            label,
            value,
            directive: None,
        }
    }
}

struct PropsExpr {
    ty: TypePath,
    _brace_token: Brace,
    fields: Punctuated<PropValue, Token![,]>,
}
impl Parse for PropsExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ty: TypePath = input.parse()?;

        // if the type isn't already qualified (`<x as y>`) and it's an associated type
        // (`MyComp::Properties`) ...
        if ty.qself.is_none() && is_associated_properties(&ty) {
            pop_last_punctuated(&mut ty.path.segments);
            // .. transform it into a "qualified-self" type
            ty = syn::parse2(quote_spanned! {ty.span()=>
                <#ty as ::yew::html::Component>::Properties
            })?;
        }

        let content;
        let brace_token = syn::braced!(content in input);
        let fields = content.parse_terminated(PropValue::parse)?;
        Ok(Self {
            ty,
            _brace_token: brace_token,
            fields,
        })
    }
}

pub struct PropsMacroInput {
    ty: TypePath,
    props: ComponentProps,
}
impl Parse for PropsMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let PropsExpr { ty, fields, .. } = input.parse()?;
        let prop_list = PropList::new(fields.into_iter().map(Into::into).collect());
        let props: Props = prop_list.try_into()?;
        props.special.check_all(|prop| {
            let label = &prop.label;
            Err(syn::Error::new_spanned(
                label,
                "special props cannot be specified in the `props!` macro",
            ))
        })?;
        Ok(Self {
            ty,
            props: props.try_into()?,
        })
    }
}
impl ToTokens for PropsMacroInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ty, props } = self;

        tokens.extend(props.build_properties_tokens(ty, None::<TokenStream>))
    }
}
