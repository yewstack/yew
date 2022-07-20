use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote, parse_quote_spanned, token, visit_mut, FnArg, GenericParam, Ident, Lifetime, Pat,
    Receiver, ReturnType, Signature, Type, TypeImplTrait, TypeReference, WhereClause,
};

use super::lifetime;

#[derive(Default)]
pub struct CollectArgs {
    needs_boxing: bool,
}

impl CollectArgs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VisitMut for CollectArgs {
    fn visit_type_impl_trait_mut(&mut self, impl_trait: &mut TypeImplTrait) {
        self.needs_boxing = true;

        visit_mut::visit_type_impl_trait_mut(self, impl_trait);
    }

    fn visit_receiver_mut(&mut self, recv: &mut Receiver) {
        emit_error!(recv, "methods cannot be hooks");

        visit_mut::visit_receiver_mut(self, recv);
    }
}

pub struct HookSignature {
    pub hook_lifetime: Lifetime,
    pub sig: Signature,
    pub output_type: Type,
    pub needs_boxing: bool,
}

impl HookSignature {
    fn rewrite_return_type(hook_lifetime: &Lifetime, rt_type: &ReturnType) -> (ReturnType, Type) {
        let bound = quote! { #hook_lifetime + };

        match rt_type {
            ReturnType::Default => (
                parse_quote! { -> impl #bound ::yew::functional::Hook<Output = ()> },
                parse_quote! { () },
            ),
            ReturnType::Type(arrow, ref return_type) => {
                if let Type::Reference(ref m) = &**return_type {
                    if m.lifetime.is_none() {
                        let mut return_type_ref = m.clone();
                        return_type_ref.lifetime = parse_quote!('hook);

                        let return_type_ref = Type::Reference(return_type_ref);

                        return (
                            parse_quote_spanned! {
                                return_type.span() => #arrow impl #bound ::yew::functional::Hook<Output = #return_type_ref>
                            },
                            return_type_ref,
                        );
                    }
                }

                (
                    parse_quote_spanned! {
                        return_type.span() => #arrow impl #bound ::yew::functional::Hook<Output = #return_type>
                    },
                    *return_type.clone(),
                )
            }
        }
    }

    /// Rewrites a Hook Signature and extracts information.
    pub fn rewrite(sig: &Signature) -> Self {
        let mut sig = sig.clone();

        let mut arg_info = CollectArgs::new();
        arg_info.visit_signature_mut(&mut sig);

        let mut lifetimes = lifetime::CollectLifetimes::new("'arg", sig.ident.span());
        for arg in sig.inputs.iter_mut() {
            match arg {
                FnArg::Receiver(arg) => lifetimes.visit_receiver_mut(arg),
                FnArg::Typed(arg) => lifetimes.visit_type_mut(&mut arg.ty),
            }
        }

        let Signature {
            ref mut generics,
            output: ref return_type,
            ..
        } = sig;

        let hook_lifetime = {
            let hook_lifetime = Lifetime::new("'hook", Span::mixed_site());
            generics.params = {
                let elided_lifetimes = &lifetimes.elided;
                let params = &generics.params;

                parse_quote!(#hook_lifetime, #(#elided_lifetimes,)* #params)
            };

            let mut where_clause = generics
                .where_clause
                .clone()
                .unwrap_or_else(|| WhereClause {
                    where_token: token::Where {
                        span: Span::mixed_site(),
                    },
                    predicates: Default::default(),
                });

            for elided in lifetimes.elided.iter() {
                where_clause
                    .predicates
                    .push(parse_quote!(#elided: #hook_lifetime));
            }

            for explicit in lifetimes.explicit.iter() {
                where_clause
                    .predicates
                    .push(parse_quote!(#explicit: #hook_lifetime));
            }

            for type_param in generics.type_params() {
                let type_param_ident = &type_param.ident;
                where_clause
                    .predicates
                    .push(parse_quote!(#type_param_ident: #hook_lifetime));
            }

            generics.where_clause = Some(where_clause);

            hook_lifetime
        };

        let (output, output_type) = Self::rewrite_return_type(&hook_lifetime, return_type);
        sig.output = output;

        Self {
            hook_lifetime,
            sig,
            output_type,
            needs_boxing: arg_info.needs_boxing,
        }
    }

    pub fn phantom_types(&self) -> Vec<Ident> {
        self.sig
            .generics
            .type_params()
            .map(|ty_param| ty_param.ident.clone())
            .collect()
    }

    pub fn phantom_lifetimes(&self) -> Vec<TypeReference> {
        self.sig
            .generics
            .lifetimes()
            .map(|life| parse_quote! { &#life () })
            .collect()
    }

    pub fn input_args(&self) -> Vec<Ident> {
        self.sig
            .inputs
            .iter()
            .filter_map(|m| {
                if let FnArg::Typed(m) = m {
                    if let Pat::Ident(ref m) = *m.pat {
                        return Some(m.ident.clone());
                    }
                }

                None
            })
            .collect()
    }

    pub fn input_types(&self) -> Vec<Type> {
        self.sig
            .inputs
            .iter()
            .filter_map(|m| {
                if let FnArg::Typed(m) = m {
                    return Some(*m.ty.clone());
                }

                None
            })
            .collect()
    }

    pub fn call_generics(&self) -> TokenStream {
        let mut generics = self.sig.generics.clone();

        // We need to filter out lifetimes.
        generics.params = generics
            .params
            .into_iter()
            .filter(|m| !matches!(m, GenericParam::Lifetime(_)))
            .collect();

        let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
        ty_generics.as_turbofish().to_token_stream()
    }
}
