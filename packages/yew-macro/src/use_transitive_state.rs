use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprClosure, ReturnType, Token, Type};

#[derive(Debug)]
pub struct TransitiveState {
    closure: ExprClosure,
    return_type: Type,
    deps: Expr,
}

impl Parse for TransitiveState {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Reads the deps.
        let deps = input.parse()?;

        input.parse::<Token![,]>().map_err(|e| {
            syn::Error::new(
                e.span(),
                "this hook takes 2 arguments but 1 argument was supplied",
            )
        })?;

        // Reads a closure.
        let expr: Expr = input.parse()?;

        let closure = match expr {
            Expr::Closure(m) => m,
            other => return Err(syn::Error::new_spanned(other, "expected closure")),
        };

        let return_type = match &closure.output {
            ReturnType::Default => {
                return Err(syn::Error::new_spanned(
                    &closure,
                    "You must specify a return type for this closure. This is used when the \
                     closure is omitted from the client side rendering bundle.",
                ))
            }
            ReturnType::Type(_rarrow, ty) => *ty.to_owned(),
        };

        if !input.is_empty() {
            let maybe_trailing_comma = input.lookahead1();

            if !maybe_trailing_comma.peek(Token![,]) {
                return Err(maybe_trailing_comma.error());
            }
        }

        Ok(Self {
            closure,
            return_type,
            deps,
        })
    }
}

impl TransitiveState {
    pub fn to_token_stream_with_closure(&self) -> TokenStream {
        let deps = &self.deps;
        let rt = &self.return_type;
        let closure = &self.closure;

        quote! {
            ::yew::functional::use_transitive_state::<#rt, _, _>(#deps, #closure)
        }
    }

    // Expose a hook for the client side.
    //
    // The closure is stripped from the client side.
    pub fn to_token_stream_without_closure(&self) -> TokenStream {
        let deps = &self.deps;
        let rt = &self.return_type;

        quote! {
            ::yew::functional::use_transitive_state::<#rt, _>(#deps)
        }
    }
}
