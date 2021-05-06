use super::event_bus::EventBus;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_agent::{Bridge, Bridged};

pub enum Msg {
    NewMessage(String),
}

pub struct Subscriber {
    message: String,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Subscriber {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            message: "No message yet.".to_owned(),
            _producer: EventBus::bridge(link.callback(Msg::NewMessage)),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewMessage(s) => {
                self.message = s;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <h1>{ &self.message }</h1>
        }
    }
}
