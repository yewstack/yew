use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
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

/// Converts literals and expressions to Cow<'static, str> construction calls
pub struct Constructor(TokenStream);

macro_rules! stringify_at_runtime {
    ($src:expr) => {{
        let src = $src;
        Self(quote_spanned! {src.span()=>
            ::std::borrow::Cow::<'static, str>::Owned(
                ::std::string::ToString::to_string(&#src),
            )
        })
    }};
}

impl From<&Expr> for Constructor {
    fn from(src: &Expr) -> Self {
        match try_stringify_expr(src) {
            Some(s) => Self::from(s),
            None => stringify_at_runtime!(src),
        }
    }
}

impl From<&Lit> for Constructor {
    fn from(src: &Lit) -> Self {
        match try_stringify_lit(src) {
            Some(s) => Self::from(s),
            None => stringify_at_runtime!(src),
        }
    }
}

impl From<String> for Constructor {
    fn from(src: String) -> Self {
        Self(quote! { ::std::borrow::Cow::<'static, str>::Borrowed(#src) })
    }
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(std::iter::once(self.0.clone()));
    }
}
