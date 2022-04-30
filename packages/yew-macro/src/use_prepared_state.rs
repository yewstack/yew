use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Expr, ExprClosure, ReturnType, Token, Type};

#[derive(Debug)]
pub struct PreparedState<const WITH_ASYNC_CLOSURE: bool> {
    closure: ExprClosure,
    return_type: Type,
    deps: Expr,
}

impl<const WITH_ASYNC_CLOSURE: bool> Parse for PreparedState<WITH_ASYNC_CLOSURE> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Reads a closure.
        let closure: ExprClosure = input.parse()?;

        input.parse::<Token![,]>()?;

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

        // Reads the deps.
        let deps = input.parse()?;

        if !input.is_empty() {
            let maybe_trailing_comma = input.lookahead1();

            if !maybe_trailing_comma.peek(Token![,]) {
                return Err(maybe_trailing_comma.error());
            }
        }

        if !WITH_ASYNC_CLOSURE {
            if let Some(m) = closure.asyncness.as_ref() {
                return Err(syn::Error::new_spanned(
                    &m,
                    "You need to enable feature tokio to use async closure under non-wasm32 \
                     targets.",
                ));
            }
        }

        Ok(Self {
            closure,
            return_type,
            deps,
        })
    }
}

impl<const WITH_ASYNC_CLOSURE: bool> PreparedState<WITH_ASYNC_CLOSURE> {
    // Async closure is not stable, so we rewrite it to clsoure + async block
    pub fn rewrite_to_closure_with_async_block(&self) -> ExprClosure {
        let async_token = match &self.closure.asyncness {
            Some(m) => m,
            None => return self.closure.clone(),
        };

        let move_token = &self.closure.capture;
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

    pub fn to_token_stream_with_closure(&self) -> TokenStream {
        let deps = &self.deps;
        let rt = &self.return_type;
        let closure = self.rewrite_to_closure_with_async_block();

        match &self.closure.asyncness {
            Some(_) => quote! {
                ::yew::functional::use_prepared_state_with_suspension::<#rt, _, _, _>(#closure, #deps)
            },
            None => quote! {
                ::yew::functional::use_prepared_state::<#rt, _, _>(#closure, #deps)
            },
        }
    }

    // Expose a hook for the client side.
    //
    // The closure is stripped from the client side.
    pub fn to_token_stream_without_closure(&self) -> TokenStream {
        let deps = &self.deps;
        let rt = &self.return_type;

        match &self.closure.asyncness {
            Some(_) => quote! {
                ::yew::functional::use_prepared_state_with_suspension::<#rt, _>(#deps)
            },
            None => quote! {
                ::yew::functional::use_prepared_state::<#rt, _>(#deps)
            },
        }
    }
}
