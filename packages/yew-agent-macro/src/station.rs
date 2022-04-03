use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, FnArg, Generics, Ident, Item, ItemFn, ReturnType, Type, Visibility};

#[derive(Clone)]
pub struct StationFn {
    recv_type: Box<Type>,
    generics: Generics,
    vis: Visibility,
    attrs: Vec<Attribute>,
    name: Ident,
    station_name: Option<Ident>,

    func: ItemFn,
}

impl Parse for StationFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        let func = match parsed {
            Item::Fn(m) => m,

            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    "`station` attribute can only be applied to functions",
                ))
            }
        };

        let ItemFn {
            attrs, vis, sig, ..
        } = func.clone();

        if sig.generics.lifetimes().next().is_some() {
            return Err(syn::Error::new_spanned(
                sig.generics,
                "stations can't have generic lifetime parameters",
            ));
        }

        if sig.asyncness.is_none() {
            return Err(syn::Error::new_spanned(
                sig.asyncness,
                "station functions must be async",
            ));
        }

        if sig.constness.is_some() {
            return Err(syn::Error::new_spanned(
                sig.constness,
                "const functions can't be stations",
            ));
        }

        if sig.abi.is_some() {
            return Err(syn::Error::new_spanned(
                sig.abi,
                "extern functions can't be stations",
            ));
        }

        match sig.output {
            ReturnType::Default => {}
            ReturnType::Type(_, ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "stations must not return anything.",
                ))
            }
        }

        let mut inputs = sig.inputs.into_iter();
        let arg = inputs
            .next()
            .unwrap_or_else(|| syn::parse_quote! { _: &() });

        let ty = match &arg {
            FnArg::Typed(arg) => arg.ty.clone(),

            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    arg,
                    "stations can't accept a receiver",
                ));
            }
        };

        // Checking after param parsing may make it a little inefficient
        // but that's a requirement for better error messages in case of receivers
        // `>0` because first one is already consumed.
        if inputs.len() > 0 {
            let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
            return Err(syn::Error::new_spanned(
                params,
                "stations can accept at most one argument",
            ));
        }

        Ok(Self {
            recv_type: ty,
            generics: sig.generics,
            vis,
            attrs,
            name: sig.ident,
            station_name: None,
            func,
        })
    }
}

impl StationFn {
    /// Filters attributes that should be copied to station definition.
    fn filter_attrs_for_station_struct(&self) -> Vec<Attribute> {
        self.attrs
            .iter()
            .filter_map(|m| {
                m.path
                    .get_ident()
                    .and_then(|ident| match ident.to_string().as_str() {
                        "doc" | "allow" => Some(m.clone()),
                        _ => None,
                    })
            })
            .collect()
    }

    /// Filters attributes that should be copied to the station impl block.
    fn filter_attrs_for_station_impl(&self) -> Vec<Attribute> {
        self.attrs
            .iter()
            .filter_map(|m| {
                m.path
                    .get_ident()
                    .and_then(|ident| match ident.to_string().as_str() {
                        "allow" => Some(m.clone()),
                        _ => None,
                    })
            })
            .collect()
    }

    fn phantom_generics(&self) -> Punctuated<Ident, Comma> {
        self.generics
            .type_params()
            .map(|ty_param| ty_param.ident.clone()) // create a new Punctuated sequence without any type bounds
            .collect::<Punctuated<_, Comma>>()
    }

    fn merge_station_name(&mut self, name: StationName) -> syn::Result<()> {
        if let Some(ref m) = name.station_name {
            if m == &self.name {
                return Err(syn::Error::new_spanned(
                    m,
                    "the station must not have the same name as the function",
                ));
            }
        }

        self.station_name = name.station_name;

        Ok(())
    }

    fn inner_fn_ident(&self) -> Ident {
        if self.station_name.is_some() {
            self.name.clone()
        } else {
            Ident::new("inner", Span::mixed_site())
        }
    }

    fn station_name(&self) -> Ident {
        self.station_name
            .clone()
            .unwrap_or_else(|| self.name.clone())
    }

    fn print_inner_fn(&self) -> ItemFn {
        let mut func = self.func.clone();
        func.sig.ident = self.inner_fn_ident();

        func.vis = Visibility::Inherited;

        func
    }
}

pub struct StationName {
    station_name: Option<Ident>,
}

impl Parse for StationName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { station_name: None });
        }

        let station_name = input.parse()?;

        Ok(Self {
            station_name: Some(station_name),
        })
    }
}

pub fn station_impl(name: StationName, mut station_fn: StationFn) -> syn::Result<TokenStream> {
    station_fn.merge_station_name(name)?;

    let struct_attrs = station_fn.filter_attrs_for_station_struct();
    let station_impl_attrs = station_fn.filter_attrs_for_station_impl();
    let phantom_generics = station_fn.phantom_generics();
    let station_name = station_fn.station_name();
    let fn_name = station_fn.inner_fn_ident();
    let inner_fn = station_fn.print_inner_fn();

    let StationFn {
        recv_type,
        generics,
        vis,
        ..
    } = station_fn;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fn_generics = ty_generics.as_turbofish();

    let rx_ident = Ident::new("rx", Span::mixed_site());

    let fn_call = quote! { #fn_name #fn_generics (#rx_ident).await; };

    let quoted = quote! {
        #(#struct_attrs)*
        #[allow(unused_parens)]
        #vis struct #station_name #generics #where_clause {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        // we cannot disable any lints here because it will be applied to the function body
        // as well.
        #(#station_impl_attrs)*
        impl #impl_generics ::yew_agent::station::Station for #station_name #ty_generics #where_clause {
            type Receiver = #recv_type;

            fn run(#rx_ident: Self::Receiver) -> ::yew_agent::__vendored::futures::future::LocalBoxFuture<'static, ()> {
                #inner_fn

                ::yew_agent::__vendored::futures::future::FutureExt::boxed_local(
                    async move {
                        #fn_call
                    }
                )
            }
        }
    };

    Ok(quoted)
}
