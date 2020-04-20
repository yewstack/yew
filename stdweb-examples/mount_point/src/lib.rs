use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    name: String,
}

pub enum Msg {
    UpdateName(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    value=&self.name
                    oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value)) />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
    }
}
