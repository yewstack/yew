use crate::Hovered;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
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
            let name = ctx.props.name.clone();
            ctx.props.on_hover.reform(move |e: MouseEvent| {
                e.stop_propagation();
                Hovered::Item(name.clone())
            })
        };
        html! {
            <div class="list-item" onmouseover=onmouseover>
                { ctx.props.name.clone() }
                { self.view_details(ctx) }
            </div>
        }
    }
}

impl ListItem {
    fn view_details(&self, ctx: &Context<Self>) -> Html {
        if ctx.props.children.is_empty() {
            html! {}
        } else {
            html! {
                <div class="list-item-details">
                    { ctx.props.children.clone() }
                </div>
            }
        }
    }
}
