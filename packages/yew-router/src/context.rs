use std::rc::Rc;

use yew::Callback;

use crate::agents::router::RouterAction;
use crate::Routable;

#[derive(Debug, PartialEq)]
pub struct RoutingContext<T: Routable> {
    pub(crate) route: Rc<T>,
    pub(crate) onroute: Callback<RouterAction<T>>,
}
