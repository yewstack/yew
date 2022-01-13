use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Ident, ItemFn, LitStr, ReturnType, Signature, Stmt};

mod lifetime;

#[derive(Clone)]
pub struct HookFn {
    inner: ItemFn,
}

/// Creates lints on hooks in invalid position in an expression.
fn lint_expr(expr: Expr) -> TokenStream {
    todo!()
}

/// Rewrite valid hooks and lint invalid hooks in an expression.
///
/// returns (branched, TokenStream)
fn rewrite_expr(expr: Expr) -> (bool, TokenStream) {
    todo!()
}

/// Creates lints on hooks in invalid position in a statement.
fn lint_stmt(stmt: Stmt) -> TokenStream {
    todo!()
}

/// Rewrite valid hooks in a statement and lint invalid hooks.
///
/// returns (branched, TokenStream)
fn rewrite_stmt(stmt: Stmt) -> (bool, TokenStream) {
    todo!()
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

fn rewrite_return_type(rt_type: &ReturnType) -> TokenStream {
    match rt_type {
        ReturnType::Default => quote! { -> impl ::yew::functional::Hook<Ouput = ()> },
        ReturnType::Type(arrow, ref return_type) => {
            quote_spanned! { return_type.span() => #arrow impl ::yew::functional::Hook<Output = #return_type> }
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
            inner.sig.to_token_stream().to_string()
        ),
        Span::mixed_site(),
    );

    let ItemFn {
        vis,
        sig,
        block,
        attrs,
    } = inner;

    let Signature {
        fn_token,
        ident,
        generics,
        inputs,
        output: return_type,
        ..
    } = sig;

    let hook_return_type = rewrite_return_type(&return_type);
    let output_type = match &return_type {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ref m) => m.clone().into_token_stream(),
    };
    let stmts = block.stmts;

    let hook_struct_name = Ident::new("HookProvider", Span::mixed_site());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let states = Ident::new("states", Span::mixed_site());

    let output = quote! {
        #(#attrs)*
        #[doc = #doc_text]
        #vis #fn_token #ident #generics (#inputs) #hook_return_type {
            struct #hook_struct_name #generics {}

            impl #impl_generics ::yew::functional::Hook for #hook_struct_name #ty_generics #where_clause {
                type Output = #output_type;

                fn run(mut self, #states: &mut ::yew::functional::HookStates) -> Self::Output {
                    #(#stmts)*
                }
            }

            #hook_struct_name {}
        }
    };

    Ok(output)
}
