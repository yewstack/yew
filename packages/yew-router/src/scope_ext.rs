use crate::history::*;
use crate::routable::Routable;
use crate::router::HistoryState;

use yew::context::ContextHandle;
use yew::prelude::*;

pub trait RouterScopeExt {
    fn history<R, H>(&self) -> Option<H>
    where
        R: Routable + 'static,
        H: History<R> + 'static;

    fn location<R, L>(&self) -> Option<L>
    where
        R: Routable + 'static,
        L: Location<R> + 'static;

    fn add_route_listener<R, H>(
        &self,
        cb: Callback<H>,
    ) -> Option<ContextHandle<HistoryState<R, H>>>
    where
        R: Routable + 'static,
        H: History<R> + 'static;
}

impl<COMP: Component> RouterScopeExt for yew::html::Scope<COMP> {
    fn history<R, H>(&self) -> Option<H>
    where
        R: Routable + 'static,
        H: History<R> + 'static,
    {
        self.context::<HistoryState<R, H>>(Callback::from(|_| {}))
            .map(|(m, _)| m.history)
    }

    fn location<R, L>(&self) -> Option<L>
    where
        R: Routable + 'static,
        L: Location<R> + 'static,
    {
        self.history::<R, L::History>().map(|m| m.location())
    }

    fn add_route_listener<R, H>(&self, cb: Callback<H>) -> Option<ContextHandle<HistoryState<R, H>>>
    where
        R: Routable + 'static,
        H: History<R> + 'static,
    {
        self.context::<HistoryState<R, H>>(Callback::from(move |m: HistoryState<R, H>| {
            cb.emit(m.history)
        }))
        .map(|(_, m)| m)
    }
}
