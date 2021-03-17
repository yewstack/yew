use gloo::events::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{Event, History};
use weblog::*;
use yew::prelude::*;
use yew_functional::*;

#[derive(Debug, Clone)]
pub struct CurrentRoute {
    path: String,
    params: route_recognizer::Params,
}

impl CurrentRoute {
    pub fn path(&self) -> &str {
        &self.path
    }

    // todo: use serde to deserialize params into a struct
    pub fn parmas(&self) -> &route_recognizer::Params {
        &self.params
    }
}

pub struct YewRouter {
    history: History,
    current_route: RefCell<Option<CurrentRoute>>,
}

impl YewRouter {
    fn new() -> Self {
        Self {
            history: yew::utils::window().history().expect("no history"),
            current_route: RefCell::new(None),
        }
    }

    pub fn push(&self, url: &str) {
        self.history
            .push_state_with_url(&JsValue::null(), "", Some(url))
            .expect("push history");
        let event = Event::new("__history_pushed").unwrap();
        yew::utils::window()
            .dispatch_event(&event)
            .expect("dispatch");
    }
}

thread_local! {
    pub static ROUTER: Rc<YewRouter> = Rc::new(YewRouter::new());
}

#[derive(Properties, Clone, PartialEq)]
pub struct RouterProps {
    pub children: ChildrenWithProps<Route>,
}

#[function_component(Router)]
pub fn router(props: &RouterProps) -> Html {
    let pathname = yew::utils::window().location().pathname().unwrap();
    let router = use_ref(|| {
        let mut router = route_recognizer::Router::new();
        props.children.iter().for_each(|child| {
            router.add(&child.props.to, child.props.to.to_string());
        });
        router
    });
    let (children, current_route) =
        from_route(&pathname, &props.children, &*router.borrow()).unwrap();

    let (force_rerender, set_force_rerender) = use_state(|| 0);

    ROUTER.with(|f| {
        console_log!("current_route", &format!("{:?}", current_route));
        *f.current_route.borrow_mut() = Some(current_route);
    });

    let _ = use_effect(move || {
        let event_listener1 = {
            let (force_rerender, set_force_rerender) =
                (Rc::clone(&force_rerender), Rc::clone(&set_force_rerender));
            EventListener::new(&yew::utils::window(), "popstate", move |_| {
                console_log!("forcing re render");
                set_force_rerender(*force_rerender + 1);
            })
        };

        let event_listener2 =
            EventListener::new(&yew::utils::window(), "__history_pushed", move |_| {
                console_log!("forcing re render");
                set_force_rerender(*force_rerender + 1);
            });

        move || {
            drop(event_listener1);
            drop(event_listener2);
        }
    });

    html! {
        { for children }
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct RouteProps {
    pub to: String,
    pub children: Children,
}

#[function_component(Route)]
pub fn route(props: &RouteProps) -> Html {
    html! {
        {for props.children.clone()}
    }
}

fn from_route(
    pathname: &str,
    routes: &ChildrenWithProps<Route>,
    router: &route_recognizer::Router<String>,
) -> Option<(Children, CurrentRoute)> {
    let mut selected = None;
    if let Ok(path) = router.recognize(pathname) {
        let children = routes
            .iter()
            .find(|it| it.props.to == **path.handler())
            .unwrap()
            .props
            .children;
        selected = Some((
            children,
            CurrentRoute {
                path: path.handler().to_string(),
                params: path.params().clone(),
            },
        ));
    }

    // TODO set `selected` 404 page if provided if this is None

    selected
}

#[derive(Debug, Copy, Clone)]
pub struct RouterService;

impl RouterService {
    pub fn push(url: &str) {
        ROUTER.with(|router| router.push(url))
    }

    pub fn current_route() -> CurrentRoute {
        ROUTER.with(|router| router.current_route.borrow().clone().unwrap())
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    pub classes: String,
    pub route: String,
    pub children: Children,
}

#[function_component(RouterAnchor)]
pub fn link(props: &LinkProps) -> Html {
    let onclick = {
        let route = props.route.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            RouterService::push(&route)
        })
    };
    html! {
        <a class=props.classes.clone() onclick=onclick>{props.children.clone()}</a>
    }
}
