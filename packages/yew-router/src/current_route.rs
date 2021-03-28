use crate::utils::get_query_params;
use crate::{Params, Routable};
use std::rc::Rc;

/// The current route.
#[derive(Debug, Clone)]
pub struct CurrentRoute {
    route: Rc<dyn Routable>,
    query: Params,
}

impl CurrentRoute {
    pub(crate) fn new(route: impl Routable + 'static) -> Self {
        Self {
            route: Rc::new(route),
            query: get_query_params().into(),
        }
    }

    pub fn route<R: Routable + 'static>(&self) -> &R {
        self.route.as_any().downcast_ref::<R>().unwrap()
    }

    /// Returns the query parameters from the path.
    #[inline]
    pub fn query(&self) -> &Params {
        &self.query
    }
}
