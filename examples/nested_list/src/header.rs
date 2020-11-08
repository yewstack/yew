use super::list::{List, Msg as ListMsg};
use super::{Hovered, WeakComponentLink};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_hover: Callback<Hovered>,
    pub text: String,
    pub list_link: WeakComponentLink<List>,
}

pub struct ListHeader {
    props: Props,
}

impl Component for ListHeader {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        let list_link = self.props.list_link.borrow().clone().unwrap();
        let onmouseover = self.props.on_hover.reform(|e: MouseEvent| {
            e.stop_propagation();
            Hovered::Header
        });

        html! {
            <div
                class="list-header"
                onmouseover=onmouseover
                onclick=list_link.callback(|_| ListMsg::HeaderClick)
            >
                { &self.props.text }
            </div>
        }
    }
}
