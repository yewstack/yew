use crate::history::*;
use crate::routable::Routable;
use crate::router::RouterState;

use yew::context::ContextHandle;
use yew::prelude::*;

pub struct HistoryHandle<H>
where
    H: History + 'static,
{
    _inner: ContextHandle<RouterState<H>>,
}

pub struct RouteHandle<R>
where
    R: Routable + 'static,
{
    _inner: ContextHandle<Option<R>>,
}

pub trait RouterScopeExt {
    fn history<H>(&self) -> Option<H>
    where
        H: History + 'static;

    fn location<L>(&self) -> Option<L>
    where
        L: Location + 'static;

    fn route<R>(&self) -> Option<R>
    where
        R: Routable + 'static;

    fn add_history_listener<H>(&self, cb: Callback<H>) -> Option<HistoryHandle<H>>
    where
        H: History + 'static;

    fn add_route_listener<R>(&self, cb: Callback<Option<R>>) -> Option<RouteHandle<R>>
    where
        R: Routable + 'static;
}

impl<COMP: Component> RouterScopeExt for yew::html::Scope<COMP> {
    fn history<H>(&self) -> Option<H>
    where
        H: History + 'static,
    {
        self.context::<RouterState<H>>(Callback::from(|_| {}))
            .map(|(m, _)| m.history())
    }

    fn location<L>(&self) -> Option<L>
    where
        L: Location + 'static,
    {
        self.history::<L::History>().map(|m| m.location())
    }

    fn route<R>(&self) -> Option<R>
    where
        R: Routable + 'static,
    {
        self.context::<Option<R>>(Callback::from(|_| {}))
            .and_then(|(m, _)| m)
    }

    fn add_history_listener<H>(&self, cb: Callback<H>) -> Option<HistoryHandle<H>>
    where
        H: History + 'static,
    {
        self.context::<RouterState<H>>(Callback::from(move |m: RouterState<H>| {
            cb.emit(m.history())
        }))
        .map(|(_, m)| HistoryHandle { _inner: m })
    }

    fn add_route_listener<R>(&self, cb: Callback<Option<R>>) -> Option<RouteHandle<R>>
    where
        R: Routable + 'static,
    {
        self.context::<Option<R>>(Callback::from(move |m: Option<R>| cb.emit(m)))
            .map(|(_, m)| RouteHandle { _inner: m })
    }
}
