mod event_bus;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, _msg: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Producer />
                <Subscriber />
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
