use yew::prelude::*;

use crate::text_input::TextInput;

pub struct Form {
    first_name: String,
    last_name: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SetFirstName(String),
    SetLastName(String),
    ResetForm,
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Form {
            first_name: "".to_string(),
            last_name: "".to_string(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetFirstName(new_value) => {
                self.first_name = new_value;
            }
            Msg::SetLastName(new_value) => {
                self.last_name = new_value;
            }
            Msg::ResetForm => {
                self.first_name = "".to_string();
                self.last_name = "".to_string();
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
                <div>
                    <TextInput
                        value=&self.first_name
                        oninput=self.link.callback(|val: String| Msg::SetFirstName(val))
                    />
                    <TextInput
                        value=&self.last_name
                        oninput=self.link.callback(|val: String| Msg::SetLastName(val))
                    />
                    <button onclick=self.link.callback(|_| Msg::ResetForm)>{ "reset form" }</button>
                </div>
                <div>
                    <div>{ "First name:" }{&self.first_name}</div>
                    <div>{ "Last name:" }{&self.last_name}</div>
                </div>
            </div>
        }
    }
}
