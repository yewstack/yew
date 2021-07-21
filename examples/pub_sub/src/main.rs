mod event_bus;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::{html, Component, Html, Context};

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
