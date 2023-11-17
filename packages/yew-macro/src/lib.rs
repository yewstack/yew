#![cfg_attr(nightly_yew, feature(proc_macro_span))]

//! This crate provides Yew's procedural macro `html!` which allows using JSX-like syntax
//! for generating html and the `Properties` derive macro for deriving the `Properties` trait
//! for components.
//!
//! ```
//! use yew::prelude::*;
//!
//! struct Component;
//!
//! #[derive(Properties, PartialEq)]
//! struct Props {
//!     prop: String,
//! }
//!
//! # enum Msg { Submit }
//! #
//! # impl yew::Component for Component {
//! #     type Message = Msg;
//! #     type Properties = Props;
//! #     fn create(_ctx: &Context<Self>) -> Self {
//! #         unimplemented!()
//! #     }
//! #
//! #
//! #     fn view(&self, ctx: &Context<Self>) -> Html {
//! #
//! // ...
//!
//! html! {
//!   <div>
//!     <button onclick={ctx.link().callback(|_| Msg::Submit)}>
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

mod classes;
mod derive_props;
mod function_component;
mod hook;
mod html_tree;
mod props;
mod stringify;
mod use_prepared_state;
mod use_transitive_state;

use derive_props::DerivePropsInput;
use function_component::{function_component_impl, FunctionComponent, FunctionComponentName};
use hook::{hook_impl, HookFn};
use html_tree::{HtmlRoot, HtmlRootVNode};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::buffer::Cursor;
use syn::parse_macro_input;
use use_prepared_state::PreparedState;
use use_transitive_state::TransitiveState;

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

/// Combine multiple `syn` errors into a single one.
/// Returns `Result::Ok` if the given iterator is empty
fn join_errors(mut it: impl Iterator<Item = syn::Error>) -> syn::Result<()> {
    it.next().map_or(Ok(()), |mut err| {
        for other in it {
            err.combine(other);
        }
        Err(err)
    })
}

fn is_ide_completion() -> bool {
    match std::env::var_os("RUST_IDE_PROC_MACRO_COMPLETION_DUMMY_IDENTIFIER") {
        None => false,
        Some(dummy_identifier) => !dummy_identifier.is_empty(),
    }
}

#[proc_macro_derive(Properties, attributes(prop_or, prop_or_else, prop_or_default))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DerivePropsInput);
    TokenStream::from(input.into_token_stream())
}

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn html_nested(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRoot);
    TokenStream::from(root.into_token_stream())
}

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRootVNode);
    TokenStream::from(root.into_token_stream())
}

#[proc_macro]
pub fn props(input: TokenStream) -> TokenStream {
    let props = parse_macro_input!(input as props::PropsMacroInput);
    TokenStream::from(props.into_token_stream())
}

#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let classes = parse_macro_input!(input as classes::Classes);
    TokenStream::from(classes.into_token_stream())
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn function_component(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as FunctionComponent);
    let attr = parse_macro_input!(attr as FunctionComponentName);

    function_component_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn hook(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as HookFn);

    if let Some(m) = proc_macro2::TokenStream::from(attr).into_iter().next() {
        return syn::Error::new_spanned(m, "hook attribute does not accept any arguments")
            .into_compile_error()
            .into();
    }

    hook_impl(item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn use_prepared_state_with_closure(input: TokenStream) -> TokenStream {
    let prepared_state = parse_macro_input!(input as PreparedState);
    prepared_state.to_token_stream_with_closure().into()
}

#[proc_macro]
pub fn use_prepared_state_without_closure(input: TokenStream) -> TokenStream {
    let prepared_state = parse_macro_input!(input as PreparedState);
    prepared_state.to_token_stream_without_closure().into()
}

#[proc_macro]
pub fn use_transitive_state_with_closure(input: TokenStream) -> TokenStream {
    let transitive_state = parse_macro_input!(input as TransitiveState);
    transitive_state.to_token_stream_with_closure().into()
}

#[proc_macro]
pub fn use_transitive_state_without_closure(input: TokenStream) -> TokenStream {
    let transitive_state = parse_macro_input!(input as TransitiveState);
    transitive_state.to_token_stream_without_closure().into()
}
