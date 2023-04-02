mod builder;
mod field;
mod generics;
mod wrapper;

use std::convert::TryInto;

use builder::PropsBuilder;
use field::PropField;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, DeriveInput, Generics, Visibility};
use wrapper::PropsWrapper;

use self::generics::to_arguments;

pub struct DerivePropsInput {
    vis: Visibility,
    generics: Generics,
    props_name: Ident,
    prop_fields: Vec<PropField>,
    preserved_attrs: Vec<Attribute>,
}

/// Some attributes on the original struct are to be preserved and added to the builder struct,
/// in order to avoid warnings (sometimes reported as errors) in the output.
fn should_preserve_attr(attr: &Attribute) -> bool {
    // #[cfg(...)]: does not usually appear in macro inputs, but rust-analyzer seems to generate it
    // sometimes.              If not preserved, results in "no-such-field" errors generating
    // the field setter for `build` #[allow(...)]: silences warnings from clippy, such as
    // dead_code etc. #[deny(...)]: enable additional warnings from clippy
    let path = attr.path();
    path.is_ident("allow") || path.is_ident("deny") || path.is_ident("cfg")
}

impl Parse for DerivePropsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let prop_fields = match input.data {
            syn::Data::Struct(data) => match data.fields {
                syn::Fields::Named(fields) => {
                    let mut prop_fields: Vec<PropField> = fields
                        .named
                        .into_iter()
                        .map(|f| f.try_into())
                        .collect::<Result<Vec<PropField>>>()?;

                    // Alphabetize
                    prop_fields.sort();

                    prop_fields
                }
                syn::Fields::Unit => Vec::new(),
                _ => unimplemented!("only structs are supported"),
            },
            _ => unimplemented!("only structs are supported"),
        };

        let preserved_attrs = input
            .attrs
            .iter()
            .filter(|a| should_preserve_attr(a))
            .cloned()
            .collect();

        Ok(Self {
            vis: input.vis,
            props_name: input.ident,
            generics: input.generics,
            prop_fields,
            preserved_attrs,
        })
    }
}

impl ToTokens for DerivePropsInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            generics,
            props_name,
            ..
        } = self;

        // The wrapper is a new struct which wraps required props in `Option`
        let wrapper_name = format_ident!("{}Wrapper", props_name, span = Span::mixed_site());
        let wrapper = PropsWrapper::new(
            &wrapper_name,
            generics,
            &self.prop_fields,
            &self.preserved_attrs,
        );
        tokens.extend(wrapper.into_token_stream());

        // The builder will only build if all required props have been set
        let builder_name = format_ident!("{}Builder", props_name, span = Span::mixed_site());
        let check_all_props_name =
            format_ident!("Check{}All", props_name, span = Span::mixed_site());
        let builder = PropsBuilder::new(
            &builder_name,
            self,
            &wrapper_name,
            &check_all_props_name,
            &self.preserved_attrs,
        );
        let generic_args = to_arguments(generics);
        tokens.extend(builder.into_token_stream());

        // The properties trait has a `builder` method which creates the props builder
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let properties = quote! {
            impl #impl_generics ::yew::html::Properties for #props_name #ty_generics #where_clause {
                type Builder = #builder_name<#generic_args>;

                fn builder() -> Self::Builder {
                    #builder_name {
                        wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                    }
                }
            }
        };
        tokens.extend(properties);
    }
}
