use child::Child;
use parent::Parent;
use yew::{AttrValue, Callback, Component, Context, Html, Properties, html};

mod child;
mod parent;

fn main() {
    yew::Renderer::<Parent>::new().render();
}
