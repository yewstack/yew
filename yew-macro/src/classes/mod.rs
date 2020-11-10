use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, ExprLit, Lit, LitStr, Token};

/// List of HTML classes.
pub struct Classes(Punctuated<ClassExpr, Token![,]>);

impl Parse for Classes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse_terminated(|x| ClassExpr::parse(x)).map(Self)
    }
}

impl ToTokens for Classes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let n = self.0.len();
        let push_classes = self.0.iter().map(|x| match x {
            ClassExpr::Lit(class) => quote! {
                unsafe { __yew_classes.unchecked_push(#class) };
            },
            ClassExpr::Expr(class) => quote! {
                __yew_classes.push(#class);
            },
        });
        let new_tokens = quote! {
            let mut __yew_classes = ::yew::html::Classes::with_capacity(#n);
            #(#push_classes)*
            __yew_classes
        };

        tokens.extend(quote! {{
            #new_tokens
        }});
    }
}

enum ClassExpr {
    Lit(LitStr),
    Expr(Expr),
}

impl Parse for ClassExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Expr>()? {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                let value = lit_str.value();
                let classes = value.split_whitespace().collect::<Vec<_>>();
                if classes.len() > 1 {
                    let fix = classes
                        .into_iter()
                        .map(|class| format!("\"{}\"", class))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let msg = format!(
                        "string literals must not contain more than one class (hint: use `{}`)",
                        fix
                    );

                    Err(syn::Error::new(lit_str.span(), msg))
                } else {
                    Ok(Self::Lit(lit_str))
                }
            }
            expr => Ok(Self::Expr(expr)),
        }
    }
}
