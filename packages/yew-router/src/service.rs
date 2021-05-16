use crate::utils::build_path_with_base;
use crate::Routable;
use gloo::events::EventListener;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::Event;
use yew::Callback;

/// Navigate to a specific route.
pub fn push_route(route: impl Routable) {
    push_impl(route.to_route())
}

/// Navigate to a specific route with query parameters.
///
/// This should be used in cases where [`Link`](crate::prelude::Link) is insufficient.
pub fn push_route_with_query<S>(
    route: impl Routable,
    query: S,
) -> Result<(), serde_urlencoded::ser::Error>
where
    S: Serialize,
{
    let mut url = route.to_route();
    let query = serde_urlencoded::to_string(query)?;
    if !query.is_empty() {
        url.push_str(&format!("?{}", query));
    }

    push_impl(url);

    Ok(())
}

fn push_impl(url: String) {
    let history = yew::utils::window().history().expect("no history");

    history
        .push_state_with_url(&JsValue::NULL, "", Some(&build_path_with_base(&url)))
        .expect("push history");
    let event = Event::new("popstate").unwrap();
    yew::utils::window()
        .dispatch_event(&event)
        .expect("dispatch");
}

pub fn parse_query<T>() -> Result<T, serde_urlencoded::de::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let url = web_sys::Url::new(&yew::utils::document().url().unwrap()).unwrap();
    let parsed =
        serde_urlencoded::from_str(&url.search().strip_prefix("?").as_deref().unwrap_or(""))?;
    Ok(parsed)
}

pub fn current_route<R: Routable>() -> Option<R> {
    let pathname = yew::utils::window().location().pathname().unwrap();
    R::recognize(&pathname)
}

// just an opaque handle to the actual listener so we don't expose implementation details
/// Handle for the router's path event listener
pub struct RouteListener {
    // this exists so listener is dropped when handle is dropped
    #[allow(dead_code)]
    listener: EventListener,
}

/// Adds a listener which is called when a route is changed.
///
/// The callback receives `Option<R>` so it can handle the error case itself.
pub fn add_onpush_listener<R: Routable + 'static>(callback: Callback<Option<R>>) -> RouteListener {
    let listener = EventListener::new(&yew::utils::window(), "popstate", move |_| {
        callback.emit(current_route())
    });

    RouteListener { listener }
}
