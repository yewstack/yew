//! This crate provides Yew's procedural macro `html!` which allows using JSX-like syntax
//! for generating html and the `Properties` derive macro for deriving the `Properties` trait
//! for components.
//!
//! The `html!` macro uses [proc_macro_hack](https://github.com/dtolnay/proc-macro-hack) in order
//! to be used in the expression position.
//!
//! ```
//! # #[macro_use] extern crate yew;
//! use yew::prelude::*;
//!
//! struct Component {
//!   link: ComponentLink<Self>,
//! }
//!
//! #[derive(Clone, Properties)]
//! struct Props {
//!     prop: String,
//! };
//!
//! # enum Msg { Submit }
//! #
//! # impl yew::Component for Component {
//! #     type Message = Msg;
//! #     type Properties = Props;
//! #     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn view(&self) -> Html {
//! #
//! // ...
//!
//! html! {
//!   <div>
//!     <button onclick=self.link.callback(|_| Msg::Submit)>
//!       { "Submit" }
//!     </button>
//!     <>
//!       <Component prop="first" />
//!       <Component prop="second" />
//!     </>
//!   </div>
//! }
//! #
//! #     }
//! # }
//! #
//! # fn main() {}
//! ```
//!
//! Please refer to [https://github.com/yewstack/yew](https://github.com/yewstack/yew) for how to set this up.

#![recursion_limit = "128"]
extern crate proc_macro;

mod derive_props;
mod html_tree;

use derive_props::DerivePropsInput;
use html_tree::{HtmlRoot, HtmlRootVNode};
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_hack::proc_macro_hack;
use quote::{quote, ToTokens};
use std::io::Read;
use syn::buffer::Cursor;
use syn::parse_macro_input;

trait Peek<'a, T> {
    fn peek(cursor: Cursor<'a>) -> Option<(T, Cursor<'a>)>;
}

trait PeekValue<T> {
    fn peek(cursor: Cursor) -> Option<T>;
}

fn non_capitalized_ascii(string: &str) -> bool {
    if !string.is_ascii() {
        false
    } else if let Some(c) = string.bytes().next() {
        c.is_ascii_lowercase()
    } else {
        false
    }
}

#[proc_macro_derive(Properties, attributes(prop_or, prop_or_else, prop_or_default))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DerivePropsInput);
    TokenStream::from(input.into_token_stream())
}

#[proc_macro_hack]
pub fn html_nested(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRoot);
    TokenStream::from(quote! {#root})
}

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRootVNode);
    TokenStream::from(quote! {#root})
}

#[proc_macro_hack]
pub fn include_html(input: TokenStream) -> TokenStream {
    return match parse_macro_input!(input as syn::Lit) {
        syn::Lit::Str(path) => {
            let mut file = match std::fs::File::open(path.value()) {
                Ok(t) => t,
                Err(_) => {
                    return syn::Error::new_spanned(
                        path,
                        "Couldn't open the supplied file. Are you sure it exists?",
                    )
                    .to_compile_error()
                    .into();
                }
            };
            let mut code = String::new();
            match file.read_to_string(&mut code) {
                Ok(_) => {}
                Err(_) => {
                    return syn::Error::new_spanned(path, "Couldn't read the supplied file.")
                        .to_compile_error()
                        .into();
                }
            };
            let span = Span::call_site();
            let parsed_code = TokenStream2::from(html(code.parse::<TokenStream>().unwrap()));
            let result = quote::quote_spanned! {span=>
                #parsed_code
            };
            result.into()
        }
        _ => syn::Error::new(Span::call_site(), "Expected a string literal.")
            .to_compile_error()
            .into(),
    };
}
