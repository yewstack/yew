pub mod global;
pub mod generate_element;

use proc_macro2::{Ident, TokenStream};
use quote::{quote};
use syn::{ Token, Type};
use syn::parse::{Parse, ParseStream};

pub(crate) fn build_setters(name: &Ident, ty: &Type, wrap_option: bool) -> TokenStream {
    let value = if wrap_option { quote! { ::std::option::Option::Some(val) } } else { quote! { val } };
    quote! {
        pub fn #name(&mut self, val: #ty) {
            self.#name = #value;
        }
    }
}

pub(crate) fn build_fields(name: &Ident, ty: &Type, wrap_option: bool) -> TokenStream {
    let ty = if wrap_option { quote! { ::std::option::Option::<#ty> } } else { quote! { #ty }};
    quote! {
        pub #name: #ty,
    }
}

struct AttributePropDefinition {
    name: Ident,
    ty: Type,
}

impl Parse for AttributePropDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _separator = input.parse::<Token![:]>();
        let ty = input.parse()?;
        Ok(Self { name, ty })
    }
}

impl AttributePropDefinition {
    fn build_fields(&self) -> TokenStream {
        let AttributePropDefinition { name, ty } = self;
        build_fields(name, ty, true)
    }

    fn build_setter(&self) -> TokenStream {
        let AttributePropDefinition { name, ty } = self;
        build_setters(name, ty, true)
    }

    fn build_if_lets(&self) -> TokenStream {
        let AttributePropDefinition { name, .. } = self;
        if name == "children" || name == "key" || name == "node_ref" {
            quote! {}
        } else {
            quote! {
                if let Some(val) = self.#name {
                    attrs.insert(::std::stringify!(#name), val);
                }
            }
        }
    }
}

pub(crate) mod kw {
    syn::custom_keyword!(extends);
    syn::custom_keyword!(props);

    syn::custom_keyword!(attrs);
    syn::custom_keyword!(listeners);
}
