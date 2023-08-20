use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ReturnType, Signature, Type};

use crate::agent_fn::{AgentFn, AgentFnType, AgentName};

pub struct ReactorFn {}

impl AgentFnType for ReactorFn {
    type OutputType = ();
    type RecvType = Type;

    fn attr_name() -> &'static str {
        "reactor"
    }

    fn agent_type_name() -> &'static str {
        "reactor"
    }

    fn parse_recv_type(sig: &Signature) -> syn::Result<Self::RecvType> {
        let mut inputs = sig.inputs.iter();
        let arg = inputs
            .next()
            .ok_or_else(|| syn::Error::new_spanned(&sig.ident, "expected 1 argument"))?;

        let ty = Self::extract_fn_arg_type(arg)?;

        Self::assert_no_left_argument(inputs, 1)?;

        Ok(ty)
    }

    fn parse_output_type(sig: &Signature) -> syn::Result<Self::OutputType> {
        match &sig.output {
            ReturnType::Default => {}
            ReturnType::Type(_, ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "reactor agents cannot return any value",
                ))
            }
        }

        Ok(())
    }
}

pub fn reactor_impl(name: AgentName, mut agent_fn: AgentFn<ReactorFn>) -> syn::Result<TokenStream> {
    agent_fn.merge_agent_name(name)?;

    if !agent_fn.is_async {
        return Err(syn::Error::new_spanned(
            &agent_fn.name,
            "reactor agents must be asynchronous",
        ));
    }

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

    let scope_ident = Ident::new("_scope", Span::mixed_site());

    let fn_call = quote! { #fn_name #fn_generics (#scope_ident).await };
    let crate_name = quote! { yew_agent };

    let quoted = quote! {
        #(#struct_attrs)*
        #[allow(unused_parens)]
        #vis struct #reactor_name #generics #where_clause {
            inner: ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ()>>>,
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        // we cannot disable any lints here because it will be applied to the function body
        // as well.
        #(#reactor_impl_attrs)*
        impl #impl_generics ::#crate_name::reactor::Reactor for #reactor_name #ty_generics #where_clause {
            type Scope = #recv_type;

            fn create(#scope_ident: Self::Scope) -> Self {
                #inner_fn

                Self {
                    inner: ::std::boxed::Box::pin(
                        async move {
                            #fn_call
                        }
                    ),
                    _marker: ::std::marker::PhantomData,
                }
            }
        }

        impl #impl_generics ::std::future::Future for #reactor_name #ty_generics #where_clause {
            type Output = ();

            fn poll(mut self: ::std::pin::Pin<&mut Self>, cx: &mut ::std::task::Context<'_>) -> ::std::task::Poll<Self::Output> {
                ::std::future::Future::poll(::std::pin::Pin::new(&mut self.inner), cx)
            }
        }

        impl #impl_generics ::#crate_name::Registrable for #reactor_name #ty_generics #where_clause {
            type Registrar = ::#crate_name::reactor::ReactorRegistrar<Self>;

            fn registrar() -> Self::Registrar {
                ::#crate_name::reactor::ReactorRegistrar::<Self>::new()
            }
        }

        impl #impl_generics ::#crate_name::Spawnable for #reactor_name #ty_generics #where_clause {
            type Spawner = ::#crate_name::reactor::ReactorSpawner<Self>;

            fn spawner() -> Self::Spawner {
                ::#crate_name::reactor::ReactorSpawner::<Self>::new()
            }
        }
    };

    Ok(quoted)
}
