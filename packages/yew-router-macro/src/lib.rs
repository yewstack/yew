use heck::ShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Ident, LitStr, Variant, Visibility,
};

const AT_PATH: &str = "at";

struct Routable {
    vis: Visibility,
    ident: Ident,
    ats: Vec<(Ident, LitStr)>,
    variants: Punctuated<Variant, syn::token::Comma>,
}

impl Parse for Routable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let DeriveInput {
            vis, ident, data, ..
        } = input.parse()?;

        let data = match data {
            Data::Enum(data) => data,
            Data::Struct(s) => return Err(syn::Error::new_spanned(
                s.struct_token,
                "expected enum, found struct",
            )),
            Data::Union(u) => return Err(syn::Error::new_spanned(
                u.union_token,
                "expected enum, found union",
            )),
        };

        let mut ats = vec![];

        for variant in data.variants.iter() {
            let attrs = variant
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident(AT_PATH))
                .collect::<Vec<_>>();
            let attr = match attrs.len() {
                1 => *attrs.first().unwrap(),
                0 => return  Err(syn::Error::new(
                    variant.span(),
                    format!("{} attribute must be present on every variant", AT_PATH),
                )),
                _ => return Err(syn::Error::new(
                    variant.span(),
                    format!("only one {} attribute must be present", AT_PATH),
                )),
            };

            if let Fields::Unnamed(_) = variant.fields {
                return Err(syn::Error::new(
                    variant.span(),
                    "only named fields are supported for dynamic paths",
                ))
            }

            let lit = attr.parse_args::<LitStr>()?;
            let ident = variant.ident.to_string().to_shouty_snake_case();
            let ident = Ident::new(&ident, variant.ident.span());
            ats.push((ident, lit.clone()));
        }

        Ok(Self {
            vis,
            ident,
            variants: data.variants,
            ats,
        })
    }
}

fn routable_derive_impl(input: Routable) -> TokenStream {
    let Routable {
        vis,
        ident,
        ats,
        variants,
    } = input;

    let at_consts = ats.iter().map(|(name, value)| {
        quote! {
            #vis const #name: &'static str = #value;
        }
    });

    let ats = ats.iter().map(|(_, lit)| lit).collect::<Vec<_>>();

    let from_path_matches = variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        let right = match &variant.fields {
            Fields::Unit => quote! { Self::#ident },
            Fields::Named(field) => {
                let fields = field.named.iter().map(|it| it.ident.as_ref().unwrap());
                quote! { Self::#ident { #(#fields: params.get(stringify!(#fields))?.parse().ok()?)*, } }
            }
            Fields::Unnamed(_) => unreachable!(), // already checked
        };

        let left = ats.get(i).unwrap();
        quote! {
            #left => ::core::option::Option::Some(#right)
        }
    });

    let to_route_matches = variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        let mut right = ats.get(i).unwrap().value();

        match &variant.fields {
            Fields::Unit => quote! {
                Self::#ident => #right.to_string()
            },
            Fields::Named(field) => {
                let fields = field
                    .named
                    .iter()
                    .map(|it| it.ident.as_ref().unwrap())
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

    quote! {
        #[automatically_derived]
        impl #ident {
            #(#at_consts)*
        }

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

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }
        }
    }
}

#[proc_macro_derive(Routable, attributes(at))]
pub fn routable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Routable);
    routable_derive_impl(input).into()
}
