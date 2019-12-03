#![recursion_limit = "512"]

mod app;
mod header;
mod item;
mod list;

pub use app::App;
use std::fmt;

#[derive(Debug)]
pub enum Hovered {
    Header,
    Item(String),
    List,
    None,
}

use yew::html::ComponentLink;
use std::rc::Rc;
use std::cell::RefCell;
pub type WeakComponentLink<COMP> = Rc<RefCell<Option<ComponentLink<COMP>>>>;

impl fmt::Display for Hovered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hovered::Header => "Header",
                Hovered::Item(name) => name,
                Hovered::List => "List container",
                Hovered::None => "Nothing",
            }
        )
    }
}
