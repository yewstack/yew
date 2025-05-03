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
pub(crate) trait DisplayExt: Display {
    /// Equivalent to [`str::eq_ignore_ascii_case`], but works for anything that's `Display` without
    /// allocations
    fn eq_str_ignore_ascii_case(&self, other: &str) -> bool {
        /// Writer that only succeeds if all of the input is a __prefix__ of the contained string,
        /// ignoring ASCII case.
        ///
        /// It cannot verify that `other` is not longer than the input.
        struct X<'src> {
            other: &'src str,
        }

        impl Write for X<'_> {
            fn write_str(&mut self, self_chunk: &str) -> std::fmt::Result {
                if self.other.len() < self_chunk.len() {
                    return Err(std::fmt::Error); // `other` is shorter than `self`.
                }
                let other_chunk;
                // Chop off a chunk from `other` the size of `self_chunk` to compare them
                (other_chunk, self.other) = self.other.split_at(self_chunk.len());
                // Check if the chunks match
                self_chunk
                    .eq_ignore_ascii_case(other_chunk)
                    .then_some(())
                    .ok_or(std::fmt::Error)
            }
        }

        let mut writer = X { other };
        // The `is_ok_and` call ensures that there's nothing left over.
        // If the remainder of `other` is not empty, it means `other` is longer than
        // `self`.
        write!(writer, "{self}").is_ok_and(|_| writer.other.is_empty())
    }

    /// Equivalent of `s1.to_string() == s2` but without allocations
    fn eq_str(&self, other: &str) -> bool {
        /// Writer that only succeeds if all of the input is a __prefix__ of the contained string.
        ///
        /// It cannot verify that `other` is not longer than the input.
        struct X<'src> {
            other: &'src str,
        }

        impl Write for X<'_> {
            fn write_str(&mut self, chunk: &str) -> std::fmt::Result {
                self.other
                    .strip_prefix(chunk) // Try to chop off a chunk of `self` from `other`
                    .map(|rest| self.other = rest) // If it matched, reassign the rest of `other`
                    .ok_or(std::fmt::Error) // Otherwise, break out signifying a mismatch
            }
        }

        let mut writer = X { other };
        // The `is_ok_and` call ensures that there's nothing left over.
        // If the remainder of `other` is not empty, it means `other` is longer than
        // `self`.
        write!(writer, "{self}").is_ok_and(|_| writer.other.is_empty())
    }

    /// Equivalent of [`str::starts_with`], but works for anything that's `Display` without
    /// allocations
    fn starts_with(&self, prefix: &str) -> bool {
        /// Writer that only succeeds if `prefix` is a prefix of the input
        struct X<'src> {
            prefix: &'src str,
        }

        impl Write for X<'_> {
            fn write_str(&mut self, chunk: &str) -> std::fmt::Result {
                match self.prefix.strip_prefix(chunk) {
                    // Try to chop off a chunk from `prefix`
                    Some(rest) => self.prefix = rest, // Reassign the rest of `prefix` on success
                    None => {
                        // Check if `prefix` became shorter than the rest of input, but can still be
                        // found in the input
                        chunk.strip_prefix(self.prefix).ok_or(std::fmt::Error)?;
                        self.prefix = ""; // All of `prefix` was found, ignore the rest of input
                    }
                }

                Ok(())
            }
        }

        let mut writer = X { prefix };
        write!(writer, "{self}").is_ok()
    }

    /// Returns `true` if `s` only displays ASCII chars & doesn't start with a capital letter
    fn is_non_capitalized_ascii(&self) -> bool {
        /// Writer that succeeds only if the input is non-capitalised ASCII _or is empty_
        ///
        /// The case of empty input should be checked afterwards by the checking `self.empty`
        struct X {
            /// Whether there was any non-empty input
            empty: bool,
        }

        impl Write for X {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                if self.empty {
                    // Executed if there was no input before that
                    self.empty = s.is_empty();
                    // Inspecting the 1st char
                    if s.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                        // The 1st char is A-Z, the input is capitalised
                        return Err(std::fmt::Error);
                    }
                }

                // Check if everything is ASCII
                s.is_ascii().then_some(()).ok_or(std::fmt::Error)
            }
        }

        let mut writer = X { empty: true };
        // The `is_ok_and` call ensures that empty input is _NOT_ considered non-capitalised ASCII
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

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter, Write};

    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};

    use crate::DisplayExt;

    const N_ITERS: usize = 0x4000;
    const STR_LEN: usize = 32;

    /// Implements `Display` by feeding the formatter 1 `char` at a time.
    ///
    /// Tests the ability of [`DisplayExt`] to handle disparate chunks of strings
    struct DisplayObfuscator<'a>(&'a str);

    impl Display for DisplayObfuscator<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for ch in self.0.chars() {
                f.write_char(ch)?;
            }
            Ok(())
        }
    }

    /// Does the same thing as [`DisplayObfuscator`] but also lowercases all chars.
    struct Lowercaser<'a>(&'a str);

    impl Display for Lowercaser<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for ch in self.0.chars() {
                f.write_char(ch.to_ascii_lowercase())?;
            }
            Ok(())
        }
    }

    #[test]
    fn display_ext_works() {
        let rng = &mut SmallRng::from_os_rng();
        let mut s = String::with_capacity(STR_LEN);

        for i in 0..N_ITERS {
            s.clear();
            // Generate `STR_LEN` ASCII chars
            s.extend(
                rng.random_iter::<u8>()
                    .take(STR_LEN)
                    .map(|b| (b & 127) as char),
            );

            assert!(Lowercaser(&s).eq_str_ignore_ascii_case(&s));
            assert!(DisplayObfuscator(&s).eq_str(&s));
            assert!(DisplayObfuscator(&s).starts_with(&s[..i % STR_LEN]));
            assert_eq!(
                DisplayObfuscator(&s).is_non_capitalized_ascii(),
                s.is_non_capitalized_ascii()
            );
        }
    }
}
