use yew::prelude::*;

pub enum Msg {
    SetText(String),
    Submit,
}

#[derive(Properties, PartialEq)]
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
            text: ctx.props.value.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetText(text) => {
                self.text = text;
                true
            }
            Msg::Submit => {
                let text = std::mem::replace(&mut self.text, ctx.props.value.clone());
                ctx.props.onsubmit.emit(text);
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, new_props: &Self::Properties) -> ShouldRender {
        if ctx.props.value != new_props.value {
            self.text = new_props.value.clone();
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                type="text"
                value=&self.text
                oninput=ctx.callback(|e: InputData| Msg::SetText(e.value))
                onkeydown=ctx.batch_callback(move |e: KeyboardEvent| {
                    e.stop_propagation();
                    if e.key() == "Enter" { Some(Msg::Submit) } else { None }
                })
            />
        }
    }
}
