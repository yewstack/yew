use std::marker::PhantomData;

use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::navigator::NavigatorKind;
use crate::scope_ext::RouterScopeExt;
use crate::utils;
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
                let navigator = ctx
                    .link()
                    .navigator()
                    .expect_throw("failed to get navigator");
                match query {
                    None => {
                        navigator.push(to.clone());
                    }
                    Some(data) => {
                        navigator
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
            query,
            children,
            disabled,
            ..
        } = ctx.props().clone();
        let onclick = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::OnClick
        });

        let navigator = ctx
            .link()
            .navigator()
            .expect_throw("failed to get navigator");
        let href: AttrValue = {
            let pathname = navigator.route_to_url(to);
            let path = query
                .and_then(|query| serde_urlencoded::to_string(query).ok())
                .and_then(|query| utils::compose_path(&pathname, &query))
                .unwrap_or_else(|| pathname.to_string());

            match navigator.kind() {
                NavigatorKind::Hash => format!("#{}", path),
                _ => path,
            }
            .into()
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
