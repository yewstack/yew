use yew::prelude::*;

pub struct InputComponent {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.props.on_hover.emit(());
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="text"
                class="input-component"
                onmouseover=self.link.callback(|_| Msg::Hover) />
        }
    }
}
