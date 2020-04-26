//! The `PropsBuilder` constructs props in alphabetical order and enforces that required props have
//! been set before allowing the build to complete. Each property has a corresponding method in the
//! builder. Required property builder methods advance the builder to the next step, optional
//! properties can be added or skipped with no effect on the build step. Once all of required
//! properties have been set, the builder moves to the final build step which implements the
//! `build()` method.

use super::generics::{to_arguments, with_param_bounds, GenericArguments};
use super::{DerivePropsInput, PropField};
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};

pub struct PropsBuilder<'a> {
    builder_name: &'a Ident,
    step_trait: &'a Ident,
    step_names: Vec<Ident>,
    props: &'a DerivePropsInput,
    wrapper_name: &'a Ident,
}

impl ToTokens for PropsBuilder<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            builder_name,
            step_trait,
            step_names,
            props,
            wrapper_name,
        } = self;

        let DerivePropsInput {
            vis,
            generics,
            props_name,
            ..
        } = props;

        let build_step = self.build_step();
        let impl_steps = self.impl_steps();
        let set_fields = self.set_fields();

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let turbofish_generics = ty_generics.as_turbofish();
        let generic_args = to_arguments(&generics, build_step.clone());

        // Each builder step implements the `BuilderStep` trait and `step_generics` is used to
        // enforce that.
        let step_generic_param = Ident::new("YEW_PROPS_BUILDER_STEP", Span::call_site());
        let step_generics =
            with_param_bounds(&generics, step_generic_param.clone(), (*step_trait).clone());

        let builder = quote! {
            #(
                #[doc(hidden)]
                #vis struct #step_names;
            )*

            #[doc(hidden)]
            #vis trait #step_trait {}

            #(impl #step_trait for #step_names {})*

            #[doc(hidden)]
            #vis struct #builder_name#step_generics
                #where_clause
            {
                wrapped: ::std::boxed::Box<#wrapper_name#ty_generics>,
                _marker: ::std::marker::PhantomData<#step_generic_param>,
            }

            #impl_steps

            impl#impl_generics #builder_name<#generic_args> #where_clause {
                #[doc(hidden)]
                #vis fn build(self) -> #props_name#ty_generics {
                    #props_name#turbofish_generics {
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
        step_trait: &'a Ident,
        props: &'a DerivePropsInput,
        wrapper_name: &'a Ident,
    ) -> PropsBuilder<'a> {
        PropsBuilder {
            builder_name: name,
            step_trait,
            step_names: Self::build_step_names(step_trait, &props.prop_fields),
            props,
            wrapper_name,
        }
    }
}

impl PropsBuilder<'_> {
    pub fn first_step_generic_args(&self) -> GenericArguments {
        to_arguments(&self.props.generics, self.first_step().clone())
    }

    fn first_step(&self) -> &Ident {
        &self.step_names[0]
    }

    fn build_step(&self) -> &Ident {
        &self.step_names[self.step_names.len() - 1]
    }

    fn build_step_names(prefix: &Ident, prop_fields: &[PropField]) -> Vec<Ident> {
        let mut step_names: Vec<Ident> = prop_fields
            .iter()
            .filter(|pf| pf.is_required())
            .map(|pf| pf.to_step_name(prefix))
            .collect();
        step_names.push(Ident::new(&format!("{}_build", prefix), Span::call_site()));
        step_names
    }

    fn set_fields(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.props.prop_fields.iter().map(|pf| pf.to_field_setter())
    }

    fn impl_steps(&self) -> proc_macro2::TokenStream {
        let Self {
            builder_name,
            props,
            step_names,
            ..
        } = self;
        let DerivePropsInput {
            vis,
            generics,
            prop_fields,
            ..
        } = props;

        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let mut fields_index = 0;
        let mut token_stream = proc_macro2::TokenStream::new();

        for (step, step_name) in step_names.iter().enumerate() {
            let mut optional_fields = Vec::new();
            let mut required_field = None;

            if fields_index >= prop_fields.len() {
                break;
            }

            while let Some(pf) = prop_fields.get(fields_index) {
                fields_index += 1;
                if pf.is_required() {
                    required_field = Some(pf);
                    break;
                } else {
                    optional_fields.push(pf);
                }
            }

            // Optional properties keep the builder on the current step
            let current_step_arguments = to_arguments(generics, step_name.clone());
            let optional_prop_fn = optional_fields
                .iter()
                .map(|pf| pf.to_build_step_fn(builder_name, &current_step_arguments, vis));

            // Required properties will advance the builder to the next step
            let required_prop_fn = required_field.iter().map(|pf| {
                let next_step_name = &step_names[step + 1];
                let next_step_arguments = to_arguments(generics, next_step_name.clone());
                pf.to_build_step_fn(builder_name, &next_step_arguments, vis)
            });

            token_stream.extend(quote! {
                impl#impl_generics #builder_name<#current_step_arguments> #where_clause {
                    #(#optional_prop_fn)*
                    #(#required_prop_fn)*
                }
            });
        }
        token_stream
    }
}
