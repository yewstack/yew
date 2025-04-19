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

use std::fmt::{Display, Write};

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

/// Extension methods for treating `Display`able values like strings, without allocating the
/// strings.
///
/// Needed to check the plentiful token-like values in the impl of the macros, which are
/// `Display`able but which either correspond to multiple source code tokens, or are themselves
/// tokens that don't provide a reference to their repr.
trait DisplayExt: Display {
    /// Equivalent to [`str::eq_ignore_ascii_case`], but works for anything that's `Display` without
    /// allocations
    fn repr_eq_ignore_ascii_case(&self, other: &str) -> bool {
        /// Writer that only succeeds if all of the input is a prefix of the contained string.
        struct X<'src>(&'src str);

        impl Write for X<'_> {
            fn write_str(&mut self, chunk: &str) -> std::fmt::Result {
                if !self
                    .0
                    .get(..chunk.len())
                    .is_some_and(|x| x.eq_ignore_ascii_case(chunk))
                {
                    return Err(std::fmt::Error);
                }
                self.0 = self.0.split_at(chunk.len()).1;
                Ok(())
            }
        }

        // The `is_ok_and` call ensures that there's nothing left over, ensuring
        // `s1.to_string().eq_ignore_ascii_case(s2)`
        // without ever allocating `s1`
        let mut writer = X(other);
        write!(writer, "{self}").is_ok_and(|_| writer.0.is_empty())
    }

    /// Equivalent of `s1.to_string() == s2` but without allocations
    fn repr_eq(&self, other: &str) -> bool {
        /// Writer that only succeeds if all of the input is a prefix of the contained string.
        struct X<'src>(&'src str);

        impl Write for X<'_> {
            fn write_str(&mut self, chunk: &str) -> std::fmt::Result {
                self.0
                    .strip_prefix(chunk)
                    .map(|rest| self.0 = rest)
                    .ok_or(std::fmt::Error)
            }
        }

        // The `is_ok_and` call ensures that there's nothing left over, ensuring `s1.to_string() ==
        // s2` without ever allocating `s1`
        let mut writer = X(other);
        write!(writer, "{self}").is_ok_and(|_| writer.0.is_empty())
    }

    /// Equivalent of [`str::starts_with`], but works for anything that's `Display` without
    /// allocations
    fn starts_with(&self, prefix: &str) -> bool {
        /// Writer that only succeeds if all of the input is a prefix of the contained string.
        struct X<'src>(&'src str);

        impl Write for X<'_> {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                match self.0.strip_prefix(s) {
                    Some(rest) => self.0 = rest,
                    None if self.0.len() < s.len() => {
                        s.strip_prefix(self.0).ok_or(std::fmt::Error)?;
                        self.0 = "";
                    }
                    None => return Err(std::fmt::Error),
                }

                Ok(())
            }
        }

        let mut writer = X(prefix);
        write!(writer, "{self}").is_ok()
    }

    /// Returns `true` if `s` only displays ASCII chars & doesn't start with a capital letter
    fn is_non_capitalized_ascii(&self) -> bool {
        /// Writer that succeeds only if the input is non-capitalised ASCII
        struct X {
            empty: bool,
        }

        impl Write for X {
            fn write_str(&mut self, mut s: &str) -> std::fmt::Result {
                if self.empty {
                    self.empty = s.is_empty();
                    let mut iter = s.chars();
                    if iter.next().is_some_and(|c| c.is_ascii_uppercase()) {
                        return Err(std::fmt::Error);
                    }
                    s = iter.as_str();
                }

                s.is_ascii().then_some(()).ok_or(std::fmt::Error)
            }
        }

        let mut writer = X { empty: true };
        write!(writer, "{self}").is_ok_and(|_| !writer.empty)
    }
}

impl<T: Display> DisplayExt for T {}

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
    let mut input = parse_macro_input!(input as DerivePropsInput);
    input.normalise();
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
pub fn function_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as FunctionComponent);
    let attr = parse_macro_input!(attr as FunctionComponentName);

    function_component_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn hook(attr: TokenStream, item: TokenStream) -> TokenStream {
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
