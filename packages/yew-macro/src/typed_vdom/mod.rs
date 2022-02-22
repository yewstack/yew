use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, LitStr, Token, Type};

pub mod generate_element;
mod globals;
pub use globals::*;

#[derive(Clone)]
pub struct AttributePropDefinition {
    pub name: Ident,
    pub ty: Type,
}

impl Parse for AttributePropDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = Ident::parse_any(input)?;
        let _separator = input.parse::<Token![:]>();
        let ty = input.parse()?;
        Ok(Self { name, ty })
    }
}

impl AttributePropDefinition {
    pub fn new(name: Ident, ty: Type) -> Self {
        Self { name, ty }
    }
    fn build_fields(&self) -> TokenStream {
        let AttributePropDefinition { ty, .. } = self;
        let name = self.name();
        quote! {
            #[prop_or_default]
            pub #name: ::std::option::Option::<#ty>,
        }
    }

    fn name(&self) -> Ident {
        format_ident!("r#{}", self.name)
    }

    fn build_if_lets(&self) -> TokenStream {
        let ident = self.name();
        let name = self.name.to_string().replace('_', "-");
        let name = LitStr::new(&name, self.name.span());
        quote! {
            if let ::std::option::Option::Some(val) = self.#ident.as_ref() {
                attrs.insert(#name, val.clone());
            }
        }
    }
}

#[derive(Clone)]
pub struct ListenerPropDefinition {
    event: Ident,
    ty: Type,
}

impl ListenerPropDefinition {
    fn new(event: Ident) -> Self {
        Self {
            event,
            ty: parse_quote! { Event },
        }
    }
    fn new_with_ty(event: Ident, ty: Type) -> Self {
        Self { event, ty }
    }
    fn ident(&self) -> Ident {
        format_ident!("on{}", self.event, span = self.event.span())
    }
    fn build_fields(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            #[prop_or_default]
            pub #ident: ::std::option::Option::<::yew::Callback::<::web_sys::#ty>>,
        }
    }
    fn build_if_lets(&self) -> TokenStream {
        let ident = self.ident();
        quote! {
            if let Some(value) = self.#ident.as_ref() {
                listeners.push(::yew::html::#ident::Wrapper::__macro_new(value));
            }
        }
    }
}

pub(crate) mod kw {
    syn::custom_keyword!(props);
}
