use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;

use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, quote_spanned};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{
    parse_quote, Attribute, Error, Expr, Field, GenericParam, Generics, Path, Type, TypePath,
    Visibility,
};

use super::should_preserve_attr;
use crate::derive_props::generics::push_type_param;

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
    extra_attrs: Vec<Attribute>,
}

impl PropField {
    #[cfg(not(yew_lints))]
    pub fn lint(&self) {}

    #[cfg(yew_lints)]
    pub fn lint(&self) {
        match &self.ty {
            Type::Path(TypePath { qself: None, path }) => {
                if is_path_a_string(path) {
                    proc_macro_error::emit_warning!(
                    path.span(),
                    "storing string values with `String` is not recommended, prefer `AttrValue`.\n\
                     for further info visit \
                     https://yew.rs/docs/concepts/function-components/properties#anti-patterns"
                )
                }
            }
            _ => (),
        }
    }

    /// All required property fields are wrapped in an `Option`
    pub fn is_required(&self) -> bool {
        matches!(self.attr, PropAttr::Required { .. })
    }

    /// This check name is descriptive to help a developer realize they missed a required prop
    fn to_check_name(&self, props_name: &Ident) -> Ident {
        format_ident!("Has{}{}", props_name, self.name, span = Span::mixed_site())
    }

    /// This check name is descriptive to help a developer realize they missed a required prop
    fn to_check_arg_name(&self, props_name: &Ident) -> GenericParam {
        let ident = format_ident!("How{}{}", props_name, self.name, span = Span::mixed_site());
        GenericParam::Type(ident.into())
    }

    /// Ident of the wrapped field name
    fn wrapped_name(&self) -> &Ident {
        match &self.attr {
            PropAttr::Required { wrapped_name } => wrapped_name,
            _ => &self.name,
        }
    }

    pub fn to_field_check<'a>(
        &'a self,
        props_name: &'a Ident,
        vis: &'a Visibility,
        token: &'a GenericParam,
    ) -> PropFieldCheck<'_> {
        let check_struct = self.to_check_name(props_name);
        let check_arg = self.to_check_arg_name(props_name);
        PropFieldCheck {
            this: self,
            vis,
            token,
            check_struct,
            check_arg,
        }
    }

    /// Used to transform the `PropWrapper` struct into `Properties`
    pub fn to_field_setter(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let setter = match &self.attr {
            PropAttr::Required { wrapped_name } => {
                quote! {
                    #name: ::std::option::Option::unwrap(this.wrapped.#wrapped_name),
                }
            }
            PropAttr::Option => {
                quote! {
                    #name: this.wrapped.#name,
                }
            }
            PropAttr::PropOr(value) => {
                quote_spanned! {value.span()=>
                    #name: ::std::option::Option::unwrap_or(this.wrapped.#name, #value),
                }
            }
            PropAttr::PropOrElse(func) => {
                quote_spanned! {func.span()=>
                    #name: ::std::option::Option::unwrap_or_else(this.wrapped.#name, #func),
                }
            }
            PropAttr::PropOrDefault => {
                quote! {
                    #name: ::std::option::Option::unwrap_or_default(this.wrapped.#name),
                }
            }
        };
        let extra_attrs = &self.extra_attrs;
        quote! {
            #( #extra_attrs )*
            #setter
        }
    }

    /// Wrap all required props in `Option`
    pub fn to_field_def(&self) -> proc_macro2::TokenStream {
        let ty = &self.ty;
        let extra_attrs = &self.extra_attrs;
        let wrapped_name = self.wrapped_name();
        match &self.attr {
            PropAttr::Option => {
                quote! {
                    #( #extra_attrs )*
                    #wrapped_name: #ty,
                }
            }
            _ => {
                quote! {
                    #( #extra_attrs )*
                    #wrapped_name: ::std::option::Option<#ty>,
                }
            }
        }
    }

    /// All optional props must implement the `Default` trait
    pub fn to_default_setter(&self) -> proc_macro2::TokenStream {
        let wrapped_name = self.wrapped_name();
        let extra_attrs = &self.extra_attrs;
        quote! {
            #( #extra_attrs )*
            #wrapped_name: ::std::option::Option::None,
        }
    }

    /// Each field is set using a builder method
    pub fn to_build_step_fn(
        &self,
        vis: &Visibility,
        props_name: &Ident,
    ) -> proc_macro2::TokenStream {
        let Self { name, ty, attr, .. } = self;
        let token_ty = Ident::new("__YewTokenTy", Span::mixed_site());
        let build_fn = match attr {
            PropAttr::Required { wrapped_name } => {
                let check_struct = self.to_check_name(props_name);
                quote! {
                    #[doc(hidden)]
                    #vis fn #name<#token_ty>(
                        &mut self,
                        token: #token_ty,
                        value: impl ::yew::html::IntoPropValue<#ty>,
                    ) -> #check_struct< #token_ty > {
                        self.wrapped.#wrapped_name = ::std::option::Option::Some(value.into_prop_value());
                        #check_struct ( ::std::marker::PhantomData )
                    }
                }
            }
            PropAttr::Option => {
                quote! {
                    #[doc(hidden)]
                    #vis fn #name<#token_ty>(
                        &mut self,
                        token: #token_ty,
                        value: impl ::yew::html::IntoPropValue<#ty>,
                    ) -> #token_ty {
                        self.wrapped.#name = value.into_prop_value();
                        token
                    }
                }
            }
            _ => {
                quote! {
                    #[doc(hidden)]
                    #vis fn #name<#token_ty>(
                        &mut self,
                        token: #token_ty,
                        value: impl ::yew::html::IntoPropValue<#ty>,
                    ) -> #token_ty {
                        self.wrapped.#name = ::std::option::Option::Some(value.into_prop_value());
                        token
                    }
                }
            }
        };
        let extra_attrs = &self.extra_attrs;
        quote! {
            #( #extra_attrs )*
            #build_fn
        }
    }

    // Detect Properties 2.0 attributes
    fn attribute(named_field: &Field) -> Result<PropAttr> {
        let attr = named_field.attrs.iter().find(|attr| {
            attr.path().is_ident("prop_or")
                || attr.path().is_ident("prop_or_else")
                || attr.path().is_ident("prop_or_default")
        });

        if let Some(attr) = attr {
            if attr.path().is_ident("prop_or") {
                Ok(PropAttr::PropOr(attr.parse_args()?))
            } else if attr.path().is_ident("prop_or_else") {
                Ok(PropAttr::PropOrElse(attr.parse_args()?))
            } else if attr.path().is_ident("prop_or_default") {
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
            let wrapped_name = format_ident!("{}_wrapper", ident, span = Span::mixed_site());
            Ok(PropAttr::Required { wrapped_name })
        }
    }
}

