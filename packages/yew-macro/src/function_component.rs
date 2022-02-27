use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Fn};
use syn::{
    parse_quote_spanned, visit_mut, Attribute, Block, FnArg, Generics, Ident, Item, ItemFn,
    ReturnType, Type, Visibility,
};

use crate::hook::BodyRewriter;

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

    component_name: Option<Ident>,
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
            component_name: None,
        })
    }
}

impl FunctionComponent {
    /// Filters attributes that should be copied to component definition.
    fn filter_attrs_for_component_struct(&self) -> Vec<Attribute> {
        self.attrs
            .iter()
            .filter_map(|m| {
                m.path
                    .get_ident()
                    .and_then(|ident| match ident.to_string().as_str() {
                        "doc" | "allow" => Some(m.clone()),
                        _ => None,
                    })
            })
            .collect()
    }

    /// Filters attributes that should be copied to the component impl block.
    fn filter_attrs_for_component_impl(&self) -> Vec<Attribute> {
        self.attrs
            .iter()
            .filter_map(|m| {
                m.path
                    .get_ident()
                    .and_then(|ident| match ident.to_string().as_str() {
                        "allow" => Some(m.clone()),
                        _ => None,
                    })
            })
            .collect()
    }

    fn phantom_generics(&self) -> Punctuated<Ident, Comma> {
        self.generics
            .type_params()
            .map(|ty_param| ty_param.ident.clone()) // create a new Punctuated sequence without any type bounds
            .collect::<Punctuated<_, Comma>>()
    }

    fn merge_component_name(&mut self, name: FunctionComponentName) -> syn::Result<()> {
        if let Some(ref m) = name.component_name {
            if m == &self.name {
                return Err(syn::Error::new_spanned(
                    m,
                    "the component must not have the same name as the function",
                ));
            }
        }

        self.component_name = name.component_name;

        Ok(())
    }

    fn inner_fn_ident(&self) -> Ident {
        if self.component_name.is_some() {
            self.name.clone()
        } else {
            Ident::new("inner", Span::mixed_site())
        }
    }

    fn component_name(&self) -> Ident {
        self.component_name
            .clone()
            .unwrap_or_else(|| self.name.clone())
    }

    // We need to cast 'static on all generics for into component.
    fn into_component_generics(&self) -> Generics {
        let mut generics = self.generics.clone();

        let where_clause = generics.make_where_clause();
        for ty_generic in self.generics.type_params() {
            let ident = &ty_generic.ident;
            let bound = parse_quote_spanned! { ident.span() =>
                #ident: 'static
            };

            where_clause.predicates.push(bound);
        }

        generics
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

fn print_fn(func_comp: &FunctionComponent) -> TokenStream {
    let name = func_comp.inner_fn_ident();
    let FunctionComponent {
        ref fn_token,
        ref attrs,
        ref block,
        ref return_type,
        ref generics,
        ref arg,
        ..
    } = func_comp;
    let mut block = *block.clone();
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // We use _ctx here so if the component does not use any hooks, the usused_vars lint will not
    // be triggered.
    let ctx_ident = Ident::new("_ctx", Span::mixed_site());

    let mut body_rewriter = BodyRewriter::new(ctx_ident.clone());
    visit_mut::visit_block_mut(&mut body_rewriter, &mut block);

    quote! {
        #(#attrs)*
        #fn_token #name #impl_generics (#ctx_ident: &mut ::yew::functional::HookContext, #arg) -> #return_type
        #where_clause
        {
            #block
        }
    }
}

pub fn function_component_impl(
    name: FunctionComponentName,
    mut component: FunctionComponent,
) -> syn::Result<TokenStream> {
    component.merge_component_name(name)?;

    let func = print_fn(&component);

    let into_comp_generics = component.into_component_generics();
    let component_attrs = component.filter_attrs_for_component_struct();
    let component_impl_attrs = component.filter_attrs_for_component_impl();
    let phantom_generics = component.phantom_generics();
    let component_name = component.component_name();
    let fn_name = component.inner_fn_ident();

    let FunctionComponent {
        props_type,
        generics,
        vis,
        ..
    } = component;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fn_generics = ty_generics.as_turbofish();

    let component_props = Ident::new("props", Span::mixed_site());
    let ctx_ident = Ident::new("ctx", Span::mixed_site());

    let into_comp_impl = {
        let (impl_generics, ty_generics, where_clause) = into_comp_generics.split_for_impl();

        quote! {
            impl #impl_generics ::yew::html::IntoComponent for #component_name #ty_generics #where_clause {
                type Message = ();
                type Properties = #props_type;
                type Component = ::yew::functional::FunctionComponent<#component_name #ty_generics>;
            }
        }
    };

    let quoted = quote! {
        #(#component_attrs)*
        #[allow(unused_parens)]
        #vis struct #component_name #generics #where_clause {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        // we cannot disable any lints here because it will be applied to the function body
        // as well.
        #(#component_impl_attrs)*
        impl #impl_generics ::yew::functional::FunctionProvider for #component_name #ty_generics #where_clause {
            type Properties = #props_type;

            fn run(#ctx_ident: &mut ::yew::functional::HookContext, #component_props: &Self::Properties) -> ::yew::html::HtmlResult {
                #func

                ::yew::html::IntoHtmlResult::into_html_result(#fn_name #fn_generics (#ctx_ident, #component_props))
            }
        }

        #into_comp_impl
    };

    Ok(quoted)
}
