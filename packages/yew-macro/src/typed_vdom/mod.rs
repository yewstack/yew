use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Token, Type};
use syn::ext::IdentExt;

pub mod generate_element;
mod globals;
#[derive(Clone)]
pub struct AttributePropDefinition {
    name: Ident,
    ty: Type,
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
            pub #name: ::std::option::Option::<#ty>,
        }
    }
    fn _build_setter(&self) -> TokenStream {
        let AttributePropDefinition { ty, .. } = self;
        let name = self.name();
        quote! {
            pub fn #name(&mut self, val: #ty) {
                self.#name = ::std::option::Option::Some(val);
            }
        }
    }
    fn name(&self) -> Ident {
        format_ident!("r#{}", self.name)
    }
    fn build_if_lets(&self) -> TokenStream {
        let ident = self.name();
        let name = self.name.to_string().replace('_', "-");
        quote! {
            if let Some(val) = self.#ident {
                attrs.insert(::std::stringify!(#name), val);
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
        format_ident!("on_{}", self.event, span = self.event.span())
    }
    fn build_fields(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            pub #ident: ::std::option::Option::<::yew::Callback::<::web_sys::#ty>>,
        }
    }
    fn build_if_lets(&self) -> TokenStream {
        let ident = self.ident();
        let on_event = Ident::new(&format!("on{}", self.event), Span::mixed_site());
        quote! {
            if let Some(value) = self.#ident {
                listeners.push(::yew::html::#on_event::Wrapper::__macro_new(value));
            }
        }
    }
}
pub(crate) mod kw {
    syn::custom_keyword!(props);
}
