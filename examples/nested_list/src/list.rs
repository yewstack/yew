use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::header::ListHeader;
use crate::item::ListItem;
use crate::{Hovered, WeakComponentLink};

pub enum Msg {
    HeaderClick,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub header: IArray<VChild<ListHeader>>,
    #[prop_or_default]
    pub children: IArray<VChild<ListItem>>,

    pub on_hover: Callback<Hovered>,
    pub weak_link: WeakComponentLink<List>,
}

pub struct List {
    inactive: bool,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .weak_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Self { inactive: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HeaderClick => {
                self.inactive = !self.inactive;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inactive = if self.inactive { "inactive" } else { "" };
        let onmouseover = ctx.props().on_hover.reform(|e: MouseEvent| {
            e.stop_propagation();
            Hovered::List
        });
        html! {
            <div class="list-container" {onmouseover}>
                <div class={classes!("list", inactive)}>
                    { &ctx.props().header }
                    <div class="items">
                        { Self::view_items(&ctx.props().children) }
                    </div>
                </div>
            </div>
        }
    }
}

impl List {
    fn view_items(children: &IArray<VChild<ListItem>>) -> Html {
        children
            .iter()
            .filter(|c| !c.props.hide)
            .enumerate()
            .map(|(i, mut c)| {
                let props = c.get_mut();
                props.name = format!("#{} - {}", i + 1, props.name).into();
                c
            })
            .collect::<Html>()
    }
}
