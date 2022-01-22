use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::visit_mut;
use syn::{Ident, ItemFn, LitStr, Signature};

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

    let output_type = &hook_sig.output_type;
    let hook_struct_name = Ident::new("HookProvider", Span::mixed_site());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let states = Ident::new("states", Span::mixed_site());

    let phantom_types = hook_sig.phantom_types();
    let phantom_lifetimes = hook_sig.phantom_lifetimes();

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
