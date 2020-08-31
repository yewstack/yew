use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Expr, Lit};

/// Attempt converting expression to str, if it's a literal
pub fn try_stringify_expr(src: &Expr) -> Option<String> {
    match src {
        Expr::Lit(l) => try_stringify_lit(&l.lit),
        _ => None,
    }
}

/// Attempt converting literal to str literal
fn try_stringify_lit(src: &Lit) -> Option<String> {
    match src {
        Lit::Str(v) => Some(v.value()),
        Lit::Char(v) => Some(v.value().to_string()),
        Lit::Int(v) => Some(v.base10_digits().to_string()),
        Lit::Float(v) => Some(v.base10_digits().to_string()),
        Lit::Bool(v) => Some(v.value.to_string()),
        _ => None,
    }
}

pub fn stringify_at_runtime(src: impl ToTokens) -> TokenStream {
    quote_spanned! {src.span()=>
        ::std::borrow::Cow::<'static, str>::Owned(
            ::std::string::ToString::to_string(&(#src)),
        )
    }
}

pub fn stringify_static(src: impl ToTokens) -> TokenStream {
    quote_spanned! {src.span()=>
        ::std::borrow::Cow::<'static, str>::Borrowed(#src)
    }
}

/// Converts literals and expressions to Cow<'static, str> construction calls
pub struct Stringify(TokenStream);

impl From<&Expr> for Stringify {
    fn from(src: &Expr) -> Self {
        match try_stringify_expr(src) {
            Some(s) => Self::from(&s),
            None => Self(stringify_at_runtime(src)),
        }
    }
}
impl From<&Lit> for Stringify {
    fn from(src: &Lit) -> Self {
        match try_stringify_lit(src) {
            Some(s) => Self::from(&s),
            None => Self(stringify_at_runtime(src)),
        }
    }
}
impl From<&String> for Stringify {
    fn from(src: &String) -> Self {
        Self(stringify_static(src))
    }
}
impl From<&str> for Stringify {
    fn from(src: &str) -> Self {
        Self(stringify_static(src))
    }
}

impl ToTokens for Stringify {
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.0
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
