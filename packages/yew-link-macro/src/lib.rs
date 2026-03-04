use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, Pat, PatType};

/// Derive a [`LinkedState`] implementation from an impl block that declares
/// `type Context`, `type Input`, and `async fn resolve`.
///
/// On all targets the macro emits `impl LinkedState for T { type Input = …; type Error = …; }`.
///
/// On the server (`not(target_arch = "wasm32")`) it additionally emits
/// `impl LinkedStateResolve for T { … }` with the user-provided resolve body.
/// This half is stripped from WASM bundles automatically.
///
/// ## `type Error` (optional)
///
/// If `type Error` is omitted, it defaults to [`std::convert::Infallible`] and
/// the resolve body is wrapped in `Ok(…)` automatically. When `type Error` is
/// present, the resolve body must return `Result<Self, Self::Error>`.
///
/// # Example
///
/// ```ignore
/// #[linked_state]
/// impl LinkedState for Post {
///     type Context = DbPool;
///     type Input = u32;
///
///     async fn resolve(ctx: &DbPool, id: &u32) -> Self {
///         ctx.get_post(*id).await
///     }
/// }
///
/// // With a typed error:
/// #[linked_state]
/// impl LinkedState for Post {
///     type Context = DbPool;
///     type Input = u32;
///     type Error = ApiError;
///
///     async fn resolve(ctx: &DbPool, id: &u32) -> Result<Self, ApiError> {
///         ctx.get_post(*id).await.map_err(ApiError::from)
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn linked_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);
    match expand(impl_block) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(impl_block: ItemImpl) -> syn::Result<proc_macro2::TokenStream> {
    let self_ty = &impl_block.self_ty;
    let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();

    let mut input_ty = None;
    let mut context_ty = None;
    let mut error_ty: Option<&syn::Type> = None;
    let mut resolve_fn: Option<&ImplItemFn> = None;

    for item in &impl_block.items {
        match item {
            ImplItem::Type(t) if t.ident == "Input" => input_ty = Some(&t.ty),
            ImplItem::Type(t) if t.ident == "Context" => context_ty = Some(&t.ty),
            ImplItem::Type(t) if t.ident == "Error" => error_ty = Some(&t.ty),
            ImplItem::Fn(f) if f.sig.ident == "resolve" => resolve_fn = Some(f),
            _ => {}
        }
    }

    let input_ty =
        input_ty.ok_or_else(|| syn::Error::new(Span::call_site(), "missing `type Input`"))?;
    let context_ty =
        context_ty.ok_or_else(|| syn::Error::new(Span::call_site(), "missing `type Context`"))?;
    let resolve_fn = resolve_fn
        .ok_or_else(|| syn::Error::new(Span::call_site(), "missing `async fn resolve`"))?;

    if resolve_fn.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            resolve_fn.sig.fn_token,
            "`resolve` must be an async fn",
        ));
    }

    let params: Vec<_> = resolve_fn.sig.inputs.iter().collect();
    if params.len() != 2 {
        return Err(syn::Error::new_spanned(
            &resolve_fn.sig.inputs,
            "`resolve` must take exactly two parameters: context and input references",
        ));
    }

    let ctx_name = param_ident(params[0])?;
    let input_name = param_ident(params[1])?;

    let resolve_stmts = &resolve_fn.block.stmts;

    let (error_ty_tokens, resolve_body) = match error_ty {
        Some(ty) => (quote! { #ty }, quote! { #(#resolve_stmts)* }),
        None => (
            quote! { ::yew_link::Never },
            quote! { ::core::result::Result::Ok({ #(#resolve_stmts)* }) },
        ),
    };

    Ok(quote! {
        impl #impl_generics ::yew_link::LinkedState for #self_ty #ty_generics #where_clause {
            type Input = #input_ty;
            type Error = #error_ty_tokens;
        }

        #[cfg(not(target_arch = "wasm32"))]
        impl #impl_generics ::yew_link::LinkedStateResolve for #self_ty #ty_generics #where_clause {
            type Context = #context_ty;

            async fn resolve<'__yew_link>(
                #ctx_name: &'__yew_link Self::Context,
                #input_name: &'__yew_link <Self as ::yew_link::LinkedState>::Input,
            ) -> ::core::result::Result<Self, <Self as ::yew_link::LinkedState>::Error> {
                #resolve_body
            }
        }
    })
}

fn param_ident(arg: &FnArg) -> syn::Result<&Ident> {
    match arg {
        FnArg::Typed(PatType { pat, .. }) => match pat.as_ref() {
            Pat::Ident(pi) => Ok(&pi.ident),
            _ => Err(syn::Error::new_spanned(pat, "expected an identifier")),
        },
        other => Err(syn::Error::new_spanned(
            other,
            "unexpected `self` parameter",
        )),
    }
}
