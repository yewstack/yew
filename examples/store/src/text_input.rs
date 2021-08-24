use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

pub enum Msg {
    Submit(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
    pub onsubmit: Callback<String>,
}

pub struct TextInput {
    text: String,
}

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: ctx.props().value.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(text) => {
                ctx.props().onsubmit.emit(text);
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> ShouldRender {
        self.text = ctx.props().value.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeydown = ctx.link().batch_callback(|e: KeyboardEvent| {
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
                placeholder={ctx.props().value.clone()}
                type="text"
                {onkeydown}
            />
        }
    }
}
