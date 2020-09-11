use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Lit, LitStr};

/// Stringify a value at runtime.
pub fn stringify_at_runtime(src: impl ToTokens) -> TokenStream {
    quote_spanned! {src.span()=>
        ::std::borrow::Cow::<'static, str>::Owned(
            ::std::string::ToString::to_string(&(#src)),
        )
    }
}

/// Map an `Option` type such that it turns into `Cow<'static, str>`.
pub fn stringify_option_at_runtime(src: impl ToTokens) -> TokenStream {
    let ident = Ident::new("__yew_str", src.span());
    let sr = stringify_at_runtime(&ident);
    quote! {
        ::std::option::Option::map(#src, |#ident| {
            #sr
        })
    }
}

/// Create `Cow<'static, str>` construction calls.
///
/// This is deliberately not implemented for strings to preserve spans.
pub trait Stringify {
    /// Try to turn the value into a string literal.
    fn try_into_lit(&self) -> Option<LitStr>;
    /// Create `Cow<'static, str>` however possible.
    fn stringify(&self) -> TokenStream;
}
impl<T: Stringify + ?Sized> Stringify for &T {
    fn try_into_lit(&self) -> Option<LitStr> {
        (*self).try_into_lit()
    }

    fn stringify(&self) -> TokenStream {
        (*self).stringify()
    }
}

impl Stringify for LitStr {
    fn try_into_lit(&self) -> Option<LitStr> {
        Some(self.clone())
    }

    fn stringify(&self) -> TokenStream {
        quote_spanned! {self.span()=>
            ::std::borrow::Cow::<'static, str>::Borrowed(#self)
        }
    }
}
impl Stringify for Lit {
    fn try_into_lit(&self) -> Option<LitStr> {
        let s = match self {
            Lit::Str(v) => v.value(),
            Lit::Char(v) => v.value().to_string(),
            Lit::Int(v) => v.base10_digits().to_string(),
            Lit::Float(v) => v.base10_digits().to_string(),
            Lit::Bool(v) => v.value.to_string(),
            _ => return None,
        };
        Some(LitStr::new(&s, self.span()))
    }

    fn stringify(&self) -> TokenStream {
        self.try_into_lit()
            .as_ref()
            .map(Stringify::stringify)
            .unwrap_or_else(|| stringify_at_runtime(self))
    }
}
impl Stringify for Expr {
    fn try_into_lit(&self) -> Option<LitStr> {
        if let Expr::Lit(v) = self {
            v.lit.try_into_lit()
        } else {
            None
        }
    }

    fn stringify(&self) -> TokenStream {
        self.try_into_lit()
            .as_ref()
            .map(Stringify::stringify)
            .unwrap_or_else(|| stringify_at_runtime(self))
    }
}
