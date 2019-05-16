#![recursion_limit = "128"]

pub mod html_tree;

use syn::buffer::Cursor;

pub trait Peek {
    fn peek(cursor: Cursor) -> Option<()>;
}
