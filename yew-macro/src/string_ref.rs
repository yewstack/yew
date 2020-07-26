use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
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
        Lit::Char(v) => Some(::std::string::ToString::to_string(&v.value())),
        Lit::Int(v) => Some(::std::string::ToString::to_string(&v.base10_digits())),
        Lit::Float(v) => Some(::std::string::ToString::to_string(&v.base10_digits())),
        Lit::Bool(v) => Some(::std::string::ToString::to_string(&v.value)),
        _ => None,
    }
}

/// Converts literals and expressions to yew::StringRef construction calls
pub struct Constructor(TokenStream);

impl From<&Expr> for Constructor {
    fn from(src: &Expr) -> Self {
        match try_stringify_expr(src) {
            Some(s) => Self::from(s),
            None => Self(quote! { ::yew::StringRef::from(#src.to_string()) }),
        }
    }
}

impl From<&Lit> for Constructor {
    fn from(src: &Lit) -> Self {
        match try_stringify_lit(src) {
            Some(s) => Self::from(s),
            None => Self(quote! { ::yew::StringRef::from(#src.to_string()) }),
        }
    }
}

impl From<String> for Constructor {
    fn from(src: String) -> Self {
        Self(quote! { ::yew::StringRef::Static(#src) })
    }
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(std::iter::once(self.0.clone()));
    }
}
