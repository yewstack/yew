mod builder;
mod field;
mod generics;
mod wrapper;

use builder::PropsBuilder;
use field::PropField;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use std::convert::TryInto;
use syn::parse::{Parse, ParseStream, Result};
use syn::{DeriveInput, Generics, Visibility};
use wrapper::PropsWrapper;

pub struct DerivePropsInput {
    vis: Visibility,
    generics: Generics,
    props_name: Ident,
    prop_fields: Vec<PropField>,
}

impl Parse for DerivePropsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let named_fields = match input.data {
            syn::Data::Struct(data) => match data.fields {
                syn::Fields::Named(fields) => fields.named,
                _ => unimplemented!("only structs are supported"),
            },
            _ => unimplemented!("only structs are supported"),
        };

        let mut prop_fields: Vec<PropField> = named_fields
            .into_iter()
            .map(|f| f.try_into())
            .collect::<Result<Vec<PropField>>>()?;

        // Alphabetize
        prop_fields.sort();

        Ok(Self {
            vis: input.vis,
            props_name: input.ident,
            generics: input.generics,
            prop_fields,
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
        let wrapper_name = Ident::new(&format!("{}Wrapper", props_name), Span::call_site());
        let wrapper = PropsWrapper::new(&wrapper_name, &generics, &self.prop_fields);
        tokens.extend(wrapper.into_token_stream());

        // The builder will only build if all required props have been set
        let builder_name = Ident::new(&format!("{}Builder", props_name), Span::call_site());
        let builder_step = Ident::new(&format!("{}BuilderStep", props_name), Span::call_site());
        let builder = PropsBuilder::new(&builder_name, &builder_step, &self, &wrapper_name);
        let builder_generic_args = builder.first_step_generic_args();
        tokens.extend(builder.into_token_stream());

        // The properties trait has a `builder` method which creates the props builder
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let properties = quote! {
            impl#impl_generics ::yew::html::Properties for #props_name#ty_generics #where_clause {
                type Builder = #builder_name<#builder_generic_args>;

                fn builder() -> Self::Builder {
                    #builder_name {
                        wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                        _marker: ::std::marker::PhantomData,
                    }
                }
            }
        };
        tokens.extend(properties);
    }
}
