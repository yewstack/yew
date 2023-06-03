use yew::prelude::*;

use crate::Hovered;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub hide: bool,
    pub on_hover: Callback<Hovered>,
    pub name: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct ListItem;

impl Component for ListItem {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouseover = {
            let name = ctx.props().name.clone();
            ctx.props().on_hover.reform(move |e: MouseEvent| {
                e.stop_propagation();
                Hovered::Item(name.clone())
            })
        };
        html! {
            <div class="list-item" {onmouseover}>
                { &ctx.props().name }
                { Self::view_details(&ctx.props().children) }
            </div>
        }
    }
}

impl ListItem {
    fn view_details(children: &Children) -> Html {
        if children.is_empty() {
            html! {}
        } else {
            html! {
                <div class="list-item-details">
                    { children.clone() }
                </div>
            }
        }
    }
}
