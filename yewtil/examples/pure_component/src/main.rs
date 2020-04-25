use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod button;
use crate::button::Button;

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                log::info!("got message");
                true
            }
        }
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Button callback=self.link.callback(|_| Msg::DoIt) text = "Click me!" />
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
