use crate::switch::shadow::{ShadowCaptureVariant, ShadowMatcherToken};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Ident, Variant};

mod attribute;
mod enum_impl;
mod shadow;
mod struct_impl;
mod switch_impl;

use self::{attribute::AttrToken, switch_impl::SwitchImpl};
use crate::switch::{enum_impl::EnumInner, struct_impl::StructInner};
use yew_router_route_parser::FieldNamingScheme;

/// Holds data that is required to derive Switch for a struct or a single enum variant.
pub struct SwitchItem {
    pub matcher: Vec<ShadowMatcherToken>,
    pub ident: Ident,
    pub fields: Fields,
}

pub fn switch_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident: Ident = input.ident;
    let generics = input.generics;

    Ok(match input.data {
        Data::Struct(ds) => {
            let field_naming_scheme = match ds.fields {
                Fields::Unnamed(_) => FieldNamingScheme::Unnamed,
                Fields::Unit => FieldNamingScheme::Unit,
                Fields::Named(_) => FieldNamingScheme::Named,
            };
            let matcher = AttrToken::convert_attributes_to_tokens(input.attrs)?
                .into_iter()
                .enumerate()
                .map(|(index, at)| at.into_shadow_matcher_tokens(index, field_naming_scheme))
                .flatten()
                .collect::<Vec<_>>();

            let item = SwitchItem {
                matcher,
                ident: ident.clone(), // TODO make SwitchItem take references instead.
                fields: ds.fields,
            };

            SwitchImpl {
                target_ident: &ident,
                generics: &generics,
                inner: StructInner {
                    from_route_part: struct_impl::FromRoutePart(&item),
                    build_route_section: struct_impl::BuildRouteSection {
                        switch_item: &item,
                        item: &Ident::new("self", Span::call_site()),
                    },
                },
            }
            .to_token_stream()
        }
        Data::Enum(de) => {
            let switch_variants = de
                .variants
                .into_iter()
                .map(|variant: Variant| {
                    let field_type = match variant.fields {
                        Fields::Unnamed(_) => yew_router_route_parser::FieldNamingScheme::Unnamed,
                        Fields::Unit => FieldNamingScheme::Unit,
                        Fields::Named(_) => yew_router_route_parser::FieldNamingScheme::Named,
                    };
                    let matcher = AttrToken::convert_attributes_to_tokens(variant.attrs)?
                        .into_iter()
                        .enumerate()
                        .map(|(index, at)| at.into_shadow_matcher_tokens(index, field_type))
                        .flatten()
                        .collect::<Vec<_>>();
                    Ok(SwitchItem {
                        matcher,
                        ident: variant.ident,
                        fields: variant.fields,
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;

            SwitchImpl {
                target_ident: &ident,
                generics: &generics,
                inner: EnumInner {
                    from_route_part: enum_impl::FromRoutePart {
                        switch_variants: &switch_variants,
                        enum_ident: &ident,
                    },
                    build_route_section: enum_impl::BuildRouteSection {
                        switch_items: &switch_variants,
                        enum_ident: &ident,
                        match_item: &Ident::new("self", Span::call_site()),
                    },
                },
            }
            .to_token_stream()
        }
        Data::Union(_du) => panic!("Deriving FromCaptures not supported for Unions."),
    })
}

trait Flatten<T> {
    /// Because flatten is a nightly feature. I'm making a new variant of the function here for
    /// stable use. The naming is changed to avoid this getting clobbered when object_flattening
    /// 60258 is stabilized.
    fn flatten_stable(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten_stable(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

fn build_matcher_from_tokens(tokens: &[ShadowMatcherToken]) -> TokenStream {
    quote! {
        let settings = ::yew_router::matcher::MatcherSettings {
            case_insensitive: true,
        };
        let matcher = ::yew_router::matcher::RouteMatcher {
            tokens: ::std::vec![#(#tokens),*],
            settings
        };
    }
}

/// Enum indicating which sort of writer is needed.
pub(crate) enum FieldType {
    Named,
    Unnamed { index: usize },
    Unit,
}

/// This assumes that the variant/struct has been destructured.
fn write_for_token(token: &ShadowMatcherToken, naming_scheme: FieldType) -> TokenStream {
    match token {
        ShadowMatcherToken::Exact(lit) => {
            quote! {
                write!(buf, "{}", #lit).unwrap();
            }
        }
        ShadowMatcherToken::Capture(capture) => match naming_scheme {
            FieldType::Named | FieldType::Unit => match &capture {
                ShadowCaptureVariant::Named(name)
                | ShadowCaptureVariant::ManyNamed(name)
                | ShadowCaptureVariant::NumberedNamed { name, .. } => {
                    let name = Ident::new(&name, Span::call_site());
                    quote! {
                        state = state.or_else(|| #name.build_route_section(buf));
                    }
                }
                ShadowCaptureVariant::Unnamed
                | ShadowCaptureVariant::ManyUnnamed
                | ShadowCaptureVariant::NumberedUnnamed { .. } => {
                    panic!("Unnamed matcher sections not allowed for named field types")
                }
            },
            FieldType::Unnamed { index } => {
                let name = unnamed_field_index_item(index);
                quote! {
                    state = state.or_else(|| #name.build_route_section(&mut buf));
                }
            }
        },
        ShadowMatcherToken::End => quote! {},
    }
}

/// Creates an ident used for destructuring unnamed fields.
///
/// There needs to be a unified way to "mangle" the unnamed fields so they can be destructured,
fn unnamed_field_index_item(index: usize) -> Ident {
    Ident::new(&format!("__field_{}", index), Span::call_site())
}
