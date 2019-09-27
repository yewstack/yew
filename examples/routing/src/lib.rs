#![recursion_limit = "128"]

mod b_component;
mod router;
mod router_button;
mod routing;
use b_component::BModel;

use crate::router_button::RouterButton;
use log::info;
use router::Route;
use yew::agent::Bridged;
use yew::{html, Bridge, Component, ComponentLink, Html, Renderable, ShouldRender};

pub enum Child {
    A,
    B,
    PathNotFound(String),
    Loading,
}

pub struct Model {
    child: Child,
    router: Box<dyn Bridge<router::Router<()>>>,
}

pub enum Msg {
    HandleRoute(Route<()>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let router = router::Router::bridge(callback);
        Model {
            child: Child::Loading, // This should be quickly overwritten by the actual route.
            router,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.router.send(router::Request::GetCurrentRoute);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                // Instead of each component selecting which parts of the path are important to it,
                // it is also possible to match on the `route.to_route_string().as_str()` once
                // and create enum variants representing the different children and pass them as props.
                self.child = if let Some(first_segment) = route.path_segments.get(0) {
                    match first_segment.as_str() {
                        "a" => Child::A,
                        "b" => Child::B,
                        other => Child::PathNotFound(other.into()),
                    }
                } else {
                    Child::PathNotFound("path_not_found".into())
                };

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu">
                    <RouterButton text="Go to A" path="/a" />
                    <RouterButton text="Go to B" path="/b" />
                </nav>
                <div>
                    {self.child.view()}
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for Child {
    fn view(&self) -> Html<Model> {
        match self {
            Child::A => html! {
                <>
                    {"This corresponds to route 'a'"}
                </>
            },
            Child::B => html! {
                <>
                    {"This corresponds to route 'b'"}
                    <BModel />
                </>
            },
            Child::PathNotFound(ref path) => html! {
                <>
                    {format!("Invalid path: '{}'", path)}
                </>
            },
            Child::Loading => html! {
                {"Loading"}
            },
        }
    }
}
