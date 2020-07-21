use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Model;

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let url1 = Some("https://example.com");
        let url2: Option<&'static str> = None;
        let url3 = "https://example.com";

        let disabled1 = Some(true);
        let disabled2 = Some(false);
        let disabled3: Option<bool> = None;

        html! {
            <div>
                <a href?=url1 disabled?=disabled1 media?=Some("print")>{ "Some URL" }</a>
                <a href?=url2 disabled?=disabled2>{ "No URL"}</a>
                <a href=url3 disabled?=disabled3>{ "Definite URL"}</a>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
