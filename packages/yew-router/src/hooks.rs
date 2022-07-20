//! Hooks to access router state and navigate between pages.

use yew::prelude::*;

use crate::history::*;
use crate::navigator::Navigator;
use crate::routable::Routable;
use crate::router::{LocationContext, NavigatorContext};

/// A hook to access the [`Navigator`].
#[hook]
pub fn use_navigator() -> Option<Navigator> {
    use_context::<NavigatorContext>().map(|m| m.navigator())
}

/// A hook to access the current [`Location`].
#[hook]
pub fn use_location() -> Option<Location> {
    Some(use_context::<LocationContext>()?.location())
}

/// A hook to access the current route.
///
/// This hook will return [`None`] if there's no available location or none of the routes match.
///
/// # Note
///
/// If your `Routable` has a `#[not_found]` route, you can use `.unwrap_or_default()` instead of
/// `.unwrap()` to unwrap.
#[hook]
pub fn use_route<R>() -> Option<R>
where
    R: Routable + 'static,
{
    let navigator = use_navigator()?;
    let location = use_location()?;
    let path = navigator.strip_basename(location.path().into());

    R::recognize(&path)
}
