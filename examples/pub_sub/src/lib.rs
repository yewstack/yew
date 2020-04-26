#![recursion_limit = "128"]

mod event_bus;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Producer />
                <Subscriber />
            </div>
        }
    }
}
