extern crate proc_macro;

mod html_tree;

use html_tree::HtmlTree;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::ParseStream;
use syn::parse_macro_input;

trait Peek: Sized {
    fn peek(input: &ParseStream) -> bool;
}

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as HtmlTree);
    TokenStream::from(quote! { 42 })
}
