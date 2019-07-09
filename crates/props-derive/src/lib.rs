#![recursion_limit = "128"]
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use std::iter;
use syn::parse_macro_input;
use syn::punctuated;
use syn::spanned::Spanned;
use syn::{
    DeriveInput, Error, GenericParam, Generics, Meta, MetaList, NestedMeta, Type, TypeParam,
    Visibility, WhereClause,
};

struct PropField {
    ty: Type,
    name: Ident,
    wrapped_name: Option<Ident>,
}

#[proc_macro_derive(Properties, attributes(props))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let props_name = input.ident;
    let vis = input.vis;

    let generics = input.generics;
    let generic_params = &generics.params;
    let generic_where = &generics.where_clause;
    let generic_idents = {
        let generic_idents = generics.params.iter().filter_map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => Some(quote! { #ident }),
            _ => unimplemented!(),
        });

        quote! {#(#generic_idents),*}
    };

    let named_fields = match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => fields,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let prop_fields: Vec<PropField> = {
        let res: Result<Vec<PropField>, Error> = named_fields
            .named
            .into_iter()
            .map(|field| {
                Ok(PropField {
                    wrapped_name: required_wrapper(&field)?,
                    ty: field.ty,
                    name: field.ident.unwrap(),
                })
            })
            .collect();

        match res {
            Err(err) => return TokenStream::from(err.to_compile_error()),
            Ok(mut prop_fields) => {
                // Alphabetize
                prop_fields.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
                prop_fields
            }
        }
    };

    // Build Idents
    let step_name = Ident::new(&format!("{}Step", props_name), Span::call_site());
    let wrapped_name = Ident::new(&format!("Wrapped{}", props_name), Span::call_site());
    let builder_name = Ident::new(&format!("{}Builder", props_name), Span::call_site());
    let mut step_names: Vec<Ident> = prop_fields
        .iter()
        .filter(|field| field.wrapped_name.is_some())
        .map(|field| {
            Ident::new(
                &format!("{}_{}_is_required", props_name, field.name),
                Span::call_site(),
            )
        })
        .collect();

    step_names.push(Ident::new(
        &format!("{}BuildStep", props_name),
        Span::call_site(),
    ));

    let start_step_name = &step_names[0];
    let build_step_name = &step_names[step_names.len() - 1];
    let all_step_names = &step_names;

    let step_methods = step_methods(
        &vis,
        &generics,
        &generic_idents,
        &generic_where,
        &builder_name,
        &step_names,
        &prop_fields,
    );
    let wrapped_fields = wrapped_fields(prop_fields.iter());
    let wrapped_default_setters = wrapped_default_setters(prop_fields.iter());
    let prop_field_setters = prop_field_setters(prop_fields.iter());
    let step_name_impls = step_name_impls(&step_name, step_names.iter());
    let vis_repeat = iter::repeat(&vis);

    let expanded = quote! {
        #(
            #[doc(hidden)]
            #vis_repeat struct #all_step_names;
        )*

        #[doc(hidden)]
        #vis trait #step_name {}
        #(#step_name_impls)*

        struct #wrapped_name#generics {
            #(#wrapped_fields)*
        }

        impl#generics ::std::default::Default for #wrapped_name<#generic_idents> #generic_where {
            fn default() -> Self {
                #wrapped_name::<#generic_idents> {
                    #(#wrapped_default_setters)*
                }
            }
        }

        #[doc(hidden)]
        #vis struct #builder_name<P: #step_name, #generic_params> #generic_where {
            wrapped: ::std::boxed::Box<#wrapped_name<#generic_idents>>,
            _marker: ::std::marker::PhantomData<P>,
        }

        impl #generics ::yew::html::Properties for #props_name<#generic_idents> #generic_where {
            type Builder = #builder_name<#start_step_name, #generic_idents>;

            fn builder() -> Self::Builder {
                #builder_name {
                    wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
                    _marker: ::std::marker::PhantomData,
                }
            }
        }

        #(#step_methods)*

        impl #generics #builder_name<#build_step_name, #generic_idents> #generic_where {
            #[doc(hidden)]
            #vis fn build(self) -> #props_name<#generic_idents> {
                #props_name::<#generic_idents> {
                    #(#prop_field_setters)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn wrapped_fields<'a>(
    prop_fields: impl Iterator<Item = &'a PropField>,
) -> impl Iterator<Item = impl ToTokens + 'a> {
    prop_fields.map(|pf| {
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

fn wrapped_default_setters<'a>(
    prop_fields: impl Iterator<Item = &'a PropField>,
) -> impl Iterator<Item = impl ToTokens + 'a> {
    prop_fields.map(|pf| {
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

fn prop_field_setters<'a>(
    prop_fields: impl Iterator<Item = &'a PropField>,
) -> impl Iterator<Item = impl ToTokens + 'a> {
    prop_fields.map(|pf| {
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

fn required_wrapper(named_field: &syn::Field) -> Result<Option<Ident>, Error> {
    let meta_list = if let Some(meta_list) = find_props_meta_list(named_field) {
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

fn step_name_impls<'a>(
    step_trait_name: &'a Ident,
    step_names: impl Iterator<Item = &'a Ident>,
) -> impl Iterator<Item = impl ToTokens + 'a> {
    step_names.map(move |name| {
        let trait_name = step_trait_name;
        quote! {
            impl #trait_name for #name {}
        }
    })
}

fn step_methods<'a>(
    vis: &'a Visibility,
    generics: &'a Generics,
    generic_idents: &'a proc_macro2::TokenStream,
    generic_where: &'a Option<WhereClause>,
    builder_name: &'a Ident,
    step_names: &'a [Ident],
    prop_fields: &'a [PropField],
) -> proc_macro2::TokenStream {
    let mut prop_fields_index = 0;
    let mut token_stream = proc_macro2::TokenStream::new();

    for (step, step_name) in step_names.iter().enumerate() {
        let mut optional_fields = Vec::new();
        let mut required_field = None;

        if prop_fields_index >= prop_fields.len() {
            break;
        }

        while let Some(pf) = prop_fields.get(prop_fields_index) {
            prop_fields_index += 1;
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
                #vis fn #prop_name(mut self, #prop_name: #prop_type) -> #builder_name<#step_name, #generic_idents> {
                    self.wrapped.#prop_name = #prop_name;
                    self
                }
            }
        });

        let required_prop_fn = required_field.iter().map(|p| {
            let prop_name = &p.name;
            let prop_type = &p.ty;
            let wrapped_name = p.wrapped_name.as_ref().unwrap();
            let next_step_name = &step_names[step + 1];

            quote! {
                #[doc(hidden)]
                #vis fn #prop_name(mut self, #prop_name: #prop_type) -> #builder_name<#next_step_name, #generic_idents> {
                    self.wrapped.#wrapped_name = ::std::option::Option::Some(#prop_name);
                    #builder_name {
                        wrapped: self.wrapped,
                        _marker: ::std::marker::PhantomData,
                    }
                }
            }
        });

        token_stream.extend(quote! {
            impl #generics #builder_name<#step_name, #generic_idents> #generic_where {
                #(#optional_prop_fn)*
                #(#required_prop_fn)*
            }
        });
    }
    token_stream
}
