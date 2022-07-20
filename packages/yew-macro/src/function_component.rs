use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Fn};
use syn::{Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, ReturnType, Type, Visibility};

#[derive(Clone)]
pub struct FunctionComponent {
    block: Box<Block>,
    props_type: Box<Type>,
    arg: FnArg,
    generics: Generics,
    vis: Visibility,
    attrs: Vec<Attribute>,
    name: Ident,
    return_type: Box<Type>,
    fn_token: Fn,
}

impl Parse for FunctionComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        let func = match parsed {
            Item::Fn(m) => m,

            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    "`function_component` attribute can only be applied to functions",
                ))
            }
        };

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
                    "function components must return `yew::Html` or `yew::HtmlResult`",
                ))
            }
            ReturnType::Type(_, ty) => ty,
        };

        let mut inputs = sig.inputs.into_iter();
        let arg = inputs
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
            fn_token: sig.fn_token,
        })
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

fn print_fn(func_comp: FunctionComponent, use_fn_name: bool) -> TokenStream {
    let FunctionComponent {
        fn_token,
        name,
        attrs,
        block,
        return_type,
        generics,
        arg,
        ..
    } = func_comp;

    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let name = if use_fn_name {
        name
    } else {
        Ident::new("inner", Span::mixed_site())
    };

    quote! {
        #(#attrs)*
        #fn_token #name #ty_generics (#arg) -> #return_type
        #where_clause
        {
            #block
        }
    }
}

pub fn function_component_impl(
    name: FunctionComponentName,
    component: FunctionComponent,
) -> syn::Result<TokenStream> {
    let FunctionComponentName { component_name } = name;

    let has_separate_name = component_name.is_some();

    let func = print_fn(component.clone(), has_separate_name);

    let FunctionComponent {
        props_type,
        generics,
        vis,
        name: function_name,
        ..
    } = component;
    let component_name = component_name.unwrap_or_else(|| function_name.clone());
    let provider_name = format_ident!(
        "{}FunctionProvider",
        component_name,
        span = Span::mixed_site()
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if has_separate_name && function_name == component_name {
        return Err(syn::Error::new_spanned(
            component_name,
            "the component must not have the same name as the function",
        ));
    }

    let phantom_generics = generics
        .type_params()
        .map(|ty_param| ty_param.ident.clone()) // create a new Punctuated sequence without any type bounds
        .collect::<Punctuated<_, Comma>>();

    let provider_props = Ident::new("props", Span::mixed_site());

    let fn_generics = ty_generics.as_turbofish();

    let fn_name = if has_separate_name {
        function_name
    } else {
        Ident::new("inner", Span::mixed_site())
    };

    let quoted = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #vis struct #provider_name #ty_generics {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        #[automatically_derived]
        impl #impl_generics ::yew::functional::FunctionProvider for #provider_name #ty_generics #where_clause {
            type TProps = #props_type;

            fn run(#provider_props: &Self::TProps) -> ::yew::html::HtmlResult {
                #func

                ::yew::html::IntoHtmlResult::into_html_result(#fn_name #fn_generics (#provider_props))
            }
        }

        #[allow(type_alias_bounds)]
        #vis type #component_name #generics = ::yew::functional::FunctionComponent<#provider_name #ty_generics>;
    };

    Ok(quoted)
}
