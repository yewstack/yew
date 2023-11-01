use quote::quote;

use crate::function_component::FunctionComponentName;

#[derive(Clone)]
pub struct Autoprops {
    item_fn: syn::ItemFn,
    properties_name: syn::Ident,
}

impl syn::parse::Parse for Autoprops {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed: syn::Item = input.parse()?;

        let item_fn = match parsed {
            syn::Item::Fn(m) => m,
            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    "`autoprops` attribute can only be applied to functions",
                ))
            }
        };

        let syn::ItemFn { attrs, sig, .. } = &item_fn;

        let mut component_name = item_fn.sig.ident.clone();

        attrs
            .iter()
            .find(|attr| {
                match &attr.meta {
                    syn::Meta::Path(path) => {
                        if let Some(last_segment) = path.segments.last() {
                            if last_segment.ident == "function_component" {
                                return true;
                            }
                        }
                    }
                    syn::Meta::List(syn::MetaList { path, tokens, .. }) => {
                        if let Some(last_segment) = path.segments.last() {
                            if last_segment.ident == "function_component" {
                                if let Ok(attr) =
                                    syn::parse2::<FunctionComponentName>(tokens.clone())
                                {
                                    if let Some(name) = attr.component_name {
                                        component_name = name;
                                    }
                                }
                                return true;
                            }
                        }
                    }
                    _ => {}
                }
                false
            })
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    sig,
                    "could not find #[function_component] attribute in function declaration \
                    (#[autoprops] must be place *before* #[function_component])",
                )
            })?;

        for input in &sig.inputs {
            match input {
                syn::FnArg::Typed(syn::PatType { pat, .. }) => match pat.as_ref() {
                    syn::Pat::Wild(wild) => {
                        return Err(syn::Error::new_spanned(
                            wild,
                            "cannot use `_` as field name",
                        ));
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let properties_name = syn::Ident::new(
            &format!("{}Props", component_name),
            proc_macro2::Span::call_site(),
        );

        Ok(Self {
            properties_name,
            item_fn,
        })
    }
}

impl Autoprops {
    pub fn apply_args(&mut self, args: AutopropsArgs) {
        if let Some(name) = args.properties_name {
            self.properties_name = name;
        }
    }

    fn print_function_component(&self) -> proc_macro2::TokenStream {
        let properties_name = &self.properties_name;
        let syn::ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = &self.item_fn;

        let fn_name = &sig.ident;
        let (impl_generics, type_generics, where_clause) = sig.generics.split_for_impl();
        let inputs = if sig.inputs.is_empty() {
            quote! { (): &() }
        } else {
            // NOTE: function components currently don't accept receivers, we're just passing the
            //       information to the next macro to fail and give its own error message
            let receivers = sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Receiver(receiver) => Some(receiver),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let args = sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Typed(syn::PatType { pat, .. }) => Some(quote! { #pat }),
                    _ => None,
                })
                .collect::<Vec<_>>();
            quote! { #(#receivers,)* #properties_name { #(#args),* }: &#properties_name #type_generics }
        };
        let clones = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(syn::PatType { pat, ty, .. })
                    if !matches!(**ty, syn::Type::Reference(_)) =>
                {
                    Some(quote! { let #pat = ::std::clone::Clone::clone(#pat); })
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        quote! {
            #(#attrs)*
            #vis fn #fn_name #impl_generics (#inputs) -> ::yew::Html #where_clause {
                #(#clones)*
                #block
            }
        }
    }

    fn print_properties_struct(&self) -> proc_macro2::TokenStream {
        let properties_name = &self.properties_name;
        let syn::ItemFn { vis, sig, .. } = &self.item_fn;

        if sig.inputs.is_empty() {
            return quote! {};
        }

        let (impl_generics, _type_generics, where_clause) = sig.generics.split_for_impl();
        let fields = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(syn::PatType { attrs, pat, ty, .. }) => match ty.as_ref() {
                    syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                        Some(quote! { #(#attrs)* #pat: #elem, })
                    }
                    _ => Some(quote! { #(#attrs)* #pat: #ty, }),
                },
                _ => None,
            })
            .collect::<Vec<_>>();

        quote! {
            #[derive(::yew::Properties, ::std::cmp::PartialEq)]
            #vis struct #properties_name #impl_generics #where_clause {
                #(#fields)*
            }
        }
    }
}

impl quote::ToTokens for Autoprops {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let function_component = self.print_function_component();
        let properties_struct = self.print_properties_struct();

        tokens.extend(quote! {
            #function_component
            #properties_struct
        })
    }
}

pub struct AutopropsArgs {
    pub properties_name: Option<syn::Ident>,
}

impl syn::parse::Parse for AutopropsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                properties_name: None,
            });
        }

        let properties_name = input.parse()?;

        Ok(Self {
            properties_name: Some(properties_name),
        })
    }
}
