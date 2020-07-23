use quote::{quote, ToTokens};
use syn::{Expr, Lit};

pub struct StringRefConstructor {
	pub src: Expr,
}

impl StringRefConstructor {
	pub fn new(src: Expr) -> Self {
		Self { src }
	}

	pub fn try_convert_literal(src: &Expr) -> Option<String> {
		match src {
			Expr::Lit(l) => match &l.lit {
				Lit::Str(v) => Some(v.value()),
				Lit::Char(v) => Some(v.value().to_string()),
				Lit::Int(v) => Some(v.base10_digits().to_string()),
				Lit::Float(v) => Some(v.base10_digits().to_string()),
				Lit::Bool(v) => Some(v.value.to_string()),
				_ => None,
			},
			_ => None,
		}
	}
}

impl ToTokens for StringRefConstructor {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		tokens.extend(match Self::try_convert_literal(&self.src) {
			Some(s) => quote! { ::yew::StringRef::Static(#s) },
			None => {
				let src = &self.src;
				quote! { ::yew::StringRef::from(#src.to_string()) }
			}
		});
	}
}
