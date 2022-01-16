use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token;
use syn::visit_mut;
use syn::{parse_quote, Ident, ItemFn, Lifetime, LitStr, ReturnType, Signature, WhereClause};

mod body;
mod lifetime;

#[derive(Clone)]
pub struct HookFn {
    inner: ItemFn,
}

impl Parse for HookFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func: ItemFn = input.parse()?;

        let sig = func.sig.clone();

        if sig.asyncness.is_some() {
            emit_error!(sig.asyncness, "hooks can't be async functions");
        }

        if sig.constness.is_some() {
            emit_error!(sig.constness, "const functions can't be hooks");
        }

        if sig.abi.is_some() {
            emit_error!(sig.abi, "extern functions can't be hooks");
        }

        if sig.unsafety.is_some() {
            emit_error!(sig.unsafety, "unsafe functions can't be hooks");
        }

        if !sig.ident.to_string().starts_with("use_") {
            emit_error!(sig.ident, "hooks must have a name starting with `use_`");
        }

        Ok(Self { inner: func })
    }
}

fn rewrite_return_type(hook_lifetime: Option<&Lifetime>, rt_type: &ReturnType) -> TokenStream {
    let bound = hook_lifetime.map(|m| quote! { #m + });

    match rt_type {
        ReturnType::Default => {
            quote! { -> impl #bound ::yew::functional::Hook<Ouput = ()> }
        }
        ReturnType::Type(arrow, ref return_type) => {
            quote_spanned! { return_type.span() => #arrow impl #bound ::yew::functional::Hook<Output = #return_type> }
        }
    }
}

pub fn hook_impl(component: HookFn) -> syn::Result<TokenStream> {
    let HookFn { inner } = component;

    let doc_text = LitStr::new(
        &format!(
            r#"
# Note

When used in function components and hooks, this hook is equivalent to:

```
{}
```
"#,
            inner.sig.to_token_stream()
        ),
        Span::mixed_site(),
    );

    let ItemFn {
        vis,
        mut sig,
        mut block,
        attrs,
    } = inner;

    let Signature {
        fn_token,
        ident,
        mut generics,
        inputs,
        output: return_type,
        ..
    } = sig.clone();

    let mut lifetimes = lifetime::CollectLifetimes::new("'arg", ident.span());
    visit_mut::visit_signature_mut(&mut lifetimes, &mut sig);

    let hook_lifetime = if !lifetimes.elided.is_empty() {
        let hook_lifetime = lifetime::find_available_lifetime(&lifetimes);
        generics.params = {
            let elided_lifetimes = &lifetimes.elided;
            let params = generics.params;

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

    let hook_return_type = rewrite_return_type(hook_lifetime.as_ref(), &return_type);
    let output_type = match &return_type {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ref m) => m.clone().into_token_stream(),
    };

    let hook_struct_name = Ident::new("HookProvider", Span::mixed_site());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let states = Ident::new("states", Span::mixed_site());

    let phantom_types = generics
        .type_params()
        .map(|ty_param| ty_param.ident.clone())
        .collect::<Vec<_>>();

    let phantom_lifetimes = generics
        .lifetimes()
        .map(|life| quote! { &#life () })
        .collect::<Vec<_>>();

    let mut body_rewriter = body::BodyRewriter::default();
    visit_mut::visit_block_mut(&mut body_rewriter, &mut *block);

    let output = quote! {
        #(#attrs)*
        #[doc = #doc_text]
        #vis #fn_token #ident #generics (#inputs) #hook_return_type {
            struct #hook_struct_name #generics {
                _marker: ::std::marker::PhantomData<( #(#phantom_types,)* #(#phantom_lifetimes,)* )>,
            }

            impl #impl_generics ::yew::functional::Hook for #hook_struct_name #ty_generics #where_clause {
                type Output = #output_type;

                fn run(mut self, #states: &mut ::yew::functional::HookStates) -> Self::Output #block
            }

            #hook_struct_name {
                _marker: ::std::marker::PhantomData,
            }
        }
    };

    Ok(output)
}
