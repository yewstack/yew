use yew::prelude::*;

pub enum Msg {
    Hover,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_hover: Callback<()>,
}

pub struct InputComponent;

impl Component for InputComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hover => {
                ctx.props().on_hover.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                type="text"
                class="input-component"
                onmouseover={ctx.link().callback(|_| Msg::Hover)}
            />
        }
    }
}
