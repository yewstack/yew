use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Variant, Visibility};

const AT_ATTR_IDENT: &str = "at";
const NOT_FOUND_ATTR_IDENT: &str = "not_found";

pub struct Routable {
    vis: Visibility,
    ident: Ident,
    ats: Vec<LitStr>,
    variants: Punctuated<Variant, syn::token::Comma>,
    not_found_route: Option<LitStr>,
}

impl Parse for Routable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let DeriveInput {
            vis, ident, data, ..
        } = input.parse()?;

        let data = match data {
            Data::Enum(data) => data,
            Data::Struct(s) => {
                return Err(syn::Error::new_spanned(
                    s.struct_token,
                    "expected enum, found struct",
                ))
            }
            Data::Union(u) => {
                return Err(syn::Error::new_spanned(
                    u.union_token,
                    "expected enum, found union",
                ))
            }
        };

        let (not_found_route, ats) = parse_variants(&data.variants)?;

        Ok(Self {
            vis,
            ident,
            variants: data.variants,
            ats,
            not_found_route,
        })
    }
}

fn parse_variants(
    variants: &Punctuated<Variant, syn::token::Comma>,
) -> syn::Result<(Option<LitStr>, Vec<LitStr>)> {
    let mut not_founds = vec![];
    let mut ats: Vec<LitStr> = vec![];

    for variant in variants.iter() {
        let variant: &syn::Variant = variant;

        if let Fields::Unnamed(_) = variant.fields {
            return Err(syn::Error::new(
                variant.span(),
                "only named fields are supported",
            ));
        }

        let attrs = &variant.attrs;
        let at_attrs = attrs
            .iter()
            .filter(|attr| attr.path.is_ident(AT_ATTR_IDENT))
            .collect::<Vec<_>>();

        let attr = match at_attrs.len() {
            1 => *at_attrs.first().expect("fucked"),
            0 => {
                return Err(syn::Error::new(
                    variant.span(),
                    format!(
                        "{} attribute must be present on every variant",
                        AT_ATTR_IDENT
                    ),
                ))
            }
            _ => {
                return Err(syn::Error::new(
                    variant.span(),
                    format!("only one {} attribute must be present", AT_ATTR_IDENT),
                ))
            }
        };

        let lit = attr.parse_args::<LitStr>()?;
        ats.push(lit.clone());

        for attr in attrs.iter() {
            if attr.path.is_ident(NOT_FOUND_ATTR_IDENT) {
                let at_attr = at_attrs
                    .iter()
                    .find(|attr| attr.path.is_ident(AT_ATTR_IDENT));
                let at_attr = match at_attr {
                    Some(at_attr) => at_attr,
                    None => {
                        return Err(syn::Error::new(
                            attr.span(),
                            format!(
                                "fields marked with {} must have {} attribute",
                                NOT_FOUND_ATTR_IDENT, AT_ATTR_IDENT
                            ),
                        ))
                    }
                };

                not_founds.push(at_attr.parse_args::<LitStr>()?)
            }
        }
    }

    if not_founds.len() > 1 {
        return Err(syn::Error::new(
            Span::call_site(),
            format!("there can only be one {}", NOT_FOUND_ATTR_IDENT),
        ));
    }

    Ok((not_founds.first().cloned(), ats))
}

pub fn routable_derive_impl(input: Routable) -> TokenStream {
    let Routable {
        vis: _vis, // todo
        ident,
        ats,
        variants,
        not_found_route,
    } = input;

    let from_path_matches = variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        let right = match &variant.fields {
            Fields::Unit => quote! { Self::#ident },
            Fields::Named(field) => {
                let fields = field.named.iter().map(|it| it.ident.as_ref().expect("fucked 4"));
                quote! { Self::#ident { #(#fields: params.get(stringify!(#fields))?.parse().ok()?)*, } }
            }
            Fields::Unnamed(_) => unreachable!(), // already checked
        };

        let left = ats.get(i).expect("fucked 1");
        quote! {
            #left => ::core::option::Option::Some(#right)
        }
    });

    let to_route_matches = variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        let mut right = ats.get(i).expect("fucked 2").value();

        match &variant.fields {
            Fields::Unit => quote! {
                Self::#ident => #right.to_string()
            },
            Fields::Named(field) => {
                let fields = field
                    .named
                    .iter()
                    .map(|it| it.ident.as_ref().expect("fucked 3"))
                    .collect::<Vec<_>>();

                fields.iter().for_each(|field| {
                    right = right.replace(&format!(":{}", field), &format!("{{{}}}", field))
                });

                quote! {
                    Self::#ident { #(#fields),* } => ::std::format!(#right, #(#fields = #fields),*)
                }
            }
            Fields::Unnamed(_) => unreachable!(), // already checked
        }
    });

    let not_found_route = match not_found_route {
        Some(route) => quote! { ::std::option::Option::Some(#route) },
        None => quote! { ::std::option::Option::None },
    };

    quote! {
        #[automatically_derived]
        impl ::yew_router::Routable for #ident {
            fn from_path(path: &str, params: &::std::collections::HashMap<&str, &str>) -> Option<Self> {
                match path {
                    #(#from_path_matches),*,
                    _ => None,
                }
            }

            fn to_route(&self) -> String {
                match self {
                    #(#to_route_matches),*,
                }
            }

            fn routes() -> ::std::vec::Vec<&'static str> {
                ::std::vec![#(#ats),*]
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn not_found_route() -> ::std::option::Option<&'static str> {
                #not_found_route
            }
        }
    }
}
