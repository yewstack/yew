use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
        }
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Model>();
}
