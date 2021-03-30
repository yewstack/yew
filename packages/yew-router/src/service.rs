use crate::{router::ROUTER, CurrentRoute, Routable};
use std::collections::HashMap;

/// Service to interface with the router.
#[derive(Debug, Copy, Clone)]
pub struct RouterService;

impl RouterService {
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
        ROUTER.with(|router| router.push(url))
    }

    /// The current route.
    pub fn current_route() -> CurrentRoute {
        ROUTER.with(|router| router.current_route.borrow().clone().unwrap())
    }
}
