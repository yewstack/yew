use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens}; // , quote_spanned
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
// use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, FnArg, Generics, Ident, Item, ItemFn, ReturnType, Type, Visibility}; // Block

pub struct FunctionComponent {
    // block: Box<Block>,
    props_type: Box<Type>,
    // arg: FnArg,
    generics: Generics,
    vis: Visibility,
    attrs: Vec<Attribute>,
    name: Ident,
    // return_type: Box<Type>,
    func: ItemFn,

    has_arg: bool,
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
                    // block,
                    ..
                } = func.clone();

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

                let _return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "function components must return `yew::Html` or `yew::HtmlResult`",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };

                let mut inputs = sig.inputs.into_iter();
                let (arg, has_arg) = inputs
                    .next()
                    .map(|m| (m, true))
                    .unwrap_or_else(|| (syn::parse_quote! { _: &() }, false));

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
                    // block,
                    // arg,
                    generics: sig.generics,
                    vis,
                    attrs,
                    name: sig.ident,
                    // return_type,
                    func,
                    has_arg,
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
    component_name: Ident,
}

impl Parse for FunctionComponentName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(input.error("expected identifier for the component"));
        }

        let component_name = input.parse()?;

        Ok(Self { component_name })
    }
}

pub fn function_component_impl(
    name: FunctionComponentName,
    component: FunctionComponent,
) -> syn::Result<TokenStream> {
    let FunctionComponentName { component_name } = name;

    let FunctionComponent {
        // block,
        props_type,
        // arg,
        generics,
        vis,
        attrs,
        name: function_name,
        // return_type,
        func,
        has_arg,
    } = component;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if function_name == component_name {
        return Err(syn::Error::new_spanned(
            component_name,
            "the component must not have the same name as the function",
        ));
    }

    let mut provider_name_str = component_name.to_string();

    if provider_name_str.ends_with("Internal") {
        // When a component ends with Internal, it will become possessive.
        // InternalsInternal instead of InternalInternal
        provider_name_str.push_str("sInternal");
    } else {
        provider_name_str.push_str("Internal");
    }

    let provider_name = Ident::new(&provider_name_str, Span::mixed_site());

    // let ret_type = quote_spanned!(return_type.span()=> ::yew::html::Html);

    let phantom_generics = generics
        .type_params()
        .map(|ty_param| ty_param.ident.clone()) // create a new Punctuated sequence without any type bounds
        .collect::<Punctuated<_, Comma>>();

    let run_impl = if has_arg {
        let provider_props = Ident::new("props", Span::mixed_site());

        quote! {
            fn run(#provider_props: &Self::TProps) -> ::yew::html::HtmlResult {
                #func

                #function_name(#provider_props).into()
            }
        }
    } else {
        let provider_props = Ident::new("_props", Span::mixed_site());

        quote! {
            fn run(#provider_props: &Self::TProps) -> ::yew::html::HtmlResult {
                #func

                #function_name().into()
            }
        }
    };

    let quoted = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #vis struct #provider_name #impl_generics {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        #[automatically_derived]
        impl #impl_generics ::yew::functional::FunctionProvider for #provider_name #ty_generics #where_clause {
            type TProps = #props_type;

            #run_impl
        }

        #(#attrs)*
        #[allow(type_alias_bounds)]
        #vis type #component_name #impl_generics = ::yew::functional::FunctionComponent<#provider_name #ty_generics>;
    };

    Ok(quoted)
}
