use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, Fields, Ident, Item, LitStr, Token, Type, Visibility};

pub struct CustomEvent {
    base: Item,
    ident: Ident,
    vis: Visibility,
    inner_type: Type,
}

const CUSTOM_EVENT_MSG: &str =
    "custom_event attribute macro can only be applied to NewType structs";

impl Parse for CustomEvent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<Item>()?;

        match &item {
            Item::Struct(item_struct) => {
                match (&item_struct.fields, item_struct.generics.lt_token) {
                    (Fields::Unnamed(ref uf), None) if uf.unnamed.len() == 1 => {
                        // unwrap safe because we just checked len of fields.
                        let inner_type = (uf.unnamed.first().unwrap().ty).clone();
                        Ok(CustomEvent {
                            ident: item_struct.ident.clone(),
                            inner_type,
                            vis: item_struct.vis.clone(),
                            base: item,
                        })
                    }
                    _ => Err(syn::Error::new_spanned(item_struct, CUSTOM_EVENT_MSG)),
                }
            }
            _ => Err(syn::Error::new_spanned(item, CUSTOM_EVENT_MSG)),
        }
    }
}

pub struct CustomEventName {
    event_ident: Ident,
    event_name: String,
}

impl Parse for CustomEventName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let event_ident = input.parse::<Ident>()?;
        let event_name = if input.parse::<Token![=]>().is_ok() {
            let event_name = input.parse::<LitStr>()?;
            event_name.value()
        } else {
            event_ident.to_string()
        };

        Ok(CustomEventName {
            event_ident,
            event_name,
        })
    }
}

pub fn custom_event_impl(name: CustomEventName, custom_event: CustomEvent) -> TokenStream {
    let CustomEventName {
        event_ident,
        event_name,
    } = name;

    let CustomEvent {
        base,
        ident,
        vis,
        inner_type,
    } = custom_event;

    quote! {

        #base

        impl ::std::ops::Deref for #ident{
            type Target = #inner_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::AsRef<::yew::web_sys::Event> for #ident {
            fn as_ref(&self) -> &::yew::web_sys::Event {
                &self.0
            }
        }

        impl ::std::convert::AsRef<::wasm_bindgen::JsValue> for #ident {
            fn as_ref(&self) -> &::wasm_bindgen::JsValue {
                &self.0
            }
        }

        #[allow(clippy::from_over_into)]
        impl ::std::convert::Into<::wasm_bindgen::JsValue> for #ident {
            fn into(self) -> ::wasm_bindgen::JsValue {
                use ::wasm_bindgen::JsCast;
                self.0.unchecked_into()
            }
        }

        impl ::wasm_bindgen::JsCast for #ident {
            fn instanceof(val: &::wasm_bindgen::JsValue) -> bool {
                <#inner_type as ::wasm_bindgen::JsCast>::instanceof(val)
            }

            fn unchecked_from_js(val: ::wasm_bindgen::JsValue) -> Self {
                Self(<#inner_type as ::wasm_bindgen::JsCast>::unchecked_from_js(val))
            }

            fn unchecked_from_js_ref(val: &::wasm_bindgen::JsValue) -> &Self {
                // SAFETY:
                // #ident is a NewType around the #inner_type and thus at runtime is equivalent,
                // this allows us to return a reference from the result of the internal call.
                unsafe { ::std::mem::transmute(<#inner_type as ::wasm_bindgen::JsCast>::unchecked_from_js_ref(val)) }
            }
        }

        #[allow(dead_code, non_camel_case_types)]
        #[doc(hidden)]
        #vis type #event_ident = #ident;

        impl ::yew::StaticEvent for #ident {
            type Event = Self;

            fn event_name() -> &'static str {
                #event_name
            }
        }
    }
}
