use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{FnArg, Ident, ReturnType, Signature, Type};

use crate::agent_fn::{AgentFn, AgentFnType, AgentName};

pub struct StationFn {}

impl AgentFnType for StationFn {
    type RecvType = Type;
    type OutputType = ();

    fn attr_name() -> &'static str {
        "station"
    }

    fn agent_type_name() -> &'static str {
        "station"
    }
    fn agent_type_name_plural() -> &'static str {
        "stations"
    }

    fn parse_recv_type(sig: &Signature) -> syn::Result<Self::RecvType> {
        let mut inputs = sig.inputs.iter();
        let arg = inputs
            .next()
            .cloned()
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

        Ok(*ty)
    }

    fn parse_output_type(sig: &Signature) -> syn::Result<Self::OutputType> {
        match &sig.output {
            ReturnType::Default => {}
            ReturnType::Type(_, ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "station functions cannot return any value",
                ))
            }
        }

        Ok(())
    }
}

pub fn station_impl(
    name: AgentName,
    mut station_fn: AgentFn<StationFn>,
) -> syn::Result<TokenStream> {
    station_fn.merge_agent_name(name)?;

    let struct_attrs = station_fn.filter_attrs_for_agent_struct();
    let station_impl_attrs = station_fn.filter_attrs_for_agent_impl();
    let phantom_generics = station_fn.phantom_generics();
    let station_name = station_fn.agent_name();
    let fn_name = station_fn.inner_fn_ident();
    let inner_fn = station_fn.print_inner_fn();

    let AgentFn {
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
