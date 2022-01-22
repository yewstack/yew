use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_quote, parse_quote_spanned, token, visit_mut, Ident, Lifetime, ReturnType, Signature,
    Type, TypeReference, WhereClause,
};

use super::lifetime;

pub struct HookSignature {
    pub hook_lifetime: Option<Lifetime>,
    pub sig: Signature,
    pub output_type: Type,
}

impl HookSignature {
    fn rewrite_return_type(
        hook_lifetime: Option<&Lifetime>,
        rt_type: &ReturnType,
    ) -> (ReturnType, Type) {
        let bound = hook_lifetime.map(|m| quote! { #m + });

        match rt_type {
            ReturnType::Default => (
                parse_quote! { -> impl #bound ::yew::functional::Hook<Ouput = ()> },
                parse_quote! { () },
            ),
            ReturnType::Type(arrow, ref return_type) => (
                parse_quote_spanned! {
                    return_type.span() => #arrow impl #bound ::yew::functional::Hook<Output = #return_type>
                },
                *return_type.clone(),
            ),
        }
    }

    /// Rewrites a Hook Signature and extracts information.
    pub fn rewrite(sig: &Signature) -> Self {
        let mut sig = sig.clone();

        let mut lifetimes = lifetime::CollectLifetimes::new("'arg", sig.ident.span());
        visit_mut::visit_signature_mut(&mut lifetimes, &mut sig);

        let Signature {
            ref mut generics,
            output: ref return_type,
            ..
        } = sig;

        let hook_lifetime = if !lifetimes.elided.is_empty() {
            let hook_lifetime = lifetime::find_available_lifetime(&lifetimes);
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

            generics.where_clause = Some(where_clause);

            Some(hook_lifetime)
        } else {
            None
        };

        let (output, output_type) = Self::rewrite_return_type(hook_lifetime.as_ref(), return_type);
        sig.output = output;

        Self {
            hook_lifetime,
            sig,
            output_type,
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
}
