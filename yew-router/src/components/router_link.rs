//! A component wrapping an `<a>` tag that changes the route.
use crate::{
    agent::{RouteAgentDispatcher, RouteRequest},
    components::{Msg, Props},
    route::Route,
    RouterState, Switch,
};
use std::marker::PhantomData;
use yew::prelude::*;
use yew::virtual_dom::VNode;

/// An anchor tag Component that when clicked, will navigate to the provided route.
///
/// Alias to RouterAnchor.
#[deprecated(note = "Has been renamed to RouterAnchor")]
pub type RouterLink<T> = RouterAnchor<T>;

/// An anchor tag Component that when clicked, will navigate to the provided route.
#[derive(Debug)]
pub struct RouterAnchor<SW: Switch, STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
    _marker: PhantomData<SW>,
}

impl<SW: Switch, STATE: RouterState> Component for RouterAnchor<SW, STATE> {
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(_ctx: &Context<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterAnchor {
            router,
            _marker: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let route = Route::from(ctx.props.route.clone());
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        #[cfg(feature = "std_web")]
        use stdweb::web::event::IEvent;

        let route: Route<STATE> = Route::from(ctx.props.route.clone());
        let target: &str = route.as_str();

        #[cfg(feature = "std_web")]
        let cb = ctx.callback(|event: ClickEvent| {
            event.prevent_default();
            Msg::Clicked
        });
        #[cfg(feature = "web_sys")]
        let cb = ctx.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Clicked
        });

        html! {
            <a
                class=ctx.props.classes.clone()
                onclick=cb
                disabled=ctx.props.disabled
                href=target
            >
                {
                    #[allow(deprecated)]
                    &ctx.props.text
                }
                {ctx.props.children.iter().collect::<VNode>()}
            </a>
        }
    }
}
