pub mod html_list;

use crate::Peek;
use html_list::HtmlList;
use syn::parse::{Parse, ParseStream, Result};

pub enum HtmlTree {
    List(HtmlList),
    Empty,
}

impl Parse for HtmlTree {
    fn parse(input: ParseStream) -> Result<Self> {
        if HtmlList::peek(&input) {
            return Ok(HtmlTree::List(input.parse()?));
        }

        if input.is_empty() {
            Ok(HtmlTree::Empty)
        } else {
            Err(input.error("expected valid html element"))
        }
    }
}
