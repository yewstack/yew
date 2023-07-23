use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::navigator::NavigatorKind;
use crate::prelude::*;
use crate::{utils, Routable};

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R, S = (), Q = ()>
where
    R: Routable,
    S: Clone + PartialEq,
    Q: Clone + PartialEq + Serialize,
{
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub to: R,
    /// Route state data
    #[prop_or_default]
    pub state: Option<S>,
    /// Route query data
    #[prop_or_default]
    pub query: Option<Q>,
    #[prop_or_default]
    pub disabled: bool,
    /// [`NodeRef`](yew::html::NodeRef) for the `<a>` element.
    #[prop_or_default]
    pub anchor_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

/// A wrapper around `<a>` tag to be used with [`Router`](crate::Router)
#[function_component]
pub fn Link<R, S = (), Q = ()>(props: &LinkProps<R, S, Q>) -> Html
where
    R: Routable + 'static,
    S: Clone + PartialEq + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
{
    let LinkProps {
        classes,
        to,
        children,
        disabled,
        state,
        query,
        anchor_ref,
    } = props.clone();

    let navigator = use_navigator().expect_throw("failed to get navigator");

    let onclick = {
        let navigator = navigator.clone();
        let to = to.clone();
        let state = state.clone();
        let query = query.clone();

        Callback::from(move |e: MouseEvent| {
            if e.meta_key() || e.ctrl_key() || e.shift_key() || e.alt_key() {
                return;
            }
            e.prevent_default();
            match (&state, &query) {
                (None, None) => {
                    navigator.push(&to);
                }
                (Some(state), None) => {
                    navigator.push_with_state(&to, state.clone());
                }
                (None, Some(query)) => {
                    navigator
                        .push_with_query(&to, query)
                        .expect_throw("failed push history with query");
                }
                (Some(state), Some(query)) => {
                    navigator
                        .push_with_query_and_state(&to, query, state.clone())
                        .expect_throw("failed push history with query and state");
                }
            }
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
            ref={anchor_ref}
        >
            { children }
        </a>
    }
}
