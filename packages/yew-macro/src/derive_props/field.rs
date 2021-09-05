use super::generics::GenericArguments;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Error, Expr, Field, Path, Type, TypePath, Visibility};

#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq)]
enum PropAttr {
    Required { wrapped_name: Ident },
    Option,
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

    /// Ident of the wrapped field name
    fn wrapped_name(&self) -> &Ident {
        match &self.attr {
            PropAttr::Required { wrapped_name } => wrapped_name,
            _ => &self.name,
        }
    }

    /// Used to transform the `PropWrapper` struct into `Properties`
    pub fn to_field_setter(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        match &self.attr {
            PropAttr::Required { wrapped_name } => {
                quote! {
                    #name: ::std::option::Option::unwrap(self.wrapped.#wrapped_name),
                }
            }
            PropAttr::Option => {
                quote! {
                    #name: self.wrapped.#name,
                }
            }
            PropAttr::PropOr(value) => {
                quote_spanned! {value.span()=>
                    #name: ::std::option::Option::unwrap_or(self.wrapped.#name, #value),
                }
            }
            PropAttr::PropOrElse(func) => {
                quote_spanned! {func.span()=>
                    #name: ::std::option::Option::unwrap_or_else(self.wrapped.#name, #func),
                }
            }
            PropAttr::PropOrDefault => {
                quote! {
                    #name: ::std::option::Option::unwrap_or_default(self.wrapped.#name),
                }
            }
        }
    }

    /// Wrap all required props in `Option`
    pub fn to_field_def(&self) -> proc_macro2::TokenStream {
        let ty = &self.ty;
        let wrapped_name = self.wrapped_name();
        match &self.attr {
            PropAttr::Option => {
                quote! {
                    #wrapped_name: #ty,
                }
            }
            _ => {
                quote! {
                    #wrapped_name: ::std::option::Option<#ty>,
                }
            }
        }
    }

    /// All optional props must implement the `Default` trait
    pub fn to_default_setter(&self) -> proc_macro2::TokenStream {
        let wrapped_name = self.wrapped_name();
        quote! {
            #wrapped_name: ::std::option::Option::None,
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
                    #vis fn #name(mut self, #name: impl ::yew::html::IntoPropValue<#ty>) -> #builder_name<#generic_arguments> {
                        self.wrapped.#wrapped_name = ::std::option::Option::Some(#name.into_prop_value());
                        #builder_name {
                            wrapped: self.wrapped,
                            _marker: ::std::marker::PhantomData,
                        }
                    }
                }
            }
            PropAttr::Option => {
                quote! {
                    #[doc(hidden)]
                    #vis fn #name(mut self, #name: impl ::yew::html::IntoPropValue<#ty>) -> #builder_name<#generic_arguments> {
                        self.wrapped.#name = #name.into_prop_value();
                        self
                    }
                }
            }
            _ => {
                quote! {
                    #[doc(hidden)]
                    #vis fn #name(mut self, #name: impl ::yew::html::IntoPropValue<#ty>) -> #builder_name<#generic_arguments> {
                        self.wrapped.#name = ::std::option::Option::Some(#name.into_prop_value());
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
        } else if matches!(
            &named_field.ty,
            Type::Path(TypePath { path, .. })
            if is_path_an_option(path)
        ) {
            Ok(PropAttr::Option)
        } else {
            let ident = named_field.ident.as_ref().unwrap();
            let wrapped_name = Ident::new(&format!("{}_wrapper", ident), Span::call_site());
            Ok(PropAttr::Required { wrapped_name })
        }
    }
}

fn is_path_segments_an_option(path_segments: impl Iterator<Item = String>) -> bool {
    fn is_option_path_seg(seg_index: usize, path: &str) -> u8 {
        match (seg_index, path) {
            (0, "core") => 0b001,
            (0, "std") => 0b001,
            (0, "Option") => 0b111,
            (1, "option") => 0b010,
            (2, "Option") => 0b100,
            _ => 0,
        }
    }

    path_segments
        .enumerate()
        .fold(0, |flags, (i, ps)| flags | is_option_path_seg(i, &ps))
        == 0b111
}

/// Returns true when the [`Path`] seems like an [`Option`] type.
///
/// This function considers the following paths as Options:
/// - core::option::Option
/// - std::option::Option
/// - Option::*
///
/// Users can define their own [`Option`] type and this will return true - this is unavoidable.
fn is_path_an_option(path: &Path) -> bool {
    is_path_segments_an_option(path.segments.iter().take(3).map(|ps| ps.ident.to_string()))
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

#[cfg(test)]
mod tests {
    use crate::derive_props::field::is_path_segments_an_option;

    #[test]
    fn all_std_and_core_option_path_seg_return_true() {
        assert!(is_path_segments_an_option(
            vec!["core".to_owned(), "option".to_owned(), "Option".to_owned()].into_iter()
        ));
        assert!(is_path_segments_an_option(
            vec!["std".to_owned(), "option".to_owned(), "Option".to_owned()].into_iter()
        ));
        assert!(is_path_segments_an_option(
            vec!["Option".to_owned()].into_iter()
        ));
        // why OR instead of XOR
        assert!(is_path_segments_an_option(
            vec!["Option".to_owned(), "Vec".to_owned(), "Option".to_owned()].into_iter()
        ));
    }
}
