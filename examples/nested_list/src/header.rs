use crate::list::Hovered;
use yew::prelude::*;

pub struct ListHeader {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub on_hover: Callback<Hovered>,
    #[props(required)]
    pub text: String,
}

pub enum Msg {
    Hover,
}

impl Component for ListHeader {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListHeader { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.props.on_hover.emit(Hovered::Header);
            }
        }
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="list-header" onmouseover=|_| Msg::Hover>
                { &self.props.text }
            </div>
        }
    }
}
