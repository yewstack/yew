use crate::{router::ROUTER, CurrentRoute};

/// Service to interface with the router.
#[derive(Debug, Copy, Clone)]
pub struct RouterService;

impl RouterService {
    /// Navigate to a specific route.
    ///
    /// This should be used in cases where [`Link`](crate::prelude::Link) is insufficient.
    pub fn push(url: &str) {
        ROUTER.with(|router| router.push(url))
    }

    /// The current route.
    pub fn current_route() -> CurrentRoute {
        ROUTER.with(|router| router.current_route.borrow().clone().unwrap())
    }
}
