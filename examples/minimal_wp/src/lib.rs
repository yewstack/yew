use yew::{html, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*; 

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Click ( wasm-pack )" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
