use crate::typed_vdom::globals::{global_attributes, listeners};
use crate::typed_vdom::{kw, AttributePropDefinition};
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
        let mut prop_definitions = self.props.clone();
        prop_definitions.extend(global_attributes());

        let props_ident = format_ident!(
            "{}Props",
            element_name.to_string().to_case(Case::Pascal),
            span = element_name.span()
        );
        let props = prop_definitions.iter().map(|it| it.build_fields());
        let attr_if_lets = prop_definitions.iter().map(|it| it.build_if_lets());

        let all_listeners = listeners();
        let listeners = all_listeners.iter().map(|it| it.build_fields());
        let listeners_if_lets = all_listeners.iter().map(|it| it.build_if_lets());

        let out = quote! {
            #[allow(non_camel_case_types)]
            struct #element_name;

            #[derive(::std::default::Default, ::std::clone::Clone, ::std::fmt::Debug, ::yew::html::Properties, ::std::cmp::PartialEq)]
            struct #props_ident {
                #(#props)*
                #(#listeners)*
            }

            impl #props_ident {
                fn into_data(self) -> ElementData {

                    ElementData {
                        node_ref: ::std::option::Option::unwrap_or_default(self.node_ref),
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
                        key: self.key,
                        children: self.children.map(|it| it.into_iter().collect()).unwrap_or_default()
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
                    let element: ElementData = ctx.props().clone().into_data();
                    // todo use __new_{other, textarea, input} depending upon the element
                    ::yew::virtual_dom::VTag::__new_other(
                        ::std::stringify!(#element_name).into(),
                        element.node_ref,
                        element.key,
                        ::yew::virtual_dom::Attributes::IndexMap(element.attributes.into_iter().collect()),
                        ::yew::virtual_dom::Listeners::Pending(element.listeners.into_boxed_slice()),
                        ::yew::virtual_dom::VList::with_children(element.children, ::std::option::Option::None),
                    ).into()
                }
            }
        };

        tokens.extend(out);
    }
}
