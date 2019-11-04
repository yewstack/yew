use yew::prelude::*;

pub struct InputComponent {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub on_hover: Callback<()>,
}

pub enum Msg {
    Hover,
}

impl Component for InputComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        InputComponent { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.props.on_hover.emit(());
            }
        }
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <input class="input-component" type="text" onmouseover=|_| Msg::Hover />
        }
    }
}
