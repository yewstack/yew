#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew::virtual_dom::VNode;
use yew_router::{route::Route, service::RouteService, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
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
            let route = app_route.clone();
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
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
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
