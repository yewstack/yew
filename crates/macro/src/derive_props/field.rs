use super::generics::GenericArguments;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Error, Field, Meta, MetaList, NestedMeta, Type, Visibility};

#[derive(Eq)]
pub struct PropField {
    ty: Type,
    name: Ident,
    wrapped_name: Option<Ident>,
}

impl PropField {
    /// All required property fields are wrapped in an `Option`
    pub fn is_required(&self) -> bool {
        self.wrapped_name.is_some()
    }

    /// This step name is descriptive to help a developer realize they missed a required prop
    pub fn to_step_name(&self, props_name: &Ident) -> Ident {
        Ident::new(
            &format!("{}_missing_required_prop_{}", props_name, self.name),
            Span::call_site(),
        )
    }

    /// Used to transform the `PropWrapper` struct into `Properties`
    pub fn to_field_setter(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        if let Some(wrapped_name) = &self.wrapped_name {
            quote! {
                #name: self.wrapped.#wrapped_name.unwrap(),
            }
        } else {
            quote! {
                #name: self.wrapped.#name,
            }
        }
    }

    /// Wrap all required props in `Option`
    pub fn to_field_def(&self) -> proc_macro2::TokenStream {
        let ty = &self.ty;
        if let Some(wrapped_name) = &self.wrapped_name {
            quote! {
                #wrapped_name: ::std::option::Option<#ty>,
            }
        } else {
            let name = &self.name;
            quote! {
                #name: #ty,
            }
        }
    }

    /// All optional props must implement the `Default` trait
    pub fn to_default_setter(&self) -> proc_macro2::TokenStream {
        if let Some(wrapped_name) = &self.wrapped_name {
            quote! {
                #wrapped_name: ::std::default::Default::default(),
            }
        } else {
            let name = &self.name;
            quote! {
                #name: ::std::default::Default::default(),
            }
        }
    }

    /// Each field is set using a builder method
    pub fn to_build_step_fn(
        &self,
        builder_name: &Ident,
        generic_arguments: &GenericArguments,
        vis: &Visibility,
    ) -> proc_macro2::TokenStream {
        let Self {
            name,
            ty,
            wrapped_name,
        } = self;
        if let Some(wrapped_name) = wrapped_name {
            quote! {
                #[doc(hidden)]
                #vis fn #name(mut self, #name: #ty) -> #builder_name<#generic_arguments> {
                    self.wrapped.#wrapped_name = ::std::option::Option::Some(#name);
                    #builder_name {
                        wrapped: self.wrapped,
                        _marker: ::std::marker::PhantomData,
                    }
                }
            }
        } else {
            quote! {
                #[doc(hidden)]
                #vis fn #name(mut self, #name: #ty) -> #builder_name<#generic_arguments> {
                    self.wrapped.#name = #name;
                    self
                }
            }
        }
    }

    // Detect the `#[props(required)]` attribute which denotes required fields
    fn required_wrapper(named_field: &syn::Field) -> Result<Option<Ident>> {
        let meta_list = if let Some(meta_list) = Self::find_props_meta_list(named_field) {
            meta_list
        } else {
            return Ok(None);
        };

        let expected_required = syn::Error::new(meta_list.span(), "expected `props(required)`");
        let first_nested = if let Some(first_nested) = meta_list.nested.first() {
            first_nested
        } else {
            return Err(expected_required);
        };

        let word_path = match first_nested {
            NestedMeta::Meta(Meta::Path(path)) => path,
            _ => return Err(expected_required),
        };

        if !word_path.is_ident("required") {
            return Err(expected_required);
        }

        if let Some(ident) = &named_field.ident {
            Ok(Some(Ident::new(
                &format!("{}_wrapper", ident),
                Span::call_site(),
            )))
        } else {
            unreachable!()
        }
    }

    fn find_props_meta_list(field: &syn::Field) -> Option<MetaList> {
        let meta_list = field
            .attrs
            .iter()
            .find_map(|attr| match attr.parse_meta().ok()? {
                Meta::List(meta_list) => Some(meta_list),
                _ => None,
            })?;

        if meta_list.path.is_ident("props") {
            Some(meta_list)
        } else {
            None
        }
    }
}

impl TryFrom<Field> for PropField {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self> {
        Ok(PropField {
            wrapped_name: Self::required_wrapper(&field)?,
            ty: field.ty,
            name: field.ident.unwrap(),
        })
    }
}

impl PartialOrd for PropField {
    fn partial_cmp(&self, other: &PropField) -> Option<Ordering> {
        if self.name == other.name {
            Some(Ordering::Equal)
        } else if self.name == "children" {
            Some(Ordering::Greater)
        } else if other.name == "children" {
            Some(Ordering::Less)
        } else {
            self.name.partial_cmp(&other.name)
        }
    }
}

impl Ord for PropField {
    fn cmp(&self, other: &PropField) -> Ordering {
        if self.name == other.name {
            Ordering::Equal
        } else if self.name == "children" {
            Ordering::Greater
        } else if other.name == "children" {
            Ordering::Less
        } else {
            self.name.cmp(&other.name)
        }
    }
}

impl PartialEq for PropField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
