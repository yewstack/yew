use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Ident, ReturnType, Signature, Type};

use crate::agent_fn::{AgentFn, AgentFnType, AgentName};

pub struct TaskFn {}

impl AgentFnType for TaskFn {
    type OutputType = Type;
    type RecvType = Type;

    fn attr_name() -> &'static str {
        "task"
    }

    fn agent_type_name() -> &'static str {
        "task"
    }

    fn agent_type_name_plural() -> &'static str {
        "tasks"
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

pub fn task_impl(name: AgentName, mut agent_fn: AgentFn<TaskFn>) -> syn::Result<TokenStream> {
    agent_fn.merge_agent_name(name)?;

    let struct_attrs = agent_fn.filter_attrs_for_agent_struct();
    let task_impl_attrs = agent_fn.filter_attrs_for_agent_impl();
    let phantom_generics = agent_fn.phantom_generics();
    let task_name = agent_fn.agent_name();
    let fn_name = agent_fn.inner_fn_ident();
    let inner_fn = agent_fn.print_inner_fn();

    let AgentFn {
        recv_type: input_type,
        generics,
        output_type,
        vis,
        ..
    } = agent_fn;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fn_generics = ty_generics.as_turbofish();

    let in_ident = Ident::new("input", Span::mixed_site());

    let fn_call = quote! { #fn_name #fn_generics (#in_ident).await };

    let quoted = quote! {
        #(#struct_attrs)*
        #[allow(unused_parens)]
        #vis struct #task_name #generics #where_clause {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        // we cannot disable any lints here because it will be applied to the function body
        // as well.
        #(#task_impl_attrs)*
        impl #impl_generics ::yew_agent::task::Task for #task_name #ty_generics #where_clause {
            type Input = #input_type;
            type Output = #output_type;

            fn run(#in_ident: Self::Input) -> ::yew_agent::__vendored::futures::future::LocalBoxFuture<'static, Self::Output> {
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
