#![recursion_limit = "1024"]

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew::prelude::*;

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

#[derive(Properties, Clone)]
pub struct TextInputProps {
    pub value: String,
    pub oninput: Callback<String>,
}

pub struct TextInput {
    value: String,
    link: ComponentLink<Self>,
    oninput: Callback<String>,
}

pub enum TextInputMsg {
    Changed(String),
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextInput {
            value: props.value,
            oninput: props.oninput,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextInputMsg::Changed(value) => {
                self.oninput.emit(value);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        self.oninput = props.oninput;
        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                value=&self.value
                oninput=self.link.callback(|e: InputData| TextInputMsg::Changed(e.value))
            />
        }
    }
}
