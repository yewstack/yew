use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, ReturnType, Type, Visibility};

pub struct FunctionComponent {
    block: Box<Block>,
    props_type: Box<Type>,
    arg: FnArg,
    generics: Generics,
    vis: Visibility,
    attrs: Vec<Attribute>,
    name: Ident,
    return_type: Box<Type>,
}

impl Parse for FunctionComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let ItemFn {
                    attrs,
                    vis,
                    sig,
                    block,
                } = func;

                if sig.generics.lifetimes().next().is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.generics,
                        "function components can't have generic lifetime parameters",
                    ));
                }

                if sig.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "function components can't be async",
                    ));
                }

                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be function components",
                    ));
                }

                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "extern functions can't be function components",
                    ));
                }

                let return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "function components must return `yew::Html`",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };

                let mut inputs = sig.inputs.into_iter();
                let arg: FnArg = inputs
                    .next()
                    .unwrap_or_else(|| syn::parse_quote! { _: &() });

                let ty = match &arg {
                    FnArg::Typed(arg) => match &*arg.ty {
                        Type::Reference(ty) => {
                            if ty.lifetime.is_some() {
                                return Err(syn::Error::new_spanned(
                                    &ty.lifetime,
                                    "reference must not have a lifetime",
                                ));
                            }

                            if ty.mutability.is_some() {
                                return Err(syn::Error::new_spanned(
                                    &ty.mutability,
                                    "reference must not be mutable",
                                ));
                            }

                            ty.elem.clone()
                        }
                        ty => {
                            let msg = format!(
                                "expected a reference to a `Properties` type (try: `&{}`)",
                                ty.to_token_stream()
                            );
                            return Err(syn::Error::new_spanned(ty, msg));
                        }
                    },

                    FnArg::Receiver(_) => {
                        return Err(syn::Error::new_spanned(
                            arg,
                            "function components can't accept a receiver",
                        ));
                    }
                };

                // Checking after param parsing may make it a little inefficient
                // but that's a requirement for better error messages in case of receivers
                // `>0` because first one is already consumed.
                if inputs.len() > 0 {
                    let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
                    return Err(syn::Error::new_spanned(
                        params,
                        "function components can accept at most one parameter for the props",
                    ));
                }

                Ok(Self {
                    props_type: ty,
                    block,
                    arg,
                    generics: sig.generics,
                    vis,
                    attrs,
                    name: sig.ident,
                    return_type,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "`function_component` attribute can only be applied to functions",
            )),
        }
    }
}

pub struct FunctionComponentName {
    component_name: Option<Ident>,
}

impl Parse for FunctionComponentName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                component_name: None,
            });
        }

        let component_name = input.parse()?;

        Ok(Self {
            component_name: Some(component_name),
        })
    }
}

pub fn function_component_impl(
    name: FunctionComponentName,
    component: FunctionComponent,
) -> syn::Result<TokenStream> {
    let FunctionComponentName { component_name } = name;

    let FunctionComponent {
        block,
        props_type,
        arg,
        generics,
        vis,
        attrs,
        name: function_name,
        return_type,
    } = component;
    let component_name = component_name.unwrap_or_else(|| function_name.clone());
    let function_name = format_ident!(
        "{}FunctionProvider",
        function_name,
        span = function_name.span()
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if function_name == component_name {
        return Err(syn::Error::new_spanned(
            component_name,
            "the component must not have the same name as the function",
        ));
    }

    let ret_type = quote_spanned!(return_type.span()=> ::yew::html::Html);

    let phantom_generics = generics
        .type_params()
        .map(|ty_param| ty_param.ident.clone()) // create a new Punctuated sequence without any type bounds
        .collect::<Punctuated<_, Comma>>();

    let quoted = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #vis struct #function_name #generics {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        impl #impl_generics ::yew::functional::FunctionProvider for #function_name #ty_generics #where_clause {
            type TProps = #props_type;

            fn run(#arg) -> #ret_type {
                #block
            }
        }

        #(#attrs)*
        #[allow(type_alias_bounds)]
        #vis type #component_name #generics = ::yew::functional::FunctionComponent<#function_name #ty_generics>;
    };

    Ok(quoted)
}
