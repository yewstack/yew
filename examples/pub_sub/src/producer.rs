use crate::event_bus::{EventBus, Request};
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

pub enum Msg {
    Clicked,
}

pub struct Producer {
    event_bus: Dispatcher<EventBus>,
}

impl Component for Producer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            event_bus: EventBus::dispatcher(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.event_bus
                    .send(Request::EventBusMsg("Message received".to_owned()));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.link().callback(|_| Msg::Clicked)}>
                { "PRESS ME" }
            </button>
        }
    }
}
