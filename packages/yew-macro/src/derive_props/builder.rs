//! The `PropsBuilder` constructs props in alphabetical order and enforces that required props have
//! been set before allowing the build to complete. Each property has a corresponding method in the
//! builder. Required property builder methods advance the builder to the next step, optional
//! properties can be added or skipped with no effect on the build step. Once all of required
//! properties have been set, the builder moves to the final build step which implements the
//! `build()` method.

use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote_spanned, Attribute, GenericParam};

use super::generics::to_arguments;
use super::DerivePropsInput;
use crate::derive_props::generics::push_type_param;

pub struct PropsBuilder<'a> {
    builder_name: &'a Ident,
    props: &'a DerivePropsInput,
    wrapper_name: &'a Ident,
    check_all_props_name: &'a Ident,
    extra_attrs: &'a [Attribute],
}

impl ToTokens for PropsBuilder<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            builder_name,
            props,
            wrapper_name,
            ..
        } = self;

        let DerivePropsInput { vis, generics, .. } = props;

        let assert_all_props = self.impl_assert_props();

        let (_, ty_generics, where_clause) = generics.split_for_impl();

        let builder = quote! {
            #[doc(hidden)]
            #vis struct #builder_name #generics
                #where_clause
            {
                wrapped: ::std::boxed::Box<#wrapper_name #ty_generics>,
            }

            #assert_all_props
        };

        tokens.extend(builder);
    }
}

impl<'a> PropsBuilder<'_> {
    pub fn new(
        name: &'a Ident,
        props: &'a DerivePropsInput,
        wrapper_name: &'a Ident,
        check_all_props_name: &'a Ident,
        extra_attrs: &'a [Attribute],
    ) -> PropsBuilder<'a> {
        PropsBuilder {
            builder_name: name,
            props,
            wrapper_name,
            check_all_props_name,
            extra_attrs,
        }
    }
}

impl PropsBuilder<'_> {
    fn set_fields(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.props.prop_fields.iter().map(|pf| pf.to_field_setter())
    }

    fn impl_assert_props(&self) -> proc_macro2::TokenStream {
        let Self {
            builder_name,
            check_all_props_name,
            extra_attrs,
            ..
        } = self;
        let DerivePropsInput {
            vis,
            generics,
            props_name,
            prop_fields,
            ..
        } = self.props;

        let set_fields = self.set_fields();
        let prop_fns = prop_fields
            .iter()
            .map(|pf| pf.to_build_step_fn(vis, props_name));

        let (builder_impl_generics, ty_generics, builder_where_clause) = generics.split_for_impl();
        let turbofish_generics = ty_generics.as_turbofish();
        let generic_args = to_arguments(generics);

        let mut assert_impl_generics = generics.clone();
        let token_arg: GenericParam = parse_quote_spanned! {Span::mixed_site()=>
            __YewToken
        };
        push_type_param(&mut assert_impl_generics, token_arg.clone());
        let assert_impl_generics = assert_impl_generics;
        let (impl_generics, _, where_clause) = assert_impl_generics.split_for_impl();

        let props_mod_name = format_ident!("_{}", props_name, span = Span::mixed_site());
        let mut check_impl_generics = assert_impl_generics.clone();
        let mut check_args = vec![];
        let mut check_props = proc_macro2::TokenStream::new();
        let prop_field_decls = prop_fields
            .iter()
            .map(|pf| pf.to_field_check(props_name, vis, &token_arg))
            .collect::<Vec<_>>();
        let prop_name_decls = prop_field_decls.iter().map(|pf| pf.to_fake_prop_decl());
        for pf in prop_field_decls.iter() {
            check_props.extend(pf.to_stream(
                &mut check_impl_generics,
                &mut check_args,
                &props_mod_name,
            ));
        }
        let (check_impl_generics, _, check_where_clause) = check_impl_generics.split_for_impl();

        quote! {
            #[automatically_derived]
            #( #extra_attrs )*
            impl #builder_impl_generics #builder_name<#generic_args> #builder_where_clause {
                #( #prop_fns )*
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            #vis mod #props_mod_name {
                #( #prop_name_decls )*
            }
            #check_props

            #[doc(hidden)]
            #vis struct #check_all_props_name<How>(::std::marker::PhantomData<How>);

            #[automatically_derived]
            impl<B, P, How> ::yew::html::HasProp<P, &dyn ::yew::html::HasProp<P, How>>
                for #check_all_props_name<B>
                where B: ::yew::html::HasProp<P, How> {}

            #[automatically_derived]
            impl #check_impl_generics ::yew::html::HasAllProps<
                #props_name #ty_generics ,
                ( #( #check_args , )* ),
            > for #check_all_props_name< #token_arg > #check_where_clause {
            }

            #[automatically_derived]
            impl #impl_generics ::yew::html::Buildable< #token_arg > for #builder_name<#generic_args> #where_clause {
                type Output = #props_name #ty_generics;
                type WrappedToken = #check_all_props_name< #token_arg >;
                fn build(this: Self) -> Self::Output {
                    #props_name #turbofish_generics {
                        #(#set_fields)*
                    }
                }
            }
        }
    }
}
