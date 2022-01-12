use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{braced, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::typed_vdom::{AttributePropDefinition, kw};

pub struct GenerateElement {
    element_name: Ident,
    props: Punctuated<AttributePropDefinition, Token![,]>,
    extends: Option<Ident>,
}

impl Parse for GenerateElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _separator = input.parse::<Token![;]>()?;

        let extends = {
            if input.parse::<kw::extends>().is_err() {
                None
            } else {
                let _separator = input.parse::<Token![:]>()?;
                let extends = input.parse()?;
                let _separator = input.parse::<Token![;]>()?;
                Some(extends)
            }
        };


        let props = {
            let _props_kw = input.parse::<kw::props>()?;
            let _separator = input.parse::<Token![:]>()?;

            let buf;
            let _brace_token = braced!(buf in input);
            buf
        };
        let props = Punctuated::parse_terminated(&props)?;


        Ok(Self {
            element_name: name,
            props,
            extends,
        })
    }
}

impl ToTokens for GenerateElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let GenerateElement { element_name, props: prop_definitions, extends } = self;

        let props_ident = format_ident!("{}Props", element_name.to_string().to_case(Case::Pascal), span = element_name.span());
        let props_len = prop_definitions.len();
        let props = prop_definitions.iter().map(|it| it.build_fields());
        let setters = prop_definitions.iter().map(|it| it.build_setter());
        let attribute_if_lets = prop_definitions.iter().map(|it| it.build_if_lets());

        let (extends_field, extends_impls, extend_element) = extends.as_ref().map(|e| impl_extends_deref(e, &props_ident)).unwrap_or_default();
        let (extend_var, extend_attributes) = if extend_element.is_empty() {
            (quote! {}, quote! {})
        } else {
            (
                quote! {
                    let parent = #extend_element;
                },
                quote! {
                    attrs.extend(parent.attributes)
                }
            )
        };

        let out = quote! {
            #[allow(non_camel_case_types)]
            struct #element_name;

            #[derive(::std::default::Default, ::std::clone::Clone, ::std::fmt::Debug, ::yew::html::Properties, ::std::cmp::PartialEq)]
            struct #props_ident {
                #(#props)*
                #extends_field
            }

            impl #props_ident {
                #(#setters)*
            }

            // todo qualified path
            impl PropsExtend for #props_ident{
                fn into_data(self) -> ElementData {
                    #extend_var;

                    ElementData {
                        node_ref: self.node_ref.unwrap_or_default(),
                        attributes: {
                            let mut attrs = ::std::collections::HashMap::with_capacity(#props_len);
                            #(#attribute_if_lets)*
                            #extend_attributes;
                            attrs
                        },
                        listeners: ::std::default::Default::default(),
                        key: self.key,
                        children: self.children.map(|it| it.into_iter().collect()).unwrap_or_default()
                    }
                }
            }

            impl ::yew::Component for #element_name {
                type Message = ();
                type Properties = #props_ident;

                fn create(_ctx: &Context<Self>) -> Self {
                    Self
                }

                fn view(&self, ctx: &Context<Self>) -> Html {
                    let element: ElementData = ctx.props().clone().into_data();
                    // todo use __new_{other, textarea, input} depending upon the element
                    VTag::__new_other(
                        ::std::stringify!(#element_name).into(),
                        element.node_ref,
                        element.key,
                        ::yew::virtual_dom::Attributes::IndexMap(element.attributes.into_iter().collect()),
                        ::yew::virtual_dom::Listeners::Pending(element.listeners.into_boxed_slice()),
                        ::yew::virtual_dom::VList::with_children(element.children, ::std::option::Option::None),
                    ).into()
                }
            }


            #extends_impls
        };

        tokens.extend(out);
    }
}

fn impl_extends_deref(extends: &Ident, props_ident: &Ident) -> (TokenStream, TokenStream, TokenStream) {
    let field = quote! { __parent: #extends };

    let impls = quote! {
        impl ::std::ops::Deref for #props_ident {
            type Target = #extends;

            fn deref(&self) -> &Self::Target {
                &self.__parent
            }
        }

        impl ::std::ops::DerefMut for #props_ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.__parent
            }
        }
    };
    let attributes = quote! {
        self.__parent.into_data()
    };

    (field, impls, attributes)
}
