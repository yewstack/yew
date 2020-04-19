#![recursion_limit = "512"]

mod app;
mod header;
mod item;
mod list;

pub use app::App;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use yew::html::ComponentLink;
pub type WeakComponentLink<COMP> = Rc<RefCell<Option<ComponentLink<COMP>>>>;

#[derive(Debug)]
pub enum Hovered {
    Header,
    Item(String),
    List,
    None,
}

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
