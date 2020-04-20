#![recursion_limit = "256"]
use yew::prelude::*;

use yew::virtual_dom::VNode;
use yew_router::{route::Route, service::RouteService, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::initialize();
    web_logger::init();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

pub struct Model {
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/a/{anything}"]
    A(String),
    #[to = "/b/{anything}/{number}"]
    B { anything: String, number: u32 },
    #[to = "/c"]
    C,
}

impl Model {
    fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
        self.link.callback(move |_| {
            let route = app_route.clone(); // TODO, I don't think I should have to clone here?
            Msg::ChangeRoute(route)
        })
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);

        Model {
            route_service,
            route,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
                // This might be derived in the future
                let route_string = match route {
                    AppRoute::A(s) => format!("/a/{}", s),
                    AppRoute::B { anything, number } => format!("/b/{}/{}", anything, number),
                    AppRoute::C => "/c".to_string(),
                };
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: (),
                };
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=&self.change_route(AppRoute::A("lorem".to_string())) > {"A"} </button>
                    <button onclick=&self.change_route(AppRoute::B{anything: "hello".to_string(), number: 42}) > {"B"} </button>
                    <button onclick=&self.change_route(AppRoute::C) > {"C"} </button>
                </nav>
                <div>
                {
                    match AppRoute::switch(self.route.clone()) {
                        Some(AppRoute::A(thing)) => VNode::from(thing.as_str()),
                        Some(AppRoute::B{anything, number}) => html!{<div> {anything} {number} </div>},
                        Some(AppRoute::C) => VNode::from("C"),
                        None => VNode::from("404")
                    }
                }
                </div>
            </div>
        }
    }
}
