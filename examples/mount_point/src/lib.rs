use yew::{html, Component, ComponentLink, Html, ShouldRender};

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

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input value=&self.name oninput=|e| Msg::UpdateName(e.value) />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
    }
}
