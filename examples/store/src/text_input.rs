use yew::prelude::*;

pub enum Msg {
    SetText(String),
    Submit,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
    pub onsubmit: Callback<String>,
}

pub struct TextInput {
    link: ComponentLink<Self>,
    text: String,
    props: Props,
}

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: props.value.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetText(text) => {
                self.text = text;
                true
            }
            Msg::Submit => {
                let text = std::mem::replace(&mut self.text, self.props.value.clone());
                self.props.onsubmit.emit(text);
                true
            }
        }
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
                value={self.text.clone()}
                oninput={self.link.callback(|e: InputData| Msg::SetText(e.value))}
                onkeydown={self.link.batch_callback(move |e: KeyboardEvent| {
                    e.stop_propagation();
                    if e.key() == "Enter" { Some(Msg::Submit) } else { None }
                })}
            />
        }
    }
}
