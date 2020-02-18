#![recursion_limit = "128"]

mod event_bus;
mod subscriber;
mod producer;

use subscriber::Subsciber;
use producer::Producer;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Producer />
                <Subsciber />
            </div>
        }
    }
}
