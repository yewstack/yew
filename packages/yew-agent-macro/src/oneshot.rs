use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Ident, ReturnType, Signature, Type};

use crate::agent_fn::{AgentFn, AgentFnType, AgentName};

pub struct OneshotFn {}

impl AgentFnType for OneshotFn {
    type OutputType = Type;
    type RecvType = Type;

    fn attr_name() -> &'static str {
        "oneshot"
    }

    fn agent_type_name() -> &'static str {
        "oneshot"
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
        let ty = match &sig.output {
            ReturnType::Default => {
                parse_quote! { () }
            }
            ReturnType::Type(_, ty) => *ty.clone(),
        };

        Ok(ty)
    }
}

pub fn oneshot_impl(name: AgentName, mut agent_fn: AgentFn<OneshotFn>) -> syn::Result<TokenStream> {
    agent_fn.merge_agent_name(name)?;

    let struct_attrs = agent_fn.filter_attrs_for_agent_struct();
    let oneshot_impl_attrs = agent_fn.filter_attrs_for_agent_impl();
    let phantom_generics = agent_fn.phantom_generics();
    let oneshot_name = agent_fn.agent_name();
    let fn_name = agent_fn.inner_fn_ident();
    let inner_fn = agent_fn.print_inner_fn();

    let AgentFn {
        recv_type: input_type,
        generics,
        output_type,
        vis,
        is_async,
        ..
    } = agent_fn;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fn_generics = ty_generics.as_turbofish();

    let in_ident = Ident::new("_input", Span::mixed_site());

    let fn_call = if is_async {
        quote! { #fn_name #fn_generics (#in_ident).await }
    } else {
        quote! { #fn_name #fn_generics (#in_ident) }
    };
    let crate_name = quote! { yew_agent };

    let quoted = quote! {
        #(#struct_attrs)*
        #[allow(unused_parens)]
        #vis struct #oneshot_name #generics #where_clause {
            inner: ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = #output_type>>>,
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        // we cannot disable any lints here because it will be applied to the function body
        // as well.
        #(#oneshot_impl_attrs)*
        impl #impl_generics ::#crate_name::oneshot::Oneshot for #oneshot_name #ty_generics #where_clause {
            type Input = #input_type;

            fn create(#in_ident: Self::Input) -> Self {
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

        impl #impl_generics ::std::future::Future for #oneshot_name #ty_generics #where_clause {
            type Output = #output_type;

            fn poll(mut self: ::std::pin::Pin<&mut Self>, cx: &mut ::std::task::Context<'_>) -> ::std::task::Poll<Self::Output> {
                ::std::future::Future::poll(::std::pin::Pin::new(&mut self.inner), cx)
            }
        }

        impl #impl_generics ::#crate_name::Registrable for #oneshot_name #ty_generics #where_clause {
            type Registrar = ::#crate_name::oneshot::OneshotRegistrar<Self>;

            fn registrar() -> Self::Registrar {
                ::#crate_name::oneshot::OneshotRegistrar::<Self>::new()
            }
        }

        impl #impl_generics ::#crate_name::Spawnable for #oneshot_name #ty_generics #where_clause {
            type Spawner = ::#crate_name::oneshot::OneshotSpawner<Self>;

            fn spawner() -> Self::Spawner {
                ::#crate_name::oneshot::OneshotSpawner::<Self>::new()
            }
        }
    };

    Ok(quoted)
}
