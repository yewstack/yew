use crate::switch::{
    shadow::ShadowMatcherToken, unnamed_field_index_item, write_for_token, FieldType, SwitchItem,
};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Fields;

pub struct BuildRouteSection<'a> {
    pub switch_items: &'a [SwitchItem],
    pub enum_ident: &'a Ident,
    pub match_item: &'a Ident,
}

impl<'a> ToTokens for BuildRouteSection<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let serializer =
            build_serializer_for_enum(self.switch_items, self.enum_ident, self.match_item);

        tokens.extend(quote!{
            fn build_route_section<__T>(self, mut buf: &mut ::std::string::String) -> ::std::option::Option<__T> {
                #serializer
            }
        });
    }
}

/// The serializer makes up the body of `build_route_section`.
pub fn build_serializer_for_enum(
    switch_items: &[SwitchItem],
    enum_ident: &Ident,
    match_item: &Ident,
) -> TokenStream {
    let variants = switch_items.iter().map(|switch_item: &SwitchItem| {
        let SwitchItem {
            matcher,
            ident,
            fields,
        } = switch_item;
        match fields {
            Fields::Named(fields_named) => {
                let field_names = fields_named
                    .named
                    .iter()
                    .filter_map(|named| named.ident.as_ref());
                let writers = matcher
                    .iter()
                    .map(|token| write_for_token(token, FieldType::Named));
                quote! {
                    #enum_ident::#ident{#(#field_names),*} => {
                        #(#writers)*
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_names = fields_unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, _)| unnamed_field_index_item(index));
                let mut item_count = 0;
                let writers = matcher.iter().map(|token| {
                    if let ShadowMatcherToken::Capture(_) = &token {
                        let ts = write_for_token(token, FieldType::Unnamed { index: item_count });
                        item_count += 1;
                        ts
                    } else {
                        // Its either a literal, or something that will panic currently
                        write_for_token(token, FieldType::Unit)
                    }
                });
                quote! {
                    #enum_ident::#ident(#(#field_names),*) => {
                        #(#writers)*
                    }
                }
            }
            Fields::Unit => {
                let writers = matcher
                    .iter()
                    .map(|token| write_for_token(token, FieldType::Unit));
                quote! {
                    #enum_ident::#ident => {
                        #(#writers)*
                    }
                }
            }
        }
    });
    quote! {
        use ::std::fmt::Write as __Write;
        let mut state: Option<__T> = None;
        match #match_item {
            #(#variants)*,
        }

        state
    }
}
