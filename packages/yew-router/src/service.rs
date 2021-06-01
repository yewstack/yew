use crate::utils::base_url;
use crate::Routable;
use gloo::events::EventListener;
use wasm_bindgen::JsValue;
use web_sys::Event;
use yew::Callback;

/// Navigate to a specific route.
pub fn push_route(route: impl Routable) {
    let url = route.to_path();
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

    history
        .push_state_with_url(&JsValue::NULL, "", Some(&path))
        .expect("push history");
    let event = Event::new("popstate").unwrap();
    yew::utils::window()
        .dispatch_event(&event)
        .expect("dispatch");
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
