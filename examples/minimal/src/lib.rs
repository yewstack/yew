use yew::{html, Component, ComponentLink, Html, ShouldRender};
mod config;
pub use config::yew_config;

pub struct Model {
    link: ComponentLink<Self>,
    clicked: bool,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            clicked: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Click ( wasm-bindgen )" }</button>
                <p>{format!("Has been clicked: {}", self.clicked)}</p>
            </div>
        }
    }
}