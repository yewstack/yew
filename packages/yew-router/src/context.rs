use std::rc::Rc;

use yew::Callback;

use crate::{Routable, RouterAction};

#[derive(Debug, PartialEq)]
pub struct RoutingContext<T: Routable> {
    pub(crate) route: Rc<T>,
    pub(crate) onroute: Callback<RouterAction<T>>,
}

impl<T: Routable> Clone for RoutingContext<T> {
    fn clone(&self) -> Self {
        Self {
            route: self.route.clone(),
            onroute: self.onroute.clone(),
        }
    }
}
