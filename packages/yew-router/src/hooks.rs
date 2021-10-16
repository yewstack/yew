use crate::history::{BrowserHistory, BrowserLocation, History, Location};
use crate::routable::Routable;
use crate::router::use_router_state;

pub fn use_history<R, H>() -> Option<H>
where
    R: Routable + 'static,
    H: History<R> + 'static,
{
    let router_state = use_router_state::<H, R>()?;

    Some(router_state.history)
}

pub fn use_location<R, L>() -> Option<L>
where
    R: Routable + 'static,
    L: Location<R> + 'static,
{
    Some(use_history::<R, L::History>()?.location())
}

pub fn use_browser_history<R>() -> Option<BrowserHistory<R>>
where
    R: Routable + 'static,
{
    let router_state = use_router_state::<BrowserHistory<R>, R>()?;

    Some(router_state.history)
}

pub fn use_browser_location<R>() -> Option<BrowserLocation<R>>
where
    R: Routable + 'static,
{
    Some(use_browser_history()?.location())
}
