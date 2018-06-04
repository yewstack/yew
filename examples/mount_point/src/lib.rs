#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    name: String,
}

pub enum Msg {
    UpdateName(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input value=&self.name, oninput=|e| Msg::UpdateName(e.value), />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
    }
}
