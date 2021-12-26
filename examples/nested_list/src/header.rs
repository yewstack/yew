use super::list::{List, Msg as ListMsg};
use super::{Hovered, WeakComponentLink};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_hover: Callback<Hovered>,
    pub text: String,
    pub list_link: WeakComponentLink<List>,
}

pub struct ListHeader;

impl Component for ListHeader {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list_link = ctx.props().list_link.borrow().clone().unwrap();
        let onmouseover = {
            let on_hover = ctx.props().on_hover.clone();
            move |e: MouseEvent| {
                e.stop_propagation();
                on_hover.emit(Hovered::Header)
            }
        };

        html! {
            <div
                class="list-header"
                {onmouseover}
                onclick={list_link.callback(|_| ListMsg::HeaderClick)}
            >
                { &ctx.props().text }
            </div>
        }
    }
}
