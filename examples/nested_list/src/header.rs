use super::list::{List, Msg as ListMsg};
use super::{Hovered, WeakComponentLink};
use yew::prelude::*;

pub struct ListHeader {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_hover: Callback<Hovered>,
    pub text: String,
    pub list_link: WeakComponentLink<List>,
}

impl Component for ListHeader {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListHeader { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let list_link = self.props.list_link.borrow().clone().unwrap();
        let onclick = list_link.callback(|_| ListMsg::HeaderClick);
        let onmouseover = self.props.on_hover.reform(|e: MouseEvent| {
            e.stop_propagation();
            Hovered::Header
        });
        html! {
            <div class="list-header" onmouseover=onmouseover onclick=onclick>
                { &self.props.text }
            </div>
        }
    }
}
