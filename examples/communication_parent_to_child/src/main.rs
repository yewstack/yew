use yew::{html, Component, Context, Html, Properties};

use parent::Parent;
use child::Child;

mod parent;
mod child;

fn main() {
    yew::Renderer::<Parent>::new().render();
}
