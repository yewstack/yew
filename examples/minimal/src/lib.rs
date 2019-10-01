use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::Click>{ "Click" }</button>
            </div>
        }
    }
}