pub struct PropFieldCheck<'a> {
    this: &'a PropField,
    vis: &'a Visibility,
    token: &'a GenericParam,
    check_struct: Ident,
    check_arg: GenericParam,
}

impl<'a> PropFieldCheck<'a> {
    pub fn to_fake_prop_decl(&self) -> proc_macro2::TokenStream {
        let Self { this, .. } = self;
        if !this.is_required() {
            return Default::default();
        }
        let mut prop_check_name = this.name.clone();
        prop_check_name.set_span(Span::mixed_site());
        quote! {
            #[allow(non_camel_case_types)]
            pub struct #prop_check_name;
        }
    }

    pub fn to_stream(
        &self,
        type_generics: &mut Generics,
        check_args: &mut Vec<GenericParam>,
        prop_name_mod: &Ident,
    ) -> proc_macro2::TokenStream {
        let Self {
            this,
            vis,
            token,
            check_struct,
            check_arg,
        } = self;
        if !this.is_required() {
            return Default::default();
        }
        let mut prop_check_name = this.name.clone();
        prop_check_name.set_span(Span::mixed_site());
        check_args.push(check_arg.clone());
        push_type_param(type_generics, check_arg.clone());
        let where_clause = type_generics.make_where_clause();
        where_clause.predicates.push(parse_quote! {
            #token: ::yew::html::HasProp< #prop_name_mod :: #prop_check_name, #check_arg >
        });

        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #vis struct #check_struct<How>(::std::marker::PhantomData<How>);

            #[automatically_derived]
            impl<B> ::yew::html::HasProp< #prop_name_mod :: #prop_check_name, #check_struct<B>>
                for #check_struct<B> {}
            #[automatically_derived]
            impl<B, P, How> ::yew::html::HasProp<P, &dyn ::yew::html::HasProp<P, How>>
                for #check_struct<B>
                where B: ::yew::html::HasProp<P, How> {}

        }
    }
}

fn is_path_segments_an_option<'a, T>(mut iter: impl Iterator<Item = &'a T>) -> bool
where
    T: 'a + ?Sized + PartialEq<str>,
{
    iter.next().map_or(false, |first| {
        first == "Option"
            || (first == "std" || first == "core")
                && iter.next().map_or(false, |second| second == "option")
                && iter.next().map_or(false, |third| third == "Option")
    })
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
    is_path_segments_an_option(path.segments.iter().map(|ps| &ps.ident))
}

#[cfg(any(yew_lints, test))]
fn is_path_segments_a_string<'a, T>(mut iter: impl Iterator<Item = &'a T>) -> bool
where
    T: 'a + ?Sized + PartialEq<str>,
{
    iter.next().map_or(false, |first| {
        first == "String"
            || (first == "std" || first == "alloc")
                && iter.next().map_or(false, |second| second == "string")
                && iter.next().map_or(false, |third| third == "String")
    })
}

/// returns true when the [`Path`] seems like a [`String`] type.
///
/// This function considers the following paths as Strings:
/// - std::string::String
/// - alloc::string::String
/// - String
#[cfg(yew_lints)]
fn is_path_a_string(path: &Path) -> bool {
    is_path_segments_a_string(path.segments.iter().map(|ps| &ps.ident))
}

impl TryFrom<Field> for PropField {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self> {
        let extra_attrs = field
            .attrs
            .iter()
            .filter(|a| should_preserve_attr(a))
            .cloned()
            .collect();

        Ok(PropField {
            attr: Self::attribute(&field)?,
            extra_attrs,
            ty: field.ty,
            name: field.ident.unwrap(),
        })
    }
}

impl PartialOrd for PropField {
    fn partial_cmp(&self, other: &PropField) -> Option<Ordering> {
        Some(self.cmp(other))
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
    use std::iter::once;

    use crate::derive_props::field::{is_path_segments_a_string, is_path_segments_an_option};

    #[test]
    fn all_std_and_core_option_path_seg_return_true() {
        assert!(is_path_segments_an_option(
            ["core", "option", "Option"].into_iter()
        ));
        assert!(is_path_segments_an_option(
            ["std", "option", "Option"].into_iter()
        ));
        assert!(is_path_segments_an_option(once("Option")));
        // why OR instead of XOR
        assert!(is_path_segments_an_option(
            ["Option", "Vec", "Option"].into_iter()
        ));
    }

    #[test]
    fn all_std_and_alloc_string_seg_return_true() {
        assert!(is_path_segments_a_string(
            ["alloc", "string", "String"].into_iter()
        ));
        assert!(is_path_segments_a_string(
            ["std", "string", "String"].into_iter()
        ));
        assert!(is_path_segments_a_string(once("String")));
    }
}
