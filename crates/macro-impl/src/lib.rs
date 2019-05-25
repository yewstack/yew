#![recursion_limit = "128"]
extern crate proc_macro;

mod html_tree;

use html_tree::HtmlRoot;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::buffer::Cursor;
use syn::parse_macro_input;

trait Peek<T> {
    fn peek(cursor: Cursor) -> Option<T>;
}

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRoot);
    TokenStream::from(quote! { #root })
}
