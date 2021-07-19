use yew::prelude::*;

pub enum Msg {
    Hover,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub on_hover: Callback<()>,
}

pub struct InputComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for InputComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.props.on_hover.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="text"
                class="input-component"
                onmouseover={self.link.callback(|_| Msg::Hover)}
            />
        }
    }
}
