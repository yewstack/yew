//! The `PropsBuilder` constructs props in alphabetical order and enforces that required props have
//! been set before allowing the build to complete. Each property has a corresponding method in the
//! builder. Required property builder methods advance the builder to the next step, optional
//! properties can be added or skipped with no effect on the build step. Once all of required
//! properties have been set, the builder moves to the final build step which implements the
//! `build()` method.

use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::Attribute;

use super::generics::{to_arguments, GenericArguments};
use super::{DerivePropsInput, PropField};

pub struct PropsBuilder<'a> {
    builder_name: &'a Ident,
    step_names: Vec<Ident>,
    props: &'a DerivePropsInput,
    wrapper_name: &'a Ident,
    extra_attrs: &'a [Attribute],
}

impl ToTokens for PropsBuilder<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            builder_name,
            step_names,
            props,
            wrapper_name,
            ..
        } = self;

        let DerivePropsInput {
            vis,
            generics,
            props_name,
            ..
        } = props;

        let impl_steps = self.impl_steps();
        let set_fields = self.set_fields();

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let turbofish_generics = ty_generics.as_turbofish();
        let generic_args = to_arguments(generics);

        let builder = quote! {
            #(
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                #vis struct #step_names;
            )*

            #[doc(hidden)]
            #vis struct #builder_name #generics
                #where_clause
            {
                wrapped: ::std::boxed::Box<#wrapper_name #ty_generics>,
            }

            #impl_steps

            #[automatically_derived]
            impl #impl_generics ::yew::html::Buildable for #builder_name<#generic_args> #where_clause {
                type Output = #props_name #ty_generics;
                fn build(this: Self) -> Self::Output {
                    #props_name #turbofish_generics {
                        #(#set_fields)*
                    }
                }
            }
        };

        tokens.extend(builder);
    }
}

impl<'a> PropsBuilder<'_> {
    pub fn new(
        name: &'a Ident,
        prefix: &'a Ident,
        props: &'a DerivePropsInput,
        wrapper_name: &'a Ident,
        extra_attrs: &'a [Attribute],
    ) -> PropsBuilder<'a> {
        PropsBuilder {
            builder_name: name,
            step_names: Self::build_step_names(prefix, &props.prop_fields),
            props,
            wrapper_name,
            extra_attrs,
        }
    }
}

impl PropsBuilder<'_> {
    pub fn first_step_generic_args(&self) -> GenericArguments {
        to_arguments(&self.props.generics)
    }

    fn build_step_names(prefix: &Ident, prop_fields: &[PropField]) -> Vec<Ident> {
        let mut step_names: Vec<Ident> = prop_fields
            .iter()
            .filter(|pf| pf.is_required())
            .map(|pf| pf.to_step_name(prefix))
            .collect();
        step_names.push(format_ident!(
            "{}PropsBuilder",
            prefix,
            span = prefix.span(),
        ));
        step_names
    }

    fn set_fields(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.props.prop_fields.iter().map(|pf| pf.to_field_setter())
    }

    fn impl_steps(&self) -> proc_macro2::TokenStream {
        let Self {
            builder_name,
            props,
            extra_attrs,
            ..
        } = self;
        let DerivePropsInput {
            vis,
            generics,
            prop_fields,
            ..
        } = props;

        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let mut token_stream = proc_macro2::TokenStream::new();

        {
            let generic_args = to_arguments(generics);
            let prop_fns = prop_fields.iter().map(|pf| pf.to_build_step_fn(vis));
            token_stream.extend(quote! {
                #[automatically_derived]
                #( #extra_attrs )*
                impl #impl_generics #builder_name<#generic_args> #where_clause {
                    #( #prop_fns )*
                }
            });
        }
        // FIXME: steps and required prop validation
        token_stream
    }
}
