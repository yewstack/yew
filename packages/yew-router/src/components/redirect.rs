use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::hooks::use_navigator;
use crate::Routable;

/// Props for [`Redirect`]
#[derive(Properties, Clone, PartialEq, Eq)]
pub struct RedirectProps<R: Routable> {
    /// Route that will be pushed when the component is rendered.
    pub to: R,
}

/// A component that will redirect to specified route when rendered.
#[function_component(Redirect)]
pub fn redirect<R>(props: &RedirectProps<R>) -> Html
where
    R: Routable + 'static,
{
    let history = use_navigator().expect_throw("failed to read history.");

    let target_route = props.to.clone();
    use_effect(move || {
        history.push(&target_route);

        || {}
    });

    Html::default()
}
