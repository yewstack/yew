use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ReturnType, Signature, Type};

use crate::agent_fn::{AgentFn, AgentFnType, AgentName};

pub struct ReactorFn {}

impl AgentFnType for ReactorFn {
    type RecvType = (Type, Type);
    type OutputType = ();

    fn attr_name() -> &'static str {
        "reactor"
    }

    fn agent_type_name() -> &'static str {
        "reactor"
    }
    fn agent_type_name_plural() -> &'static str {
        "reactors"
    }

    fn parse_recv_type(sig: &Signature) -> syn::Result<Self::RecvType> {
        let mut inputs = sig.inputs.iter();
        let arg1 = inputs
            .next()
            .ok_or_else(|| syn::Error::new_spanned(&sig.ident, "expected 2 arguments"))?;
        let arg2 = inputs
            .next()
            .ok_or_else(|| syn::Error::new_spanned(&sig.ident, "expected 2 arguments"))?;

        let ty1 = Self::extract_fn_arg_type(arg1)?;
        let ty2 = Self::extract_fn_arg_type(arg2)?;

        Self::assert_no_left_argument(inputs, 2)?;

        Ok((ty1, ty2))
    }

    fn parse_output_type(sig: &Signature) -> syn::Result<Self::OutputType> {
        match &sig.output {
            ReturnType::Default => {}
            ReturnType::Type(_, ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "reactor functions cannot return any value",
                ))
            }
        }

        Ok(())
    }
}

pub fn reactor_impl(name: AgentName, mut agent_fn: AgentFn<ReactorFn>) -> syn::Result<TokenStream> {
    agent_fn.merge_agent_name(name)?;

    let struct_attrs = agent_fn.filter_attrs_for_agent_struct();
    let reactor_impl_attrs = agent_fn.filter_attrs_for_agent_impl();
    let phantom_generics = agent_fn.phantom_generics();
    let reactor_name = agent_fn.agent_name();
    let fn_name = agent_fn.inner_fn_ident();
    let inner_fn = agent_fn.print_inner_fn();

    let AgentFn {
        recv_type,
        generics,
        vis,
        ..
    } = agent_fn;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fn_generics = ty_generics.as_turbofish();
    let (tx_type, rx_type) = recv_type;

    let rx_ident = Ident::new("rx", Span::mixed_site());
    let tx_ident = Ident::new("tx", Span::mixed_site());

    let fn_call = quote! { #fn_name #fn_generics (#tx_ident, #rx_ident).await; };

    let quoted = quote! {
        #(#struct_attrs)*
        #[allow(unused_parens)]
        #vis struct #reactor_name #generics #where_clause {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        // we cannot disable any lints here because it will be applied to the function body
        // as well.
        #(#reactor_impl_attrs)*
        impl #impl_generics ::yew_agent::reactor::Reactor for #reactor_name #ty_generics #where_clause {
            type Sender = #tx_type;
            type Receiver = #rx_type;

            fn run(#tx_ident: Self::Sender, #rx_ident: Self::Receiver) -> ::yew_agent::__vendored::futures::future::LocalBoxFuture<'static, ()> {
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
