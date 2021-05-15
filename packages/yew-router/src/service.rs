use crate::utils::build_path_with_base;
use crate::Routable;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::Event;

/// Navigate to a specific route.
///
/// This should be used in cases where [`Link`](crate::prelude::Link) is insufficient.
pub fn push(route: impl Routable, query: Option<HashMap<&str, String>>) {
    let mut url = route.to_route();
    if let Some(query) = query {
        url.push('?');
        query.iter().for_each(|(k, v)| {
            url.push_str(&format!("{}={}&", k, v));
        })
    }
    let url = url.strip_suffix('&').unwrap_or_else(|| url.as_str());

    let history = yew::utils::window().history().expect("no history");

    history
        .push_state_with_url(&JsValue::NULL, "", Some(&build_path_with_base(url)))
        .expect("push history");
    let event = Event::new("popstate").unwrap();
    yew::utils::window()
        .dispatch_event(&event)
        .expect("dispatch");
}

pub fn query() -> HashMap<String, String> {
    crate::utils::get_query_params()
}
