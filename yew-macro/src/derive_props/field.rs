use super::generics::GenericArguments;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Error, Expr, Field, Type, Visibility};

#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq)]
enum PropAttr {
    Required { wrapped_name: Ident },
    PropOr(Expr),
    PropOrElse(Expr),
    PropOrDefault,
}

#[derive(Eq)]
pub struct PropField {
    ty: Type,
    name: Ident,
    attr: PropAttr,
}

impl PropField {
    /// All required property fields are wrapped in an `Option`
    pub fn is_required(&self) -> bool {
        matches!(self.attr, PropAttr::Required { .. })
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
        match &self.attr {
            PropAttr::Required { wrapped_name } => {
                quote! {
                    #name: self.wrapped.#wrapped_name.unwrap(),
                }
            }
            _ => {
                quote! {
                    #name: self.wrapped.#name,
                }
            }
        }
    }

    /// Wrap all required props in `Option`
    pub fn to_field_def(&self) -> proc_macro2::TokenStream {
        let ty = &self.ty;
        match &self.attr {
            PropAttr::Required { wrapped_name } => {
                quote! {
                    #wrapped_name: ::std::option::Option<#ty>,
                }
            }
            _ => {
                let name = &self.name;
                quote! {
                    #name: #ty,
                }
            }
        }
    }

    /// All optional props must implement the `Default` trait
    pub fn to_default_setter(&self) -> proc_macro2::TokenStream {
        match &self.attr {
            PropAttr::Required { wrapped_name } => {
                quote! {
                    #wrapped_name: ::std::option::Option::None,
                }
            }
            PropAttr::PropOr(value) => {
                let name = &self.name;
                let span = value.span();
                quote_spanned! {span=>
                    #name: #value,
                }
            }
            PropAttr::PropOrElse(func) => {
                let name = &self.name;
                let span = func.span();
                quote_spanned! {span=>
                    #name: (#func)(),
                }
            }
            PropAttr::PropOrDefault => {
                let name = &self.name;
                quote! {
                    #name: ::std::default::Default::default(),
                }
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
        let Self { name, ty, attr } = self;
        match attr {
            PropAttr::Required { wrapped_name } => {
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
            }
            _ => {
                quote! {
                    #[doc(hidden)]
                    #vis fn #name(mut self, #name: #ty) -> #builder_name<#generic_arguments> {
                        self.wrapped.#name = #name;
                        self
                    }
                }
            }
        }
    }

    // Detect Properties 2.0 attributes
    fn attribute(named_field: &Field) -> Result<PropAttr> {
        let attr = named_field.attrs.iter().find(|attr| {
            attr.path.is_ident("prop_or")
                || attr.path.is_ident("prop_or_else")
                || attr.path.is_ident("prop_or_default")
        });

        if let Some(attr) = attr {
            if attr.path.is_ident("prop_or") {
                Ok(PropAttr::PropOr(attr.parse_args()?))
            } else if attr.path.is_ident("prop_or_else") {
                Ok(PropAttr::PropOrElse(attr.parse_args()?))
            } else if attr.path.is_ident("prop_or_default") {
                Ok(PropAttr::PropOrDefault)
            } else {
                unreachable!()
            }
        } else {
            let ident = named_field.ident.as_ref().unwrap();
            let wrapped_name = Ident::new(&format!("{}_wrapper", ident), Span::call_site());
            Ok(PropAttr::Required { wrapped_name })
        }
    }
}

impl TryFrom<Field> for PropField {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self> {
        Ok(PropField {
            attr: Self::attribute(&field)?,
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
