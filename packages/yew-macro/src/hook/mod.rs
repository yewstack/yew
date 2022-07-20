use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_file, parse_quote, visit_mut, Attribute, Ident, ItemFn, LitStr, ReturnType, Signature,
    Type,
};

mod body;
mod lifetime;
mod signature;

pub use body::BodyRewriter;
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
            emit_error!(sig.asyncness, "async functions can't be hooks");
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

impl HookFn {
    fn doc_attr(&self) -> Attribute {
        let vis = &self.inner.vis;
        let sig = &self.inner.sig;

        let sig_s = quote! { #vis #sig {
            __yew_macro_dummy_function_body__
        } }
        .to_string();

        let sig_file = parse_file(&sig_s).unwrap();
        let sig_formatted = prettyplease::unparse(&sig_file);

        let literal = LitStr::new(
            &format!(
                r#"
# Note

When used in function components and hooks, this hook is equivalent to:

```
{}
```
"#,
                sig_formatted.replace(
                    "__yew_macro_dummy_function_body__",
                    "/* implementation omitted */"
                )
            ),
            Span::mixed_site(),
        );

        parse_quote!(#[doc = #literal])
    }
}

pub fn hook_impl(hook: HookFn) -> syn::Result<TokenStream> {
    let doc_attr = hook.doc_attr();

    let HookFn { inner: original_fn } = hook;

    let ItemFn {
        ref vis,
        ref sig,
        ref block,
        ref attrs,
    } = original_fn;
    let mut block = *block.clone();

    let hook_sig = HookSignature::rewrite(sig);

    let Signature {
        ref fn_token,
        ref ident,
        ref inputs,
        output: ref hook_return_type,
        ref generics,
        ..
    } = hook_sig.sig;

    let output_type = &hook_sig.output_type;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let call_generics = hook_sig.call_generics();

    // We use _ctx so that if a hook does not use other hooks, it will not trigger unused_vars.
    let ctx_ident = Ident::new("_ctx", Span::mixed_site());

    let mut body_rewriter = BodyRewriter::new(ctx_ident.clone());
    visit_mut::visit_block_mut(&mut body_rewriter, &mut block);

    let inner_fn_ident = Ident::new("inner_fn", Span::mixed_site());
    let input_args = hook_sig.input_args();

    // there might be some overridden lifetimes in the return type.
    let inner_fn_rt = match &sig.output {
        ReturnType::Default => None,
        ReturnType::Type(rarrow, _) => Some(quote! { #rarrow #output_type }),
    };

    let inner_fn = quote! { fn #inner_fn_ident #generics (#ctx_ident: &mut ::yew::functional::HookContext, #inputs) #inner_fn_rt #where_clause #block };

    let inner_type_impl = if hook_sig.needs_boxing {
        let with_output = !matches!(hook_sig.output_type, Type::ImplTrait(_),);
        let inner_fn_rt = with_output.then(|| &inner_fn_rt);
        let output_type = with_output.then(|| &output_type);

        let hook_lifetime = &hook_sig.hook_lifetime;
        let hook_lifetime_plus = quote! { #hook_lifetime + };

        let boxed_inner_ident = Ident::new("boxed_inner", Span::mixed_site());
        let boxed_fn_type = quote! { ::std::boxed::Box<dyn #hook_lifetime_plus ::std::ops::FnOnce(&mut ::yew::functional::HookContext) #inner_fn_rt> };

        let as_boxed_fn = with_output.then(|| quote! { as #boxed_fn_type });

        // We need boxing implementation for `impl Trait` arguments.
        quote! {
            let #boxed_inner_ident = ::std::boxed::Box::new(
                    move |#ctx_ident: &mut ::yew::functional::HookContext| #inner_fn_rt {
                        #inner_fn_ident (#ctx_ident, #(#input_args,)*)
                    }
                ) #as_boxed_fn;

            ::yew::functional::BoxedHook::<#hook_lifetime, #output_type>::new(#boxed_inner_ident)
        }
    } else {
        let input_types = hook_sig.input_types();

        let args_ident = Ident::new("args", Span::mixed_site());
        let hook_struct_name = Ident::new("HookProvider", Span::mixed_site());

        let phantom_types = hook_sig.phantom_types();
        let phantom_lifetimes = hook_sig.phantom_lifetimes();

        quote! {
            struct #hook_struct_name #generics #where_clause {
                _marker: ::std::marker::PhantomData<( #(#phantom_types,)* #(#phantom_lifetimes,)* )>,
                #args_ident: (#(#input_types,)*),
            }

            #[automatically_derived]
            impl #impl_generics ::yew::functional::Hook for #hook_struct_name #ty_generics #where_clause {
                type Output = #output_type;

                fn run(mut self, #ctx_ident: &mut ::yew::functional::HookContext) -> Self::Output {
                    let (#(#input_args,)*) = self.#args_ident;

                    #inner_fn_ident #call_generics (#ctx_ident, #(#input_args,)*)
                }
            }

            #[automatically_derived]
            impl #impl_generics #hook_struct_name #ty_generics #where_clause {
                fn new(#inputs) -> Self {
                   #hook_struct_name {
                        _marker: ::std::marker::PhantomData,
                        #args_ident: (#(#input_args,)*),
                    }
                }
            }

            #hook_struct_name #call_generics ::new(#(#input_args,)*)
        }
    };

    // There're some weird issues with doc tests that it cannot detect return types properly.
    // So we print original implementation instead.
    let output = quote! {
        #[cfg(not(doctest))]
        #(#attrs)*
        #doc_attr
        #vis #fn_token #ident #generics (#inputs) #hook_return_type #where_clause {
            #inner_fn

            #inner_type_impl
        }

        #[cfg(doctest)]
        #original_fn
    };

    Ok(output)
}
