use super::{HtmlProp, HtmlPropList};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    TypePath,
};

/// Pop from `Punctuated` without leaving it in a state where it has trailing punctuation.
fn pop_last_punctuated<T, P>(punctuated: &mut Punctuated<T, P>) -> Option<T> {
    let value = punctuated.pop().map(|pair| pair.into_value());
    // remove the 2nd last value and push it right back to remove the trailing punctuation
    if let Some(pair) = punctuated.pop() {
        punctuated.push_value(pair.into_value());
    }
    value
}

/// Check if the given type path looks like an associated type.
fn is_associated_properties(ty: &TypePath) -> bool {
    let mut segments_it = ty.path.segments.iter();
    if let Some(seg) = segments_it.next_back() {
        // if the last segment is `Properties` ...
        if seg.ident == "Properties" {
            if let Some(seg) = segments_it.next_back() {
                // ... and we can be reasonably sure that the previous segment is a component ...
                if !crate::non_capitalized_ascii(&seg.ident.to_string()) {
                    // ... then we assume that this is an associated type like `Component::Properties`
                    return true;
                }
            }
        }
    }

    false
}

pub struct ComponentProps {
    ty: TypePath,
    props: HtmlPropList,
}
impl Parse for ComponentProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ty: TypePath = input.parse()?;

        // if the type isn't already qualified (`<x as y>`) and it's an associated type (`MyComp::Properties`) ...
        if ty.qself.is_none() && is_associated_properties(&ty) {
            pop_last_punctuated(&mut ty.path.segments);
            // .. transform it into a "qualified-self" type
            ty = syn::parse2(quote_spanned! {ty.span()=>
                <#ty as ::yew::html::Component>::Properties
            })?;
        }

        let props: HtmlPropList = input.parse()?;
        for prop in props.iter() {
            if prop.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &prop.label,
                    "optional attributes are only supported on HTML tags. Yew components can use `Option<T>` properties to accomplish the same thing.",
                ));
            }
        }

        Ok(Self { ty, props })
    }
}
impl ToTokens for ComponentProps {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ty, props } = self;
        let set_props = props.iter().map(|HtmlProp { label, value, .. }| {
            quote_spanned! {value.span()=>
                .#label(<::yew::virtual_dom::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(#value))
            }
        });

        tokens.extend(quote_spanned! {ty.span()=>
            <#ty as ::yew::html::Properties>::builder()
                #(#set_props)*
                .build()
        })
    }
}
