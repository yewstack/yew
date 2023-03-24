use std::rc::Rc;
use grandparent::GrandParent;
use parent::Parent;
use child::Child;

mod grandparent;
mod parent;
mod child;

use yew::{function_component, html, Component, Context, ContextHandle, ContextProvider, Html};

/// This is the shared state between the parent and child components.
#[derive(Clone, Eq, PartialEq)]
pub struct AppState {
    /// The total number of clicks received.
    total_clicks: u32,
}

fn main() {
    yew::Renderer::<GrandParent>::new().render();
}
