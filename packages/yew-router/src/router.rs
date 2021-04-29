//! Router Component.

use crate::utils::{base_url, build_path_with_base};
use crate::Routable;
use gloo::events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

pub struct RenderFn<R>(Rc<dyn Fn(R) -> Html>);

impl<R> RenderFn<R> {
    pub fn new(value: impl Fn(R) -> Html + 'static) -> Self {
        Self(Rc::new(value))
    }
}

impl<T> Clone for RenderFn<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> PartialEq for RenderFn<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Props for [`Router`]
#[derive(Properties, Clone, PartialEq)]
pub struct RouterProps<R: Clone> {
    #[prop_or(None)]
    pub not_found_route: Option<String>,
    pub render: RenderFn<R>,
}

/// The router component.
///
/// When a route can't be matched, it looks for the `not_found_route` prop.
/// If the said prop is specified, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
#[function_component(Router)]
pub fn router<R: Routable + Clone + PartialEq + 'static>(props: &RouterProps<R>) -> Html {
    let pathname = yew::utils::window().location().pathname().unwrap();
    let base: Option<String> = base_url();

    let router = use_ref(|| {
        let mut router = route_recognizer::Router::new();
        R::routes().iter().for_each(|path| {
            let path = match &base {
                Some(base) if base != "/" => build_path_with_base(path),
                _ => path.to_string(),
            };
            router.add(&path, path.clone());
        });
        router
    });

    let route = {
        let router = router.borrow();
        let matched = router.recognize(&pathname.strip_suffix("/").unwrap_or(&pathname));
        let matched = match matched {
            Ok(matched) => R::from_path(matched.handler(), &matched.params().into_iter().collect()),
            Err(_) => match props.not_found_route.as_ref() {
                Some(it) => R::from_path(it, &HashMap::new()),
                None => None,
            },
        };
        matched
    };

    let output = match route {
        Some(route) => (props.render.0)(route),
        None => html! {},
    };
    let (force_rerender, set_force_rerender) = use_state(|| 0);

    let _ = use_effect(move || {
        let event_listener = EventListener::new(&yew::utils::window(), "popstate", move |_| {
            set_force_rerender(*force_rerender + 1);
        });

        move || {
            drop(event_listener);
        }
    });

    html! {
        { output }
    }
}
