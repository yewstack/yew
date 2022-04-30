use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::navigator::NavigatorKind;
use crate::prelude::*;
use crate::{utils, Routable};

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
#[function_component]
pub fn Link<R, Q = ()>(props: &LinkProps<R, Q>) -> Html
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
{
    let LinkProps {
        classes,
        to,
        children,
        disabled,
        query,
    } = props.clone();

    let navigator = use_navigator().expect_throw("failed to get navigator");

    let onclick = {
        use_callback(
            |e: MouseEvent, (navigator, to, query)| {
                e.prevent_default();

                match query {
                    None => {
                        navigator.push(to.clone());
                    }
                    Some(ref data) => {
                        navigator
                            .push_with_query(to.clone(), data.clone())
                            .expect_throw("failed push history with query");
                    }
                }
            },
            (navigator.clone(), to.clone(), query.clone()),
        )
    };

    let href: AttrValue = {
        (*use_memo(
            |(to, query, navigator)| {
                let pathname = navigator.route_to_url(to.to_owned());
                let path = query
                    .clone()
                    .and_then(|query| serde_urlencoded::to_string(query).ok())
                    .and_then(|query| utils::compose_path(&pathname, &query))
                    .unwrap_or_else(|| pathname.to_string());

                AttrValue::from(match navigator.kind() {
                    NavigatorKind::Hash => format!("#{}", path),
                    _ => path,
                })
            },
            (to, query, navigator),
        ))
        .clone()
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
