use crate::event_bus::EventBus;
use yew::agent::Dispatched;
use yew::prelude::*;

use crate::event_bus::Request;
use yew::agent::Dispatcher;

pub struct Producer {
    link: ComponentLink<Producer>,
    event_bus: Dispatcher<EventBus>,
}

pub enum Msg {
    Clicked,
}

impl Component for Producer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = EventBus::dispatcher();

        Producer { event_bus, link }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.event_bus
                    .send(Request::EventBusMsg("Message received".to_string()));
                false
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {"PUSH ME"}
            </button>
        }
    }
}
