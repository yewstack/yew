use yew::{html, AttrValue, Callback, Component, Context, Html, Properties};

use parent::Parent;
use child::Child;

mod child;
mod parent;

pub enum Msg {
    ButtonClick(AttrValue),
}

fn main() {
    yew::Renderer::<Parent>::new().render();
}
