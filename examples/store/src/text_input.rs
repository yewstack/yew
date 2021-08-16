use weblog::web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Submit(String),
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
            Msg::Submit(text) => {
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
        let onkeydown = self.link.batch_callback(|e: KeyboardEvent| {
            e.stop_propagation();
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Submit(value))
            } else {
                None
            }
        });

        html! {
            <input
                placeholder={self.props.value.clone()}
                type="text"
                {onkeydown}
            />
        }
    }
}
