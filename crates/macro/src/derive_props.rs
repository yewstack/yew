use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use std::convert::{TryFrom, TryInto};
use std::iter;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated;
use syn::spanned::Spanned;
use syn::{
    DeriveInput, Error, Field, GenericParam, Generics, Meta, MetaList, NestedMeta, Type, TypeParam,
    Visibility,
};

struct PropField {
    ty: Type,
    name: Ident,
    wrapped_name: Option<Ident>,
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
        prop_fields.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

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
            vis,
            generics,
            props_name,
            ..
        } = self;
        let generic_params = &generics.params;
        let generic_where = &generics.where_clause;
        let generic_types = self.generic_types();

        let wrapped_name = Ident::new(&format!("Wrapped{}", props_name), Span::call_site());
        let wrapped_field_defs = self.wrapped_field_defs();
        let wrapped_default_setters = self.wrapped_default_setters();

        let builder_name = Ident::new(&format!("{}Builder", props_name), Span::call_site());
        let builder_step = Ident::new(&format!("{}BuilderStep", props_name), Span::call_site());
        let builder_step_names = self.builder_step_names();
        let builder_start_step = &builder_step_names[0];
        let builder_build_step = &builder_step_names[builder_step_names.len() - 1];
        let builder_steps = &builder_step_names;
        let builder_step_repeat = iter::repeat(&builder_step);
        let impl_builder_for_steps = self.impl_builder_for_steps(&builder_name, &builder_steps);
        let builder_set_fields = self.builder_set_fields();
        let vis_repeat = iter::repeat(&vis);

        let expanded = quote! {
            struct #wrapped_name#generics {
                #(#wrapped_field_defs)*
            }

            impl#generics ::std::default::Default for #wrapped_name<#generic_types> #generic_where {
                fn default() -> Self {
                    #wrapped_name::<#generic_types> {
                        #(#wrapped_default_setters)*
                    }
                }
            }

            #(
                #[doc(hidden)]
                #vis_repeat struct #builder_steps;
            )*

            #[doc(hidden)]
            #vis trait #builder_step {}

            #(impl #builder_step_repeat for #builder_steps {})*

            #[doc(hidden)]
            #vis struct #builder_name<YEW_PROPS_BUILDER_STEP: #builder_step, #generic_params> #generic_where {
                wrapped: ::std::boxed::Box<#wrapped_name<#generic_types>>,
                _marker: ::std::marker::PhantomData<YEW_PROPS_BUILDER_STEP>,
            }

            #(#impl_builder_for_steps)*

            impl #generics #builder_name<#builder_build_step, #generic_types> #generic_where {
                #[doc(hidden)]
                #vis fn build(self) -> #props_name<#generic_types> {
                    #props_name::<#generic_types> {
                        #(#builder_set_fields)*
                    }
                }
            }

            impl #generics ::yew::html::Properties for #props_name<#generic_types> #generic_where {
                type Builder = #builder_name<#builder_start_step, #generic_types>;

                fn builder() -> Self::Builder {
                    #builder_name {
                        wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                        _marker: ::std::marker::PhantomData,
                    }
                }
            }
        };

        tokens.extend(proc_macro2::TokenStream::from(expanded));
    }
}

impl PropField {
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

        let word_ident = match first_nested {
            punctuated::Pair::End(NestedMeta::Meta(Meta::Word(ident))) => ident,
            _ => return Err(expected_required),
        };

        if word_ident != "required" {
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

        if meta_list.ident == "props" {
            Some(meta_list)
        } else {
            None
        }
    }
}

impl DerivePropsInput {
    fn generic_types(&self) -> proc_macro2::TokenStream {
        let generic_types = self.generics.params.iter().map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => ident,
            _ => unimplemented!("only generic types are supported"),
        });
        quote! {#(#generic_types),*}
    }

    fn builder_step_names(&self) -> Vec<Ident> {
        let mut step_names: Vec<Ident> = self
            .prop_fields
            .iter()
            .filter(|prop_field| prop_field.wrapped_name.is_some())
            .map(|prop_field| {
                Ident::new(
                    &format!("{}_{}_is_required", self.props_name, prop_field.name),
                    Span::call_site(),
                )
            })
            .collect();

        step_names.push(Ident::new(
            &format!("{}BuildStep", self.props_name),
            Span::call_site(),
        ));

        step_names
    }

    fn wrapped_field_defs(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.prop_fields.iter().map(|pf| {
            let PropField { name, ty, .. } = &pf;
            if let Some(wrapped_name) = &pf.wrapped_name {
                quote! {
                    #wrapped_name: ::std::option::Option<#ty>,
                }
            } else {
                quote! {
                    #name: #ty,
                }
            }
        })
    }

    fn wrapped_default_setters(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.prop_fields.iter().map(|pf| {
            if let Some(wrapped_name) = &pf.wrapped_name {
                quote! {
                    #wrapped_name: ::std::default::Default::default(),
                }
            } else {
                let name = &pf.name;
                quote! {
                    #name: ::std::default::Default::default(),
                }
            }
        })
    }

    fn builder_set_fields(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.prop_fields.iter().map(|pf| {
            let name = &pf.name;
            if let Some(wrapped_name) = &pf.wrapped_name {
                quote! {
                    #name: self.wrapped.#wrapped_name.unwrap(),
                }
            } else {
                quote! {
                    #name: self.wrapped.#name,
                }
            }
        })
    }

    fn impl_builder_for_steps(
        &self,
        builder_name: &Ident,
        builder_step_names: &[Ident],
    ) -> proc_macro2::TokenStream {
        let Self { vis, generics, .. } = self;
        let generic_types = self.generic_types();
        let generic_where = &generics.where_clause;

        let mut fields_index = 0;
        let mut token_stream = proc_macro2::TokenStream::new();

        for (step, step_name) in builder_step_names.iter().enumerate() {
            let mut optional_fields = Vec::new();
            let mut required_field = None;

            if fields_index >= self.prop_fields.len() {
                break;
            }

            while let Some(pf) = self.prop_fields.get(fields_index) {
                fields_index += 1;
                if pf.wrapped_name.is_some() {
                    required_field = Some(pf);
                    break;
                } else {
                    optional_fields.push((&pf.name, &pf.ty));
                }
            }

            let optional_prop_fn = optional_fields.into_iter().map(|(prop_name, prop_type)| {
                quote! {
                    #[doc(hidden)]
                    #vis fn #prop_name(mut self, #prop_name: #prop_type) -> #builder_name<#step_name, #generic_types> {
                        self.wrapped.#prop_name = #prop_name;
                        self
                    }
                }
            });

            let required_prop_fn = required_field.iter().map(|p| {
                let prop_name = &p.name;
                let prop_type = &p.ty;
                let wrapped_name = p.wrapped_name.as_ref().unwrap();
                let next_step_name = &builder_step_names[step + 1];

                quote! {
                    #[doc(hidden)]
                    #vis fn #prop_name(mut self, #prop_name: #prop_type) -> #builder_name<#next_step_name, #generic_types> {
                        self.wrapped.#wrapped_name = ::std::option::Option::Some(#prop_name);
                        #builder_name {
                            wrapped: self.wrapped,
                            _marker: ::std::marker::PhantomData,
                        }
                    }
                }
            });

            token_stream.extend(quote! {
                impl #generics #builder_name<#step_name, #generic_types> #generic_where {
                    #(#optional_prop_fn)*
                    #(#required_prop_fn)*
                }
            });
        }
        token_stream
    }
}
