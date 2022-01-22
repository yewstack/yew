use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::visit_mut;
use syn::{Ident, ItemFn, LitStr, ReturnType, Signature};

mod body;
mod lifetime;
mod signature;

use signature::HookSignature;

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
        sig,
        mut block,
        attrs,
    } = inner;

    let hook_sig = HookSignature::rewrite(&sig);

    let Signature {
        ref fn_token,
        ref ident,
        ref inputs,
        output: ref hook_return_type,
        ref generics,
        ..
    } = hook_sig.sig;

    let hook_lifetime = hook_sig.hook_lifetime.as_ref();
    let output_type = &hook_sig.output_type;
    let hook_struct_name = Ident::new("HookProvider", Span::mixed_site());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let call_generics = ty_generics.as_turbofish();

    let states = Ident::new("states", Span::mixed_site());

    let phantom_types = hook_sig.phantom_types();
    let phantom_lifetimes = hook_sig.phantom_lifetimes();

    let mut body_rewriter = body::BodyRewriter::default();
    visit_mut::visit_block_mut(&mut body_rewriter, &mut *block);

    let hook_lifetime_plus = hook_lifetime.map(|m| quote! { #m + });
    let inner_ident = Ident::new("inner", Span::mixed_site());

    // let inner_fn_ident = Ident::new("inner_fn", Span::mixed_site());
    // let input_args = hook_sig.input_args();
    let boxed_fn_type = {
        let rt = match &sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, _) => Some(quote! { -> #output_type }),
        };

        quote! { ::std::boxed::Box<dyn #hook_lifetime_plus FnOnce(&mut ::yew::functional::HookStates) #rt> }
    };

    let output = quote! {
        #[doc = #doc_text]
        #(#attrs)*
        #vis #fn_token #ident #generics (#inputs) #hook_return_type #where_clause {
            // fn #inner_fn_ident #generics (#states: &mut ::yew::functional::HookStates, #inputs) -> #output_type #block

            // always capture inputs with closure for now, we need boxing implementation for `impl Trait`
            // arguments anyways.
            // let inner = ::std::boxed::Box::new(move |#states: &mut ::yew::functional::HookStates| #inner_fn_ident #call_generics (#states, #(#input_args)*) )
            //     as ::std::boxed::Box<#hook_lifetime_plus FnOnce(&mut ::yew::functional::HookStates) -> #output_type>;

            let #inner_ident = ::std::boxed::Box::new(move |#states: &mut ::yew::functional::HookStates| #block )
                as #boxed_fn_type;

            struct #hook_struct_name #generics #where_clause {
                _marker: ::std::marker::PhantomData<( #(#phantom_types,)* #(#phantom_lifetimes,)* )>,
                #inner_ident: #boxed_fn_type,
            }

            impl #impl_generics ::yew::functional::Hook for #hook_struct_name #ty_generics #where_clause {
                type Output = #output_type;

                fn run(mut self, #states: &mut ::yew::functional::HookStates) -> Self::Output {
                    (self.inner)(#states)
                }
            }

            impl #impl_generics #hook_struct_name #ty_generics #where_clause {
                fn new(#inner_ident: #boxed_fn_type) -> Self {
                   #hook_struct_name {
                        _marker: ::std::marker::PhantomData,
                        #inner_ident,
                    }
                }
            }

            #hook_struct_name #call_generics ::new(#inner_ident)
        }
    };

    Ok(output)
}
