pub mod html_tree;

use syn::parse::ParseStream;

pub trait Peek: Sized {
    fn peek(input: &ParseStream) -> bool;
}
