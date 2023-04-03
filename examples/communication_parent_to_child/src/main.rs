use child::Child;
use parent::Parent;
use yew::{html, Component, Context, Html, Properties};

mod child;
mod parent;

fn main() {
    yew::Renderer::<Parent>::new().render();
}
