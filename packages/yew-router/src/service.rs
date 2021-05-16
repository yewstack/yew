use crate::utils::build_path_with_base;
use crate::Routable;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::Event;
use std::fmt;

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

#[derive(Debug, thiserror::Error)]
pub enum ParseQueryError {
    /// Serialize error
    Ser(#[from] serde_urlencoded::ser::Error),
    /// Deserialize error
    De(#[from] serde_urlencoded::de::Error),
}

impl fmt::Display for ParseQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseQueryError::Ser(e) => write!(f, "{}", e),
            ParseQueryError::De(e) => write!(f, "{}", e),
        }
    }
}

pub fn parse_query<T>() -> Result<T, ParseQueryError>
where
    T: for<'de> Deserialize<'de>,
{
    let raw = crate::utils::get_query_params();
    let string = serde_urlencoded::to_string(&raw)?;
    let parsed = serde_urlencoded::from_str(&string)?;
    Ok(parsed)
}
