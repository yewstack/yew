use std::rc::Rc;

use child::Child;
use grandparent::GrandParent;
use parent::Parent;

mod child;
mod grandparent;
mod parent;

use yew::{
    function_component, html, AttrValue, Callback, Component, Context, ContextHandle,
    ContextProvider, Html, Properties,
};

/// This is the shared state between the parent and child components.
#[derive(Clone, PartialEq)]
pub struct AppState {
    /// Total number of clicks received.
    total_clicks: u32,
    /// Callback used when a child is clicked. The AttrValue is the name of the child that was
    /// clicked.
    child_clicked: Callback<AttrValue>,
    /// The name of the child that was last clicked.
    last_clicked: Option<AttrValue>,
}

fn main() {
    yew::Renderer::<GrandParent>::new().render();
}
