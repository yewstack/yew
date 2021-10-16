use crate::{service, Routable};
use std::marker::PhantomData;
use yew::prelude::*;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R: Routable + Clone + PartialEq> {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub to: R,
    pub children: Children,
}

/// A wrapper around `<a>` tag to be used with [`Router`](crate::Router)
pub struct Link<R: Routable + Clone + PartialEq + 'static> {
    _data: PhantomData<R>,
}

pub enum Msg {
    OnClick,
}

impl<R: Routable + Clone + PartialEq + 'static> Component for Link<R> {
    type Message = Msg;
    type Properties = LinkProps<R>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { _data: PhantomData }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                service::push_route(ctx.props().to.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <a class={ctx.props().classes.clone()}
                href={ctx.props().to.to_path()}
                onclick={ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    Msg::OnClick
                })}
            >
                { ctx.props().children.clone() }
            </a>
        }
    }
}
