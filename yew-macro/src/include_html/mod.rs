use crate::html;
use proc_macro2::TokenStream;
use syn::export::ToTokens;
use syn::parse::{Parse, ParseStream, Result};

pub struct IncludeHTML {
    file: String,
}

impl Parse for IncludeHTML {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<syn::LitStr>()?;
        return Ok(Self { file: lit.value() });
    }
}

impl ToTokens for IncludeHTML {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let text = match std::fs::read_to_string(&self.file) {
            Ok(text) => text,
            Err(e) => {
                tokens.extend(
                    syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!("Couldn't open the file. The underlying error is: `{}`.", e),
                    )
                    .to_compile_error(),
                );
                return;
            }
        };
        tokens.extend::<proc_macro2::TokenStream>(
            html(text.parse::<proc_macro2::TokenStream>().unwrap().into()).into(),
        );
    }
}
