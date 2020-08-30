use yew::prelude::*;

pub struct TextInput {
    link: ComponentLink<Self>,
    text: String,
    props: TextInputProperties,
}

pub enum TextInputMsg {
    SetText(String),
    Submit,
    None,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TextInputProperties {
    pub value: String,
    pub onsubmit: Callback<String>,
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextInput {
            link,
            text: props.value.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextInputMsg::SetText(text) => self.text = text,
            TextInputMsg::Submit => {
                let text = std::mem::replace(&mut self.text, self.props.value.clone());
                self.props.onsubmit.emit(text);
            }
            TextInputMsg::None => return false,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.text = self.props.value.clone();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="text"
                value=&self.text
                oninput=self.link.callback(|e: InputData| TextInputMsg::SetText(e.value))
                onkeydown=self.link.callback(move |e: KeyboardEvent| {
                    e.stop_propagation();
                    if e.key() == "Enter" { TextInputMsg::Submit } else { TextInputMsg::None }
                })
                />
        }
    }
}
