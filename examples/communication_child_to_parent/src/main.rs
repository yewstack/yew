use yew::{html, AttrValue, Callback, Component, Context, Html, Properties};

use child::Child;
use parent::Parent;

mod child;
mod parent;

pub enum Msg {
    ButtonClick(AttrValue),
}

fn main() {
    yew::Renderer::<Parent>::new().render();
}
