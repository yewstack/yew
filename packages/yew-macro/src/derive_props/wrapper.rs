use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics};

use super::PropField;

pub struct PropsWrapper<'a> {
    wrapper_name: &'a Ident,
    generics: &'a Generics,
    prop_fields: &'a [PropField],
    extra_attrs: &'a [Attribute],
}

impl ToTokens for PropsWrapper<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            generics,
            wrapper_name,
            extra_attrs,
            ..
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let turbofish_generics = ty_generics.as_turbofish();

        let wrapper_field_defs = self.field_defs();
        let wrapper_default_setters = self.default_setters();

        let wrapper = quote! {
            #[doc(hidden)]
            #(#extra_attrs)*
            struct #wrapper_name #generics
                #where_clause
            {
                #(#wrapper_field_defs)*
            }

            #[automatically_derived]
            impl #impl_generics ::std::default::Default for #wrapper_name #ty_generics #where_clause {
                fn default() -> Self {
                    #wrapper_name #turbofish_generics {
                        #(#wrapper_default_setters)*
                    }
                }
            }
        };
        wrapper.to_tokens(tokens);
    }
}

impl<'a> PropsWrapper<'_> {
    pub fn new(
        name: &'a Ident,
        generics: &'a Generics,
        prop_fields: &'a [PropField],
        extra_attrs: &'a [Attribute],
    ) -> PropsWrapper<'a> {
        PropsWrapper {
            wrapper_name: name,
            generics,
            prop_fields,
            extra_attrs,
        }
    }
}

impl PropsWrapper<'_> {
    fn field_defs(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.prop_fields.iter().map(|pf| pf.to_field_def())
    }

    fn default_setters(&self) -> impl Iterator<Item = impl ToTokens + '_> {
        self.prop_fields.iter().map(|pf| pf.to_default_setter())
    }
}
