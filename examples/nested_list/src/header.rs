use crate::Hovered;
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

impl Component for ListHeader {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListHeader { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onmouseover = self.props.on_hover.reform(|_| Hovered::Header);
        html! {
            <div class="list-header" onmouseover=onmouseover>
                { &self.props.text }
            </div>
        }
    }
}
