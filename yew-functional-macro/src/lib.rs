use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Block, FnArg, Ident, Item, ItemFn, Type, Visibility};

struct FunctionalComponent {
    body: Box<Block>,
    props_type: Box<Type>,
    arg: FnArg,
    vis: Visibility,
}

impl Parse for FunctionalComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let ItemFn {
                    attrs: _,
                    vis,
                    sig,
                    block,
                } = func;

                if !sig.generics.params.is_empty() {
                    // TODO maybe find a way to handle those
                    return Err(syn::Error::new_spanned(
                        sig.generics,
                        "functional components can't contain generics",
                    ));
                }

                if sig.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "functional components can't be async",
                    ));
                }

                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be functional components",
                    ));
                }

                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "extern functions can't be functional components",
                    ));
                }

                let mut inputs = sig.inputs.into_iter();
                let arg: FnArg = inputs
                    .next()
                    .unwrap_or_else(|| syn::parse_quote! { _: &() });

                // Check here so we don't compute anything if params are invalid
                // `>0` because first one is already consumed.
                if inputs.len() > 0 {
                    let params: TokenStream = inputs.map(|it| it.to_token_stream()).collect();
                    return Err(syn::Error::new_spanned(
                        params,
                        "functional components can accept at most one parameter for the props",
                    ));
                }

                let ty = match &arg {
                    FnArg::Typed(arg) => match &*arg.ty {
                        Type::Reference(ty) => {
                            if ty.lifetime.is_some() {
                                return Err(syn::Error::new_spanned(
                                    &ty.lifetime,
                                    "reference must not have life time",
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
                            "functional components can't accept a receiver",
                        ));
                    }
                };

                Ok(Self {
                    body: block,
                    props_type: ty,
                    arg,
                    vis,
                })
            }
            _ => Err(syn::Error::new(
                Span::call_site(),
                "`functional_component` attribute can only be applied to functions",
            )),
        }
    }
}

struct FunctionalComponentName {
    function_name: Ident,
    component_name: Ident,
}

impl Parse for FunctionalComponentName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_name = input.parse().map_err(|e| {
            syn::Error::new(
                e.span(),
                format!("invalid name for component provided ({})", e.to_string()),
            )
        })?;
        let function_name = format_ident!("Function{}", component_name);

        Ok(Self {
            function_name,
            component_name,
        })
    }
}

#[proc_macro_attribute]
pub fn functional_component(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let FunctionalComponent {
        body,
        props_type,
        arg,
        vis,
    } = parse_macro_input!(item as FunctionalComponent);

    let FunctionalComponentName {
        function_name,
        component_name,
    } = parse_macro_input!(attr as FunctionalComponentName);

    let quoted = quote! {
        #vis struct #function_name;

        impl ::yew_functional::FunctionProvider for #function_name {
            type TProps = #props_type;

            fn run(#arg) -> ::yew::html::Html {
                #body
            }
        }

        #vis type #component_name = ::yew_functional::FunctionComponent<#function_name>;
    };

    quoted.into()
}
