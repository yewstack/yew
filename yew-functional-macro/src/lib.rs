use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Block, FnArg, Ident, Item, Type};

struct FunctionalComponent {
    body: Box<Block>,
    props_type: Type,
    props_name: Ident,
}

impl Parse for FunctionalComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {

                if !func.sig.generics.params.is_empty() {
                    // TODO maybe find a way to handle those
                    return Err(syn::Error::new_spanned(
                        func.sig.generics,
                        "functional components cannot contain generics",
                    ));
                }

                let inputs = &func.sig.inputs;

                if inputs.len() > 1 {
                    return Err(syn::Error::new_spanned(
                        inputs,
                        "functional components must take only parameter of props",
                    ))
                }

                let (props_type, props_name) = if let Some(arg) = inputs.into_iter().next() {
                    match arg {
                        FnArg::Typed(arg) => {
                            let ident = match &*arg.pat {
                                syn::Pat::Ident(ident) => &ident.ident,
                                pat => {
                                    return Err(syn::Error::new_spanned(
                                        pat,
                                        "Cannot obtain ident. This should never happen",
                                    ))
                                }
                            };

                            let ty = match &*arg.ty {
                                Type::Reference(ty) => {
                                    if ty.lifetime.is_some() {
                                        return Err(syn::Error::new_spanned(&ty.lifetime, "reference must not have life time"));
                                    }

                                    if ty.mutability.is_some() {
                                        return Err(syn::Error::new_spanned(&ty.mutability, "reference must not be mutable"));
                                    }

                                    &*ty.elem
                                }
                                ty => return Err(syn::Error::new_spanned(ty, "invalid argument passed. (hint: type must be props struct by reference)"))
                            };

                            (ty.clone(), ident.clone())
                        }

                        arg => {
                            return Err(syn::Error::new_spanned(
                                arg,
                                "functional components cannot accept a receiver",
                            ));
                        }
                    }
                } else {
                    (
                        Type::Tuple(syn::TypeTuple {
                            paren_token: Default::default(),
                            elems: Default::default(),
                        }),
                        Ident::new("_", Span::call_site()),
                    )
                };

                Ok(Self {
                    body: func.block,
                    props_type,
                    props_name,
                })
            }
            _ => Err(syn::Error::new(
                Span::call_site(),
                "`functional_component` can only be applied to functions",
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
        let component_name = input.parse()?;
        let function_name = format_ident!("Function{}", component_name);

        Ok(Self {
            function_name,
            component_name,
        })
    }
}

#[proc_macro_attribute]
pub fn functional_component(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let FunctionalComponent {
        body,
        props_type,
        props_name,
    } = parse_macro_input!(item as FunctionalComponent);

    let FunctionalComponentName {
        function_name,
        component_name,
    } = parse_macro_input!(attr as FunctionalComponentName);

    let quoted = quote! {
        pub struct #function_name;

        impl ::yew_functional::FunctionProvider for #function_name {
            type TProps = #props_type;

            fn run(#props_name: &Self::TProps) -> ::yew::html::Html {
                #body
            }
        }

        pub type #component_name = ::yew_functional::FunctionComponent<#function_name>;
    };

    TokenStream1::from(quoted)
}
