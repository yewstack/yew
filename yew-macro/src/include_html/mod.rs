use crate::html;
use proc_macro2::TokenStream;
use syn::export::ToTokens;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};

pub struct IncludeHTML {
    file: String,
    variables: Vec<syn::Ident>,
}

impl Parse for IncludeHTML {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<syn::LitStr>()?;
        let mut variables = Vec::new();
        loop {
            if input.is_empty() {
                return Ok(Self {
                    file: lit.value(),
                    variables,
                });
            }
            input.parse::<syn::Token![,]>()?;
            if input.is_empty() {
                return Ok(Self {
                    file: lit.value(),
                    variables,
                });
            }
            let ident = input.call(syn::Ident::parse_any)?;
            variables.push(ident);
        }
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
                        format!("Couldn't open the file. Error cause: `{}`.", e),
                    )
                    .to_compile_error(),
                );
                return;
            }
        };
        tokens.extend::<proc_macro2::TokenStream>(
            html(
                text.parse::<proc_macro2::TokenStream>()
                    .unwrap()
                    .into_iter()
                    .map(|item| match item {
                        proc_macro2::TokenTree::Ident(mut ident) => {
                            let found_identifier = self
                                .variables
                                .iter()
                                .filter(|syn_ident| syn_ident.to_string() == ident.to_string())
                                .next()
                                .map(|a| a.clone());
                            if let Some(found_identifier) = found_identifier {
                                ident.set_span(found_identifier.span())
                            }
                            proc_macro2::TokenTree::Ident(ident)
                        }
                        _ => item,
                    })
                    .collect::<proc_macro2::TokenStream>()
                    .into(),
            )
            .into(),
        );
    }
}
