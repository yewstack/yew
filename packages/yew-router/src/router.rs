use crate::utils::{base_url, build_path_with_base, from_route};
use crate::{components::route::Route, CurrentRoute};
use gloo::events::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Event, History};
use weblog::*;
use yew::prelude::*;
use yew_functional::*;

pub(crate) struct YewRouter {
    pub(crate) history: History,
    pub(crate) current_route: RefCell<Option<CurrentRoute>>,
}

impl YewRouter {
    fn new() -> Self {
        Self {
            history: yew::utils::window().history().expect("no history"),
            current_route: RefCell::new(None),
        }
    }

    pub(crate) fn push(&self, url: &str) {
        self.history
            .push_state_with_url(&JsValue::null(), "", Some(&build_path_with_base(url)))
            .expect("push history");
        let event = Event::new("popstate").unwrap();
        yew::utils::window()
            .dispatch_event(&event)
            .expect("dispatch");
    }
}

thread_local! {
    pub(crate) static ROUTER: Rc<YewRouter> = Rc::new(YewRouter::new());
}

#[derive(Properties, Clone, PartialEq)]
pub struct RouterProps {
    #[prop_or(None)]
    pub not_found_route: Option<String>,
    pub children: ChildrenWithProps<Route>,
}

#[function_component(Router)]
pub fn router(props: &RouterProps) -> Html {
    let pathname = yew::utils::window().location().pathname().unwrap();
    let base: Option<String> = base_url();

    let router = use_ref(|| {
        let mut router = route_recognizer::Router::new();
        props.children.iter().for_each(|child| {
            let to = match &base {
                Some(base) if base != "/" => build_path_with_base(&child.props.to),
                _ => child.props.to,
            };
            router.add(&to, to.clone());
        });
        router
    });
    let route = from_route(
        &pathname,
        &props.children,
        props.not_found_route.as_deref(),
        &*router.borrow(),
    );
    let (children, current_route) = match route {
        Some(route) => route,
        None => {
            weblog::console_warn!("no route matched");
            return html!();
        }
    };

    let (force_rerender, set_force_rerender) = use_state(|| 0);

    ROUTER.with(|f| {
        *f.current_route.borrow_mut() = Some(current_route);
    });

    let _ = use_effect(move || {
        let event_listener = EventListener::new(&yew::utils::window(), "popstate", move |_| {
            set_force_rerender(*force_rerender + 1);
        });

        move || {
            drop(event_listener);
        }
    });

    html! {
        { for children }
    }
}
