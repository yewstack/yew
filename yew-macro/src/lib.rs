//! This crate provides Yew's procedural macro `html!` which allows using JSX-like syntax
//! for generating html and the `Properties` derive macro for deriving the `Properties` trait
//! for components.
//!
//! ```
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

mod classes;
mod derive_props;
mod html_tree;
mod props;
mod stringify;

use derive_props::DerivePropsInput;
use html_tree::{HtmlRoot, HtmlRootVNode};
use proc_macro::TokenStream;
use quote::ToTokens;
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

#[proc_macro_derive(Properties, attributes(prop_or, prop_or_else, prop_or_default))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DerivePropsInput);
    TokenStream::from(input.into_token_stream())
}

#[proc_macro]
pub fn html_nested(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRoot);
    TokenStream::from(root.into_token_stream())
}

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
