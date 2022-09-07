use yew::prelude::*;

pub enum Msg {
    Hover,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_hover: Callback<()>,
    pub placeholder: AttrValue,
    pub input_ref: NodeRef,
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
        let placeholder = ctx.props().placeholder.clone();
        html! {
            <input
                ref={&ctx.props().input_ref}
                type="text"
                class="input-component"
                placeholder={placeholder}
                onmouseover={ctx.link().callback(|_| Msg::Hover)}
            />
        }
    }
}
