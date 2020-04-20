use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, GenericParam, Generics};

// Todo, consider removing the T here and replacing it with an enum.
/// Creates the "impl <X,Y,Z> ::yew_router::Switch for TypeName<X,Y,Z> where etc.." line.
///
/// Then populates the body of the implementation with the specified `T`.
pub struct SwitchImpl<'a, T> {
    pub target_ident: &'a Ident,
    pub generics: &'a Generics,
    pub inner: T,
}

impl<'a, T: ToTokens> ToTokens for SwitchImpl<'a, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.target_ident;
        let inner = &self.inner;

        let line_tokens = if self.generics.params.is_empty() {
            quote! {
                impl ::yew_router::Switch for #ident {
                    #inner
                }
            }
        } else {
            let params = &self.generics.params;
            let param_idents = params
                .iter()
                .map(|p: &GenericParam| {
                    match p {
                        GenericParam::Type(ty) => ty.ident.clone(),
//                    GenericParam::Lifetime(lt) => lt.lifetime, // TODO different type here, must be handled by collecting into a new enum and defining how to convert _that_ to tokens.
                        _ => unimplemented!("Not all type parameter variants (lifetimes and consts) are supported in Switch")
                    }
                })
                .collect::<Punctuated<_,syn::token::Comma>>();

            let where_clause = &self.generics.where_clause;
            quote! {
                impl <#params> ::yew_router::Switch for #ident <#param_idents> #where_clause
                {
                    #inner
                }
            }
        };
        tokens.extend(line_tokens)
    }
}
