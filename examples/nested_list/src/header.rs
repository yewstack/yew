use super::Hovered;
use yew::prelude::*;

pub struct ListHeader {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ListHeader { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover => {
                self.props.on_hover.emit(Hovered::Header);
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="list-header" onmouseover=self.link.send_back(|_| Msg::Hover)>
                { &self.props.text }
            </div>
        }
    }
}
