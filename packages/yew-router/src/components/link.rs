use gloo::utils::window;
use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
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
    pub target: Option<String>,
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
        target,
        disabled,
        query,
    } = props.clone();

    let navigator = use_navigator().expect_throw("failed to get navigator");

    let onclick = {
        let navigator = navigator.clone();
        let to = to.clone();
        let query = query.clone();
        let target = target.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            match target {
                Some(ref target) => {
                    let window = window();
                    let href = window
                        .location()
                        .href()
                        .expect_throw("Failed to read location href");
                    match query {
                        None => {
                            window
                                .open_with_url_and_target(&to.to_path(), target)
                                .unwrap();
                            ()
                        }
                        Some(ref data) => {
                            let route: &str = &to.to_path();
                            let query: &str = &serde_urlencoded::to_string(data)
                                .expect_throw("unable to encode query");

                            let url = Url::new_with_base(route, &href)
                                .expect_throw("current url is not valid.");
                            url.set_search(query);

                            window
                                .open_with_url_and_target(&url.href(), target)
                                .unwrap();
                        }
                    }
                }
                None => match query {
                    None => {
                        navigator.push(&to);
                    }
                    Some(ref data) => {
                        navigator
                            .push_with_query(&to, data)
                            .expect_throw("failed push history with query");
                    }
                },
            };
        })
    };

    let href = {
        let route_s = to.to_path();
        let pathname = navigator.prefix_basename(&route_s);
        let mut path = query
            .and_then(|query| serde_urlencoded::to_string(query).ok())
            .and_then(|query| utils::compose_path(&pathname, &query))
            .unwrap_or_else(|| pathname.into_owned());

        if navigator.kind() == NavigatorKind::Hash {
            path.insert(0, '#');
        }

        AttrValue::from(path)
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
