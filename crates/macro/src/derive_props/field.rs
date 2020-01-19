use super::generics::GenericArguments;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{
    Error, ExprPath, Field, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Type, Visibility,
};

#[derive(PartialEq, Eq)]
enum PropAttr {
    Required { wrapped_name: Ident },
    Default { default: ExprPath },
    None,
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
        match self.attr {
            PropAttr::Required { .. } => true,
            _ => false,
        }
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
            PropAttr::Default { default } => {
                let name = &self.name;
                let ty = &self.ty;
                let span = default.span();
                // Hacks to avoid misleading error message.
                quote_spanned! {span=>
                    #name: {
                        match true {
                            #[allow(unreachable_code)]
                            false => {
                                let __unreachable: #ty = ::std::unreachable!();
                                __unreachable
                            },
                            true => #default()
                        }
                    },
                }
            }
            PropAttr::None => {
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

    // Detect `#[props(required)]` or `#[props(default="...")]` attribute
    fn attribute(named_field: &syn::Field) -> Result<PropAttr> {
        let meta_list = if let Some(meta_list) = Self::find_props_meta_list(named_field) {
            meta_list
        } else {
            return Ok(PropAttr::None);
        };

        let expected_attr = syn::Error::new(
            meta_list.span(),
            "expected `props(required)` or `#[props(default=\"...\")]`",
        );
        let first_nested = if let Some(first_nested) = meta_list.nested.first() {
            first_nested
        } else {
            return Err(expected_attr);
        };
        match first_nested {
            NestedMeta::Meta(Meta::Path(word_path)) => {
                if !word_path.is_ident("required") {
                    return Err(expected_attr);
                }

                if let Some(ident) = &named_field.ident {
                    let wrapped_name = Ident::new(&format!("{}_wrapper", ident), Span::call_site());
                    Ok(PropAttr::Required { wrapped_name })
                } else {
                    unreachable!()
                }
            }
            NestedMeta::Meta(Meta::NameValue(name_value)) => {
                let MetaNameValue { path, lit, .. } = name_value;

                if !path.is_ident("default") {
                    return Err(expected_attr);
                }

                if let Lit::Str(lit_str) = lit {
                    let default = lit_str.parse()?;
                    Ok(PropAttr::Default { default })
                } else {
                    Err(expected_attr)
                }
            }
            _ => Err(expected_attr),
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
