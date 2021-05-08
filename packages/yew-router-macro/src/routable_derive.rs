use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Meta, Variant};

const AT_ATTR_IDENT: &str = "at";
const BIND_ATTR_IDENT: &str = "bind";

fn hidden_module() -> TokenStream {
    quote! { ::yew_router::hidden }
}

pub struct Routable {
    ident: Ident,
    variants: Vec<ParsedVariant>,
}

impl Parse for Routable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let DeriveInput { ident, data, .. } = input.parse()?;

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

        let variants = parse_variants(&ident, &data.variants)?;

        Ok(Self { ident, variants })
    }
}

#[derive(Debug)]
struct ParsedField {
    name: String,
    index: usize,
    meta: Option<Meta>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum BindMode {
    Rest,
    Path,
    QueryArg,
    HashArg,
}

enum BindType {
    String,
    Rest,
}

impl BindType {
    fn decoder(&self) -> TokenStream {
        let hidden = hidden_module();
        match self {
            BindType::Rest => quote! { #hidden::Routable::from_route(args.take_route()).ok() },
            BindType::String => quote! { args.pop_str() },
        }
    }
    fn encoder(&self, arg: &Ident) -> TokenStream {
        let hidden = hidden_module();
        match self {
            BindType::Rest => quote! { args.store_route(#hidden::Routable::to_route(#arg)); },
            BindType::String => quote! { args.push_str(#arg); },
        }
    }
}

struct ConstMuncher {
    muncher: Option<TokenStream>,
    binds: Vec<(usize, BindType)>,
}

fn generate_path_muncher(fields: &[ParsedField], at: &ParsedAt) -> syn::Result<ConstMuncher> {
    let hidden = hidden_module();
    let mut binds = Vec::new();
    for name in &at.names {
        if let Some(field) = fields.iter().find(|field| &field.name == name) {
            binds.push((field.index, BindType::String));
        } else {
            return Err(syn::Error::new(
                at.span,
                format_args!("No field named {:?}", name),
            ));
        }
    }
    let parts: TokenStream = at
        .parts
        .iter()
        .map(|part| match part {
            PathPart::Match(s) => quote! { #hidden::PathPart::Match(#s), },
            PathPart::ExtractOne => quote! { #hidden::PathPart::ExtractOne, },
            PathPart::ExtractAll => quote! { #hidden::PathPart::ExtractAll, },
        })
        .collect();
    Ok(ConstMuncher {
        muncher: Some(quote! {
            #hidden::PathSegmentMuncher {
                parts: &[#parts]
            }
        }),
        binds,
    })
}

fn generate_arg_muncher(muncher: &Ident, fields: &[ParsedField]) -> syn::Result<ConstMuncher> {
    let hidden = hidden_module();
    let mut binds = Vec::new();
    let mut names = Vec::new();
    for field in fields {
        let name = match &field.meta {
            Some(syn::Meta::Path(_)) => field.name.clone(),
            Some(syn::Meta::NameValue(syn::MetaNameValue {
                lit: syn::Lit::Str(name),
                ..
            })) => name.value(),
            _ => {
                return Err(syn::Error::new(
                    field.meta.span(),
                    "Invalid argument binding",
                ))
            }
        };
        names.push(name);
        binds.push((field.index, BindType::String));
    }
    Ok(ConstMuncher {
        muncher: Some(quote! {
            #hidden::#muncher {
                names: &[#(#names,)*]
            }
        }),
        binds,
    })
}

fn generate_rest_glue(fields: &[ParsedField]) -> syn::Result<ConstMuncher> {
    let binds = match fields {
        [field] => vec![(field.index, BindType::Rest)],
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Only one `rest` binding can be used",
            ))
        }
    };

    Ok(ConstMuncher {
        muncher: None,
        binds,
    })
}

impl BindMode {
    fn from_path(path: &syn::Path) -> syn::Result<Self> {
        Ok(if path.is_ident("query_arg") {
            Self::QueryArg
        } else if path.is_ident("hash_arg") {
            Self::HashArg
        } else if path.is_ident("rest") {
            Self::Rest
        } else {
            return Err(syn::Error::new(
                path.span(),
                format!("Unknown binding mode: {:?}", path),
            ));
        })
    }
    fn parse_and_gen(&self, fields: Vec<ParsedField>, at: &ParsedAt) -> syn::Result<ConstMuncher> {
        match self {
            BindMode::Path => generate_path_muncher(&fields, at),
            BindMode::QueryArg => {
                generate_arg_muncher(&Ident::new("QueryArgMuncher", Span::call_site()), &fields)
            }
            BindMode::HashArg => {
                generate_arg_muncher(&Ident::new("HashArgMuncher", Span::call_site()), &fields)
            }
            BindMode::Rest => generate_rest_glue(&fields),
        }
    }
}

fn parse_fields(fields: &Fields, at: &mut ParsedAt) -> syn::Result<Vec<ConstMuncher>> {
    let mut map = BTreeMap::<_, Vec<_>>::default();

    map.insert(BindMode::Path, Vec::new());
    for (index, field) in fields.into_iter().enumerate() {
        let name = if let Some(ident) = &field.ident {
            ident.to_string()
        } else {
            index.to_string()
        };
        let bind_attrs: Vec<_> = field
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident(BIND_ATTR_IDENT))
            .collect();
        match bind_attrs.as_slice() {
            [] => {
                if !at.names.contains(&name) {
                    return Err(syn::Error::new(field.span(), "Unbound field"));
                }

                map.entry(BindMode::Path).or_default().push(ParsedField {
                    index,
                    meta: None,
                    name,
                })
            }
            [attr] => {
                let nested_meta = match attr.parse_meta()? {
                    syn::Meta::List(mut ml) if ml.nested.len() == 1 => {
                        ml.nested.pop().unwrap().into_value()
                    }
                    _ => return Err(syn::Error::new(field.span(), "Invalid binding attribute")),
                };
                let meta = match nested_meta {
                    syn::NestedMeta::Meta(meta) => meta,
                    _ => return Err(syn::Error::new(field.span(), "Invalid binding attribute")),
                };
                let bind_mode = BindMode::from_path(meta.path())?;
                map.entry(bind_mode).or_default().push(ParsedField {
                    index,
                    meta: Some(meta),
                    name,
                });
            }
            _ => {
                return Err(syn::Error::new(
                    field.span(),
                    "Only one bind attribute allowed per field",
                ))
            }
        }
    }

    if map.contains_key(&BindMode::Rest) {
        // If we're binding the rest of the route to a field,
        // allow the path to continue.
        if at.regex.is_empty() {
            at.regex += "/";
        } else {
            at.regex += "(?:/|$)";
        }
    } else {
        // Otherwise, require the path to terminate.
        if at.regex.is_empty() {
            at.regex += "/$";
        } else {
            at.regex += "/?$";
        }
    }

    let mut res = Vec::new();
    for (k, v) in map {
        res.push(k.parse_and_gen(v, at)?);
    }

    Ok(res)
}

enum PathPart {
    Match(String),
    ExtractOne,
    ExtractAll,
}

struct ParsedAt {
    span: Span,
    regex: String,
    names: Vec<String>,
    parts: Vec<PathPart>,
}

fn parse_at(span: Span, at: &str) -> syn::Result<ParsedAt> {
    if !at.starts_with("/") {
        return Err(syn::Error::new(span, "Url must begin with /"));
    }

    let mut regex = "^".to_string();
    let mut names = Vec::new();
    let mut parts = Vec::new();

    for part in at[1..].split("/") {
        if part.is_empty() {
            continue;
        } else if let Some(name) = part.strip_prefix(":") {
            regex += "/([^/]+)";
            names.push(name.to_string());
            parts.push(PathPart::ExtractOne);
        } else if let Some(name) = part.strip_prefix("*") {
            regex += "/(.+)";
            names.push(name.to_string());
            parts.push(PathPart::ExtractAll);
        } else {
            regex += "/";
            regex += &regex::escape(part);
            parts.push(PathPart::Match(part.into()));
        }
    }

    if regex.is_empty() {
        regex += "/";
    } else {
        regex += "(?:/|$)";
    }

    Ok(ParsedAt {
        regex,
        names,
        parts,
        span,
    })
}

struct ParsedVariant {
    parsed_at: ParsedAt,
    munchers: Vec<ConstMuncher>,
    pattern: TokenStream,
}

fn parse_variants(
    ident: &Ident,
    variants: &Punctuated<Variant, syn::token::Comma>,
) -> syn::Result<Vec<ParsedVariant>> {
    let mut res = Vec::new();

    for variant in variants.iter() {
        let variant: &syn::Variant = variant;

        let attrs = &variant.attrs;
        let at_attrs = attrs
            .iter()
            .filter(|attr| attr.path.is_ident(AT_ATTR_IDENT))
            .collect::<Vec<_>>();

        let at_attr = if let [attr] = at_attrs.as_slice() {
            *attr
        } else {
            return Err(syn::Error::new(
                variant.span(),
                format!(
                    "exactly one {} attribute must be present on each variant",
                    AT_ATTR_IDENT
                ),
            ));
        };

        let at_str = at_attr.parse_args::<LitStr>()?.value();
        let mut parsed_at = parse_at(at_attr.span(), &at_str)?;
        let munchers = parse_fields(&variant.fields, &mut parsed_at)?;

        res.push(ParsedVariant {
            parsed_at,
            munchers,
            pattern: fields_to_pattern(ident, &variant.ident, &variant.fields),
        });
    }

    Ok(res)
}

fn fields_to_pattern(ident: &Ident, variant_ident: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unit => quote! { #ident :: #variant_ident },
        Fields::Named(named) => {
            let bindings: TokenStream = named
                .named
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_ident = field.ident.as_ref().expect("Named fields to have a name");
                    let bind_ident = format_ident!("arg{}", index);
                    quote! {
                        #field_ident: #bind_ident,
                    }
                })
                .collect();
            quote! {
                #ident :: #variant_ident { #bindings }
            }
        }
        Fields::Unnamed(unnamed) => {
            let bindings: TokenStream = (0..unnamed.unnamed.len())
                .map(|index| {
                    let bind_ident = format_ident!("arg{}", index);
                    quote! {
                        #bind_ident,
                    }
                })
                .collect();
            quote! {
                #ident :: #variant_ident { #bindings }
            }
        }
    }
}

pub fn routable_derive_impl(input: Routable) -> TokenStream {
    let Routable { ident, variants } = input;

    let hidden = hidden_module();

    let variants_arr = variants.iter().map(|variant| {
        let regex = &variant.parsed_at.regex;
        let pattern = &variant.pattern;

        let munchers = variant.munchers.iter().map(|muncher| &muncher.muncher);
        let decode_args: TokenStream = variant
            .munchers
            .iter()
            .flat_map(|m| &m.binds)
            .map(|(index, bind_type)| {
                let name = format_ident!("arg{}", index);
                let decoder = bind_type.decoder();
                quote! {
                    let #name = #decoder?;
                }
            })
            .collect();

        quote! {
            #hidden::RouteMuncher {
                regex: #regex,
                munchers: &[#(&#munchers,)*],
                ctor: &|mut args| {
                    #decode_args
                    Some(#pattern)
                }
            }
        }
    });

    let match_arms = variants.iter().enumerate().map(|(variant_index, variant)| {
        let pattern = &variant.pattern;
        let encode_args: TokenStream = variant
            .munchers
            .iter()
            .flat_map(|m| &m.binds)
            .map(|(index, bind_type)| {
                let name = format_ident!("arg{}", index);
                bind_type.encoder(&name)
            })
            .collect();
        quote! {
            #pattern => {
                let mut args = #hidden::Args::empty();
                #encode_args
                (#variant_index, args)
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #hidden::DerivedRoutable for #ident {
            const VARIANTS: &'static [#hidden::RouteMuncher<Self>] = &[
                #(#variants_arr,)*
            ];
            fn to_args(&self) -> (usize, #hidden::Args) {
                match self {
                    #(#match_arms,)*
                }
            }
        }
    }
}
