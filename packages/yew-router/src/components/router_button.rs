//! A component wrapping a `<button>` tag that changes the route.
use crate::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
    Switch,
};
use yew::prelude::*;

use super::{Msg, Props};
use crate::RouterState;
use yew::virtual_dom::VNode;

/// Changes the route when clicked.
#[derive(Debug)]
pub struct RouterButton<SW: Switch + Clone + 'static, STATE: RouterState = ()> {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    props: Props<SW>,
}

impl<SW: Switch + Clone + 'static, STATE: RouterState> Component for RouterButton<SW, STATE> {
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterButton {
            link,
            router,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let route = Route::from(self.props.route.clone());
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        let cb = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Clicked
        });
        html! {
            <button
                class=self.props.classes.clone()
                onclick=cb
                disabled=self.props.disabled
            >
                {
                    #[allow(deprecated)]
                    &self.props.text
                }
                {self.props.children.iter().collect::<VNode>()}
            </button>
        }
    }
}
