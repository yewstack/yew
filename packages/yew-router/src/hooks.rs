use crate::history::*;
use crate::routable::Routable;
use crate::router::HistoryState;

use yew::prelude::*;

pub fn use_history<R, H>() -> Option<H>
where
    R: Routable + 'static,
    H: History<R> + 'static,
{
    let history_state = use_context::<HistoryState<R, H>>()?;

    Some(history_state.history)
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
    use_history::<R, BrowserHistory<R>>()
}

pub fn use_browser_location<R>() -> Option<BrowserLocation<R>>
where
    R: Routable + 'static,
{
    Some(use_browser_history()?.location())
}

pub fn use_any_history<R>() -> Option<AnyHistory<R>>
where
    R: Routable + 'static,
{
    use_history::<R, AnyHistory<R>>()
}

pub fn use_any_location<R>() -> Option<AnyLocation<R>>
where
    R: Routable + 'static,
{
    use_location::<R, AnyLocation<R>>()
}

pub fn use_route<R>() -> Option<R>
where
    R: Routable + 'static,
{
    use_any_location::<R>().and_then(|m| m.route())
}
