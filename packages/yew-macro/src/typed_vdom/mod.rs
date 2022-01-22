use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Token, Type};

pub mod generate_element;
mod globals;

#[derive(Clone)]
pub struct AttributePropDefinition {
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
    pub fn new(name: Ident, ty: Type) -> Self {
        Self { name, ty }
    }
    fn build_fields(&self) -> TokenStream {
        let AttributePropDefinition { name, ty } = self;
        quote! {
            pub #name: ::std::option::Option::<#ty>,
        }
    }

    fn _build_setter(&self) -> TokenStream {
        let AttributePropDefinition { name, ty } = self;
        quote! {
            pub fn #name(&mut self, val: #ty) {
                self.#name = ::std::option::Option::Some(val);
            }
        }
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
    syn::custom_keyword!(props);
}
