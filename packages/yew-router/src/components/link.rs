use std::borrow::Cow;
use std::marker::PhantomData;

use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::history::{BrowserHistory, History};
use crate::scope_ext::RouterScopeExt;
use crate::Routable;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R, Q = ()>
where
    R: Routable,
    Q: Clone + PartialEq + Serialize,
{
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub to: R,
    /// Route query data
    #[prop_or_default]
    pub query: Option<Q>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

/// A wrapper around `<a>` tag to be used with [`Router`](crate::Router)
pub struct Link<R, Q = ()>
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
{
    _route: PhantomData<R>,
    _query: PhantomData<Q>,
}

pub enum Msg {
    OnClick,
}

impl<R, Q> Component for Link<R, Q>
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
{
    type Message = Msg;
    type Properties = LinkProps<R, Q>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _route: PhantomData,
            _query: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                let LinkProps { to, query, .. } = ctx.props();
                let history = ctx.link().history().expect_throw("failed to read history");
                match query {
                    None => {
                        history.push(to.clone());
                    }
                    Some(data) => {
                        history
                            .push_with_query(to.clone(), data.clone())
                            .expect_throw("failed push history with query");
                    }
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let LinkProps {
            classes,
            to,
            children,
            disabled,
            ..
        } = ctx.props().clone();
        let onclick = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::OnClick
        });
        let href: AttrValue = match BrowserHistory::route_to_url(to) {
            Cow::Owned(href) => href.into(),
            Cow::Borrowed(href) => href.into(),
        };
        html! {
            <a class={classes}
                {href}
                {onclick}
                {disabled}
            >
                { children }
            </a>
        }
    }
}
