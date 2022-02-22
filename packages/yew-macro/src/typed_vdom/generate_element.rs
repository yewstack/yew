use crate::typed_vdom::globals::listeners;
use crate::typed_vdom::{all_shared_attributes, kw, AttributePropDefinition};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Token};

pub struct GenerateElement {
    element_name: Ident,
    props: Vec<AttributePropDefinition>,
}

impl Parse for GenerateElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _separator = input.parse::<Token![;]>()?;

        let props = {
            let _props_kw = input.parse::<kw::props>()?;
            let _separator = input.parse::<Token![:]>()?;

            let buf;
            let _brace_token = braced!(buf in input);
            buf
        };
        let props: Punctuated<AttributePropDefinition, Token![,]> =
            Punctuated::parse_terminated(&props)?;

        Ok(Self {
            element_name: name,
            props: props.into_iter().collect(),
        })
    }
}

impl ToTokens for GenerateElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let element_name = &self.element_name;
        let prop_definitions = &self.props;
        let all_prop_definitions = {
            let mut all_prop_definitions = all_shared_attributes();
            all_prop_definitions.extend(self.props.clone());
            all_prop_definitions
        };
        let props_ident = format_ident!(
            "{}Props",
            element_name.to_string().to_case(Case::Pascal),
            span = element_name.span()
        );
        let props = prop_definitions.iter().map(|it| it.build_fields());
        let attr_if_lets = all_prop_definitions.iter().map(|it| it.build_if_lets());

        let all_listeners = listeners();
        let listeners_if_lets = all_listeners.iter().map(|it| it.build_if_lets());

        let vtag_init = match element_name.to_string().trim() {
            "input" => quote! {
                ::yew::virtual_dom::VTag::__new_input(
                    element.attributes.remove("value"),
                    element.attributes.remove("checked").map(|_| true).unwrap_or(false),
                    element.node_ref,
                    element.key,
                    ::yew::virtual_dom::Attributes::IndexMap(element.attributes.into_iter().collect()),
                    ::yew::virtual_dom::Listeners::Pending(element.listeners.into_boxed_slice()),
                )
            },
            _ => quote! {
                ::yew::virtual_dom::VTag::__new_other(
                    ::std::stringify!(#element_name).into(),
                    element.node_ref,
                    element.key,
                    ::yew::virtual_dom::Attributes::IndexMap(element.attributes.into_iter().collect()),
                    ::yew::virtual_dom::Listeners::Pending(element.listeners.into_boxed_slice()),
                    ::yew::virtual_dom::VList::with_children(element.children, ::std::option::Option::None),
                )
            },
        };

        let out = quote! {
            #[allow(non_camel_case_types)]
            pub struct #element_name;

            #[derive(::std::default::Default, ::std::clone::Clone, ::std::fmt::Debug, ::yew::html::Properties, ::std::cmp::PartialEq)]
            pub struct #props_ident {
                #[prop_or_default]
                pub __globals: ::yew::virtual_dom::typings::globals::Globals,
                #[prop_or_default]
                pub node_ref: ::std::option::Option::<::yew::NodeRef>,
                #[prop_or_default]
                pub key: ::std::option::Option::<::yew::virtual_dom::Key>,
                #[prop_or_default]
                pub children: ::yew::Children,
                #(#props)*
            }

            impl std::ops::Deref for #props_ident {
                type Target = ::yew::virtual_dom::typings::globals::Globals;

                fn deref(&self) -> &Self::Target {
                    &self.__globals
                }
            }


            impl std::ops::DerefMut for #props_ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.__globals
                }
            }

            impl #props_ident {
                fn into_data(self) -> ::yew::virtual_dom::typings::ElementData {

                    ::yew::virtual_dom::typings::ElementData {
                        node_ref: ::std::option::Option::unwrap_or_default(self.node_ref.clone()),
                        attributes: {
                            let mut attrs = ::std::collections::HashMap::new();
                            #(#attr_if_lets)*
                            attrs
                        },
                        listeners: {
                            let mut listeners = ::std::vec![];
                            #(#listeners_if_lets)*
                            listeners
                        },
                        key: self.key.clone(),
                        children: self.children.into_iter().collect(),
                    }
                }
            }

            impl ::yew::Component for #element_name {
                type Message = ();
                type Properties = #props_ident;

                fn create(_ctx: &::yew::html::Context<Self>) -> Self {
                    Self
                }

                fn view(&self, ctx: &::yew::html::Context<Self>) -> ::yew::html::Html {
                    #[allow(unused_mut)]
                    let mut element = ctx.props().clone().into_data();

                    ::std::convert::Into::<::yew::virtual_dom::VNode>::into({ #vtag_init })
                }
            }
        };

        tokens.extend(out);
    }
}
