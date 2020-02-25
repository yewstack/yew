use super::event_bus::EventBus;
use yew::agent::Bridged;
use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    NewMessage(String),
}

pub struct Subsciber {
    message: String,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Subsciber {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|s| Msg::NewMessage(s));
        let _producer = EventBus::bridge(callback);
        Subsciber {
            message: format!("No message yet"),
            _producer,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewMessage(s) => self.message = s,
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <h1>{self.message.clone()}</h1>
        }
    }
}
