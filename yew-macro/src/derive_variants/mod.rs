mod variant;

use proc_macro2::{Ident};
use quote::{quote, ToTokens};
use std::convert::TryInto;
use syn::parse::{Parse, ParseStream, Result};
use syn::{DeriveInput, Generics, Visibility};
use variant::VariantsVariant;

pub struct DeriveVariantsInput {
    vis: Visibility,
    generics: Generics,
    variants_name: Ident,
    variants_variants: Vec<VariantsVariant>,
}

impl Parse for DeriveVariantsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let variants = match input.data {
            syn::Data::Enum(data) => data.variants,
            _ => unimplemented!("only enums are supported"),
        };
        let variants_variants: Vec<VariantsVariant> = variants
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<VariantsVariant>>>()?;
        Ok(DeriveVariantsInput {
            vis: input.vis,
            variants_name: input.ident,
            generics: input.generics,
            variants_variants,
        })
    }
}

impl ToTokens for DeriveVariantsInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let variants_name = &self.variants_name;
        for variant in self.variants_variants.iter() {
            let variant_ty = &variant.ty;
            let variant_name = &variant.name;
            let variant = quote! {
                impl#impl_generics ::std::convert::From<#variant_ty> for #variants_name#ty_generics #where_clause {
                    fn from(v: #variant_ty) -> Self {
                        Self::#variant_name(v)
                    }
                }
            };
            tokens.extend(variant);
        }
    }
}

