use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_quote, Token};
use syn::punctuated::Punctuated;
use crate::typed_vdom::{AttributePropDefinition, kw};

pub struct ListenerPropDefinition {
    name: Ident,
    ty: Ident,
    ident: Ident,
}

impl Parse for ListenerPropDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _separator = input.parse::<Token![:]>();
        let ty = input.parse().unwrap_or_else(|_| parse_quote!(::web_sys::Event));
        let ident = format_ident!("on_{}", name);
        Ok(Self { name, ty, ident })
    }
}

impl ListenerPropDefinition {
    fn build_fields(&self) -> TokenStream {
        let ListenerPropDefinition { ident, ty, .. } = self;

        quote! {
            pub #ident: ::std::option::Option::<::yew::Callback::<#ty>>,
        }
    }

    fn build_setter(&self) -> TokenStream {
        let ListenerPropDefinition { ty, ident, .. } = self;

        quote! {
            pub fn #ident(&mut self, val: ::yew::Callback::<#ty>) {
                self.#ident = ::std::option::Option::Some(val);
            }
        }
    }

    fn build_if_lets(&self) -> TokenStream {
        let ListenerPropDefinition { name, ident, .. } = self;

        let listener_path: Ident = syn::parse_str(&format!("on{}", name)).unwrap();
        quote! {
            if let Some(callback) = self.#ident {
                let cb: ::std::rc::Rc::<dyn ::yew::virtual_dom::Listener> = ::std::rc::Rc::new(::yew::html::#listener_path::Wrapper::new(callback));
                listeners.push(::std::option::Option::Some(cb));
            }
        }
    }
}

pub struct GlobalAttributes {
    attrs: Punctuated<AttributePropDefinition, Token![,]>,
    listeners: Punctuated<ListenerPropDefinition, Token![,]>,
}

impl Parse for GlobalAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = {
            let _props_kw = input.parse::<kw::attrs>()?;

            let buf;
            let _brace_token = braced!(buf in input);
            buf
        };
        let attrs = Punctuated::parse_terminated(&attrs)?;

        let listeners = {
            let _props_kw = input.parse::<kw::listeners>()?;

            let buf;
            let _brace_token = braced!(buf in input);
            buf
        };
        let listeners = Punctuated::parse_terminated(&listeners)?;

        Ok(Self { attrs, listeners })
    }
}

impl ToTokens for GlobalAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let GlobalAttributes { attrs, listeners } = self;

        let attributes = attrs.iter().map(|it| it.build_fields());
        let setters = attrs.iter().map(|it| it.build_setter());
        let attribute_if_lets = attrs.iter().map(|it| it.build_if_lets());

        let listeners_fields = listeners.iter().map(|it| it.build_fields());
        let listeners_setters = listeners.iter().map(|it| it.build_setter());
        let listeners_if_lets = listeners.iter().map(|it| it.build_if_lets());


        let output = quote! {
            #[derive(::std::default::Default, ::std::clone::Clone, ::std::fmt::Debug, ::yew::html::Properties, ::std::cmp::PartialEq)]
            struct GlobalAttributes {
                #(#attributes)*
                #(#listeners_fields)*
            }

            impl GlobalAttributes {
                #(#setters)*
                #(#listeners_setters)*
            }

            impl PropsExtend for GlobalAttributes {
                fn into_data(self) -> ElementData {
                    ElementData {
                        node_ref: ::std::default::Default::default(),
                        attributes: {
                            let mut attrs = ::std::collections::HashMap::new();
                            #(#attribute_if_lets)*
                            attrs
                        },
                        listeners: {
                            let mut listeners = ::std::vec::Vec::new();
                            #(#listeners_if_lets)*
                            listeners
                        },
                        key: None,
                        children: ::std::default::Default::default(),
                    }
                }
            }
        };

        tokens.extend(output)
    }
}
