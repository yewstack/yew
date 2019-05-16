pub mod html_list;

use crate::Peek;
use html_list::HtmlList;
use syn::parse::{Parse, ParseStream, Result};

pub enum HtmlTree {
    List(HtmlList),
    Empty,
}

pub struct HtmlRoot(HtmlTree);
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let html_tree = input.parse::<HtmlTree>()?;
        if !input.is_empty() {
            Err(input.error("only one root html element allowed"))
        } else {
            Ok(HtmlRoot(html_tree))
        }
    }
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        if HtmlList::peek(&input) {
            Ok(HtmlTree::List(input.parse()?))
        } else if input.is_empty() {
            Ok(HtmlTree::Empty)
        } else {
            Err(input.error("expected valid html element"))
        }
    }
}
