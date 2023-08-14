use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprClosure, ReturnType, Token, Type};

#[derive(Debug)]
pub struct PreparedState {
    closure: ExprClosure,
    return_type: Type,
    deps: Expr,
}

impl Parse for PreparedState {
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

impl PreparedState {
    // Async closure is not stable, so we rewrite it to closure + async block
    #[cfg(not(nightly_yew))]
    pub fn rewrite_to_closure_with_async_block(&self) -> ExprClosure {
        use proc_macro2::Span;
        use syn::parse_quote;

        let async_token = match &self.closure.asyncness {
            Some(m) => m,
            None => return self.closure.clone(),
        };

        // The async block always need to be move so input can be moved into it.
        let move_token = self
            .closure
            .capture
            .unwrap_or_else(|| Token![move](Span::call_site()));
        let body = &self.closure.body;

        let inner = parse_quote! {
            #async_token #move_token {
                #body
            }
        };

        let mut closure = self.closure.clone();

        closure.asyncness = None;
        // We omit the output type as it's an opaque future type.
        closure.output = ReturnType::Default;

        closure.body = inner;

        closure.attrs.push(parse_quote! { #[allow(unused_braces)] });

        closure
    }

    #[cfg(nightly_yew)]
    pub fn rewrite_to_closure_with_async_block(&self) -> ExprClosure {
        self.closure.clone()
    }

    pub fn to_token_stream_with_closure(&self) -> TokenStream {
        let deps = &self.deps;
        let rt = &self.return_type;
        let closure = self.rewrite_to_closure_with_async_block();

        match &self.closure.asyncness {
            Some(_) => quote! {
                ::yew::functional::use_prepared_state_with_suspension::<#rt, _, _, _>(#deps, #closure)
            },
            None => quote! {
                ::yew::functional::use_prepared_state::<#rt, _, _>(#deps, #closure)
            },
        }
    }

    // Expose a hook for the client side.
    //
    // The closure is stripped from the client side.
    pub fn to_token_stream_without_closure(&self) -> TokenStream {
        let deps = &self.deps;
        let rt = &self.return_type;

        quote! {
            ::yew::functional::use_prepared_state::<#rt, _>(#deps)
        }
    }
}
