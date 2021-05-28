use crate::router::use_router;
use crate::{Routable, RouterAction};
use yew::prelude::*;

use yew_functional::function_component;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R: Routable> {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub route: R,
    pub children: Children,
}

#[function_component(Link)]
pub fn link<R>(props: &LinkProps<R>) -> Html
where
    R: Routable,
{
    let router = use_router::<R>();
    let route = props.route.clone();
    let callback = router.dispatcher(move |_| Some(RouterAction::Push(route.clone())));
    html! {
        <a class=props.classes.clone() onclick=callback>{ props.children.clone() }</a>
    }
}
