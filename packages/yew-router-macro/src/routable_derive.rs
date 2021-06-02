use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, LitStr};

const AT_ATTR_PATH: &str = "at";
const BIND_ATTR_PATH: &str = "bind";

pub fn routable_derive_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput { ident, data, .. } = input;
    let data = match data {
        Data::Enum(e) => e,
        Data::Struct(s) => {
            return Err(syn::Error::new(
                s.struct_token.span(),
                "expected enum, found struct",
            ))
        }
        Data::Union(u) => {
            return Err(syn::Error::new(
                u.union_token.span(),
                "expected enum, found union",
            ))
        }
    };

    let mut variants = Vec::new();
    for variant in data.variants {
        let mut binds = HashMap::new();

        let at = {
            let mut at = None;
            for attr in variant.attrs.iter() {
                if attr.path.is_ident(AT_ATTR_PATH) {
                    at = Some(attr.parse_args::<LitStr>()?);
                }
            }

            at.ok_or_else(|| {
                let attrs = &variant.attrs;
                syn::Error::new_spanned(
                    quote! { #(#attrs)* },
                    &format!("{} attribute not found", AT_ATTR_PATH),
                )
            })?
        };

        match &variant.fields {
            Fields::Unit => {}
            Fields::Named(fields) => {
                for field in &fields.named {
                    let attrs = field
                        .attrs
                        .iter()
                        .filter(|attr| attr.path.is_ident(BIND_ATTR_PATH))
                        .collect::<Vec<_>>();

                    if attrs.len() > 1 {
                        return Err(syn::Error::new_spanned(
                            quote! { #(#attrs)* },
                            "a field can have only one `#[bind(...)]`"
                        ))
                    }

                    for attr in attrs {
                        // todo maybe don't clone?
                        let ident = field.ident.clone().unwrap(); // named fields have idents
                        let attr = attr.parse_args::<Ident>()?;
                        binds.insert(ident, attr);
                    }
                }
            }
            Fields::Unnamed(fields) => {
                return Err(syn::Error::new_spanned(
                    quote! { #fields },
                    "only named fields are allowed",
                ))
            }
        }

        // special casing because it's the only field level bind
        let not_found = {
            let not_founds = variant
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident(BIND_ATTR_PATH))
                .filter(|attr| {
                    attr.parse_args::<Ident>()
                        .map(|it| it == "not_found")
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();

            let len = not_founds.len();

            if len == 0 {
                false
            } else if len == 1 {
                true
            } else {
                return Err(syn::Error::new_spanned(
                    quote! { #(#not_founds)* },
                    "only one `#[bind(not_found)]` can be present",
                ));
            }
        };

        variants.push(RoutableVariant {
            at,
            binds,
            not_found,
            ident: variant.ident,
            fields: variant.fields,
        });
    }

    let cache_thread_local_ident = Ident::new(
        &format!("__{}_ROUTER_CURRENT_ROUTE_CACHE", ident),
        ident.span(),
    );

    let routes = variants.iter().map(|it| &it.at);
    let not_found_route = variants
        .iter()
        .find(|it| it.not_found)
        .map(|it| {
            let ident = &it.ident;
            quote! {
                ::std::option::Option::Some(Self::#ident)
            }
        })
        .unwrap_or_else(|| {
            quote! {
                ::std::option::Option::None
            }
        });

    let to_paths = variants
        .iter()
        .map(|it| it.build_to_path())
        .collect::<Vec<_>>();
    let from_path = variants
        .iter()
        .map(|it| it.build_from_path())
        .collect::<Vec<_>>();

    let bindings = {
        let items = variants
            .iter()
            .map(|it| {
                let at = it.at.value();
                let bindings = it.build_bindings();
                quote! { map.insert(#at, #bindings); }
            })
            .collect::<Vec<_>>();

        let len = items.len();

        quote! {
            {
                let mut map = ::std::collections::HashMap::with_capacity(#len);
                #(#items)*
                map
            }
        }
    };

    let output = quote! {
        ::std::thread_local! {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #cache_thread_local_ident: ::std::cell::RefCell<::std::option::Option<#ident>> = ::std::cell::RefCell::new(::std::option::Option::None);
        }


        impl ::yew_router::Routable for #ident {
            fn from_path(
                path: &str,
                params: &::std::collections::HashMap<&str, &str>,
                queries: ::std::collections::HashMap<::std::string::String, ::std::string::String>
            ) -> ::std::option::Option<Self> {
                match path {
                    #(#from_path),*,
                    _ => ::std::option::Option::None,
                }
            }

            fn to_path(&self) -> ::std::string::String {
                match self {
                    #(#to_paths),*
                }
            }

            fn routes() -> ::std::vec::Vec<&'static str> {
                ::std::vec![#(#routes),*]
            }

            fn not_found_route() -> ::std::option::Option<Self> {
                #not_found_route
            }


            fn current_route() -> ::std::option::Option<Self> {
                #cache_thread_local_ident.with(|val| ::std::clone::Clone::clone(&*val.borrow()))
            }

            fn recognize(location: ::yew_router::__macro::Location) -> ::std::option::Option<Self> {
                ::std::thread_local! {
                    static ROUTER: ::yew_router::__macro::Router = ::yew_router::__macro::build_router::<#ident>();
                }

                let bindings = #bindings;

                let route = ROUTER.with(|router| ::yew_router::__macro::recognize_with_router(router, location, bindings));
                {
                    let route = ::std::clone::Clone::clone(&route);
                    #cache_thread_local_ident.with(move |val| {
                        *val.borrow_mut() = route;
                    });
                }
                route
            }

            fn cleanup() {
                #cache_thread_local_ident.with(move |val| {
                    *val.borrow_mut() = ::std::option::Option::None;
                });
            }
        }
    };
    Ok(output)
}

#[derive(Debug)]
struct RoutableVariant {
    /// The path literal
    ///
    /// `#[at("/")]`
    at: LitStr,
    /// The `#[bind(Ident)]` found on the named fields values.
    /// Map of [`Ident`] of field to [`Ident`] of the binding type (query, etc)
    binds: HashMap<Ident, Ident>,
    /// Is this 404 redirect route
    not_found: bool,
    /// Variant's ident
    ident: Ident,
    fields: Fields,
}

impl RoutableVariant {
    fn build_to_path(&self) -> TokenStream {
        let mut at = self.at.value();
        let ident = &self.ident;
        match &self.fields {
            Fields::Unit => quote! { Self::#ident => ::std::string::ToString::to_string(#at) },
            Fields::Named(field) => {
                let fields = field
                    .named
                    .iter()
                    .map(|it| it.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                for field in fields.iter() {
                    // :param -> {param}
                    // so we can pass it to `format!("...", param)`
                    at = at.replace(&format!(":{}", field), &format!("{{{}}}", field))
                }

                let query_params = self
                    .binds
                    .iter()
                    .filter(|(_, attr)| *attr == "query")
                    .collect::<Vec<_>>();
                if !query_params.is_empty() {
                    at.push('?')
                }

                for (ident, _) in query_params {
                    at.push_str(&format!("{}={{{}}}", ident, ident))
                }

                quote! {
                    Self::#ident { #(#fields),* } => ::std::format!(#at, #(#fields = #fields),*)
                }
            }
            Fields::Unnamed(_) => unreachable!(), // already checked
        }
    }

    fn build_from_path(&self) -> TokenStream {
        let Self {
            at,
            binds,
            ident,
            fields,
            ..
        } = &self;
        let binds = binds
            .iter()
            .map(|(ident, attr)| (ident.to_string(), attr.to_string()))
            .collect::<HashMap<_, _>>();
        let right = match fields {
            Fields::Unit => quote! { Self::#ident },
            Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|it| it.ident.as_ref().expect("named fields have idents"))
                    .map(|field| {
                        let map_ident = if binds.contains_key(&field.to_string()) {
                            quote! { queries }
                        } else {
                            quote! { params }
                        };
                        quote! { #field: #map_ident.get(stringify!(#field)).map(|it| it.parse().ok()).flatten()? }
                    });

                quote! { Self::#ident { #(#fields),* } }
            }
            Fields::Unnamed(_) => unreachable!(),
        };
        quote! {
            #at => ::std::option::Option::Some(#right)
        }
    }

    fn build_bindings(&self) -> TokenStream {
        let mut binds = vec![];
        for (ident, attr) in self.binds.iter() {
            if attr == "query" {
                let bind = quote_spanned!(attr.span()=> Query);
                let ident = quote_spanned!(ident.span()=> stringify!(#ident));
                binds.push(quote! {
                    map.insert(#ident, ::yew_router::__macro::Binding::#bind);
                });
            }
        }

        quote! {
            {
                let mut map = ::std::collections::HashMap::with_capacity(1);
                #(#binds)*
                map
            }
        }
    }
}
