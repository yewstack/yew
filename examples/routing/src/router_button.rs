//! A component wrapping a <button/> tag that changes the route.
use crate::router::Route;
use crate::router::Router;
use yew::agent::Dispatched;
use yew::prelude::*;

use crate::router::Request;
use yew::agent::Dispatcher;

/// Changes the route when clicked.
pub struct RouterButton {
    link: ComponentLink<Self>,
    router: Dispatcher<Router<()>>,
    props: Props,
}

pub enum Msg {
    Clicked,
}

/// Properties for Routing Components
#[derive(Properties, Default, Clone, Debug, PartialEq)]
pub struct Props {
    /// The route that will be set when the component is clicked.
    pub path: String,
    /// The text to display.
    pub text: String,
    /// Disable the component.
    pub disabled: bool,
    /// Classes to be added to component.
    pub classes: String,
}

impl Component for RouterButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = Router::dispatcher();

        RouterButton {
            link,
            router,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let route = Route {
                    path_segments: self
                        .props
                        .path
                        .split("/")
                        .skip(1)
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>(),
                    query: None,
                    state: (),
                    fragment: None,
                };
                self.router.send(Request::ChangeRoute(route));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <button
                class=self.props.classes.clone(),
                onclick=self.link.callback(|_| Msg::Clicked),
                disabled=self.props.disabled,
            >
                {&self.props.text}
            </button>
        }
    }
}
