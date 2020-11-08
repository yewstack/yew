use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Lit, Token};

/// List of HTML classes.
pub struct Classes(Punctuated<Expr, Token![,]>);

impl Parse for Classes {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse_terminated(class_expr_parser).map(Self)
    }
}

impl ToTokens for Classes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let n = self.0.len();
        let push_classes = self.0.iter().map(|x| match x {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => quote!(__yew_classes.unchecked_push(#lit_str);),
            x => quote!(__yew_classes.push(#x);),
        });
        let new_tokens = quote! {
            let mut __yew_classes = ::yew::virtual_dom::Classes::with_capacity(#n);
            #(#push_classes)*
            __yew_classes
        };

        tokens.extend(quote! {{
            #new_tokens
        }});
    }
}

fn class_expr_parser(input: ParseStream) -> Result<Expr> {
    let expr = Expr::parse(input)?;

    if let Expr::Lit(ExprLit {
        lit: Lit::Str(lit_str),
        ..
    }) = &expr
    {
        let value = lit_str.value();
        if value.contains(' ') {
            return Err(syn::Error::new(
                expr.span(),
                r"string literals should not contain spaces: please use two separate literals",
            ));
        }
    }

    Ok(expr)
}
