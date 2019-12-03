use super::{Hovered, WeakComponentLink};
use super::list::{List, Msg as ListMsg};
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
    #[props(required)]
    pub list_link: WeakComponentLink<List>,
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
        let list_link = self.props.list_link.borrow().clone().unwrap();
        let onclick = list_link.callback(|_| ListMsg::HeaderClick);
        let onmouseover = self.link.callback(|_| Msg::Hover);
        html! {
            <div class="list-header" onmouseover=onmouseover onclick=onclick>
                { &self.props.text }
            </div>
        }
    }
}
