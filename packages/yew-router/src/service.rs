use crate::{router::ROUTER, CurrentRoute};

#[derive(Debug, Copy, Clone)]
pub struct RouterService;

impl RouterService {
    pub fn push(url: &str) {
        ROUTER.with(|router| router.push(url))
    }

    pub fn current_route() -> CurrentRoute {
        ROUTER.with(|router| router.current_route.borrow().clone().unwrap())
    }
}
