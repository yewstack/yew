use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub value: String,
    pub oninput: Callback<String>,
}

pub struct TextInput {
    value: String,
    link: ComponentLink<Self>,
    oninput: Callback<String>,
}

pub enum Msg {
    Changed(String),
}

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextInput {
            value: props.value,
            oninput: props.oninput,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed(value) => {
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
                oninput=self.link.callback(|e: InputData| Msg::Changed(e.value))
            />
        }
    }
}
