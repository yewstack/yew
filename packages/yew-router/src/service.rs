use crate::utils::base_url;
use crate::Routable;
use gloo::events::EventListener;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::Event;
use yew::Callback;

/// Navigate to a specific route, pushing the new route onto the
/// user's history stack.
pub fn push_route(route: impl Routable) {
    update_route_impl(route.to_path(), true)
}

/// Navigate to a specific route, replacing the current route on the
/// user's history stack.
pub fn replace_route(route: impl Routable) {
    update_route_impl(route.to_path(), false)
}

/// Navigate to a specific route with query parameters, pushing the
/// new route onto the user's history stack.
///
/// This should be used in cases where [`Link`](crate::prelude::Link) is insufficient.
pub fn push_route_with_query<S>(
    route: impl Routable,
    query: S,
) -> Result<(), serde_urlencoded::ser::Error>
where
    S: Serialize,
{
    update_route_with_query_impl(route, query, true)
}

/// Navigate to a specific route with query parameters, replacing the
/// current route on the user's history stack.
pub fn replace_route_with_query<S>(
    route: impl Routable,
    query: S,
) -> Result<(), serde_urlencoded::ser::Error>
where
    S: Serialize,
{
    update_route_with_query_impl(route, query, false)
}

fn update_route_with_query_impl<S>(
    route: impl Routable,
    query: S,
    push: bool,
) -> Result<(), serde_urlencoded::ser::Error>
where
    S: Serialize,
{
    let mut url = route.to_path();
    let query = serde_urlencoded::to_string(query)?;
    if !query.is_empty() {
        url.push_str(&format!("?{}", query));
    }

    update_route_impl(url, push);

    Ok(())
}

fn update_route_impl(url: String, push: bool) {
    let history = yew::utils::window().history().expect("no history");
    let base = base_url();
    let path = match base {
        Some(base) => {
            let path = format!("{}{}", base, url);
            if path.is_empty() {
                "/".to_string()
            } else {
                path
            }
        }
        None => url,
    };

    if push {
        history
            .push_state_with_url(&JsValue::NULL, "", Some(&path))
            .expect("push history");
    } else {
        history
            .replace_state_with_url(&JsValue::NULL, "", Some(&path))
            .expect("replace history");
    }
    let event = Event::new("popstate").unwrap();
    yew::utils::window()
        .dispatch_event(&event)
        .expect("dispatch");
}

pub fn parse_query<T>() -> Result<T, serde_urlencoded::de::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let query = yew::utils::document().location().unwrap().search().unwrap();
    serde_urlencoded::from_str(query.strip_prefix('?').unwrap_or(""))
}

pub fn current_route<R: Routable>() -> Option<R> {
    R::current_route()
}

/// Handle for the router's path event listener
pub struct RouteListener {
    // this exists so listener is dropped when handle is dropped
    #[allow(dead_code)]
    listener: EventListener,
}

/// Adds a listener which is called when the current route is changed.
///
/// The callback receives `Option<R>` so it can handle the error case itself.
pub fn attach_route_listener<R>(callback: Callback<Option<R>>) -> RouteListener
where
    R: Routable + 'static,
{
    let listener = EventListener::new(&yew::utils::window(), "popstate", move |_| {
        callback.emit(current_route())
    });

    RouteListener { listener }
}
