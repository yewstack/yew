use crate::utils::build_path_with_base;
use crate::Routable;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::Event;

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

pub fn query_parameters() -> HashMap<String, String> {
    crate::utils::get_query_params()
}
