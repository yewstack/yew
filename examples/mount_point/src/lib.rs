#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    name: String,
}

pub enum Msg {
    UpdateName(String),
}

impl<CTX> Component<CTX> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <input value=&self.name, oninput=|e| Msg::UpdateName(e.value), />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
    }
}
