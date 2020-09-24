use super::{HtmlProp, HtmlPropList};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Type,
};

pub struct ComponentProps {
    ty: Type,
    props: HtmlPropList,
}
impl Parse for ComponentProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
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
