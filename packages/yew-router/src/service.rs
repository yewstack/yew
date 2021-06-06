use std::cell::RefCell;
use std::rc::Rc;

use anymap::AnyMap;
use yew::Callback;

use super::history::{self, HistoryListener, Route};
use crate::Routable;

type Entry<'a, T> = anymap::Entry<'a, dyn anymap::any::Any, T>;

#[derive(Debug, Clone)]
#[non_exhaustive]
enum RouterAction<T: Routable> {
    Push(T),
    Replace(T),
}

/// Specializes the history API for a particular `Routable` type
pub(crate) struct RouterState<T: Routable> {
    last_route: Rc<T>,
    subscribers: Vec<Callback<Rc<T>>>,
    _listener: HistoryListener,
}

thread_local! {
    static ROUTER_STATE: RefCell<AnyMap> = RefCell::new(AnyMap::new());
}

impl<T: Routable> RouterState<T> {
    fn with<R>(f: impl FnOnce(Entry<Self>) -> R) -> R {
        ROUTER_STATE.with(|state| f(state.borrow_mut().entry()))
    }
    fn try_with<R>(mut f: impl FnMut(Entry<Self>) -> Result<R, T>) -> R {
        match Self::with(&mut f) {
            Ok(res) => return res,
            Err(error) => Self::handle_error(error),
        };
        match Self::with(&mut f) {
            Ok(res) => res,
            Err(_) => panic!("Bug in `Routable` implementation"),
        }
    }
    fn determine_current_route() -> Result<T, T> {
        T::from_route(&*history::current())
    }
    fn new() -> Result<Self, T> {
        let last_route = Rc::new(Self::determine_current_route()?);
        Ok(Self {
            last_route,
            subscribers: Vec::new(),
            _listener: history::register(Callback::from(Self::update)),
        })
    }

    fn try_insert(entry: Entry<Self>) -> Result<&mut Self, T> {
        Ok(match entry {
            Entry::Occupied(occ) => occ.into_mut(),
            Entry::Vacant(vac) => vac.insert(Self::new()?),
        })
    }

    // We sometimes return a function to run when the state is not borrowed.
    // This is so that callbacks don't panic if they try to access the state.
    fn update_inner(
        entry: Entry<Self>,
        route: Rc<Route>,
    ) -> Result<Option<impl FnOnce() + 'static>, T> {
        let mut occ = match entry {
            Entry::Occupied(occ) => occ,
            Entry::Vacant(_) => return Ok(None),
        };
        let state = occ.get_mut();
        Ok(if state.subscribers.is_empty() {
            occ.remove();
            None
        } else {
            let route = Rc::new(T::from_route(&*route)?);
            if state.last_route != route {
                state.last_route = route.clone();
                let subscribers = state.subscribers.clone();
                Some(move || {
                    for subscriber in subscribers {
                        subscriber.emit(route.clone());
                    }
                })
            } else {
                None
            }
        })
    }

    fn handle_error(error: T) {
        // Whenever we fail to recognize a route, we redirect to the default one
        history::replace(error.to_route());
    }

    fn update(route: Rc<Route>) {
        match Self::with(|entry| Self::update_inner(entry, route)) {
            Ok(None) => {}
            Ok(Some(f)) => f(),
            Err(error) => Self::handle_error(error),
        }
    }

    fn current_route(entry: Entry<Self>) -> Result<Rc<T>, T> {
        Ok(Self::try_insert(entry)?.last_route.clone())
    }

    fn register(entry: Entry<Self>, callback: Callback<Rc<T>>) -> Result<RouteListener<T>, T> {
        Self::try_insert(entry)?.subscribers.push(callback.clone());
        Ok(RouteListener(callback))
    }

    fn unregister(entry: Entry<Self>, callback: &Callback<Rc<T>>) {
        if let Entry::Occupied(occ) = entry {
            let state = occ.into_mut();
            if let Some(index) = state
                .subscribers
                .iter()
                .position(|subscriber| subscriber == callback)
            {
                state.subscribers.remove(index);
            }
        }
    }
}

/// A guard returned from `attach_route_listener`. When dropped, the callback will
/// be detached and will no longer receive events.
pub struct RouteListener<T: Routable>(Callback<Rc<T>>);

impl<T: Routable> Drop for RouteListener<T> {
    fn drop(&mut self) {
        RouterState::with(|entry| RouterState::unregister(entry, &self.0));
    }
}

fn dispatch<T: Routable>(action: RouterAction<T>) {
    match action {
        RouterAction::Push(route) => history::push(route.to_route()),
        RouterAction::Replace(route) => history::replace(route.to_route()),
    }
}
/// Navigate to the specified route. This will add a new entry to the user's history.
pub fn push_route<T: Routable>(route: T) {
    dispatch(RouterAction::Push(route));
}
/// Navigate to the specified route. This will replace the current entry in the user's history.
pub fn replace_route<T: Routable>(route: T) {
    dispatch(RouterAction::Replace(route));
}
/// Obtain the current route.
pub fn current_route<T: Routable>() -> Rc<T> {
    RouterState::try_with(RouterState::current_route)
}
/// Register a callback to be notified if the route changes.
pub fn attach_route_listener<T: Routable>(callback: Callback<Rc<T>>) -> RouteListener<T> {
    RouterState::try_with(|entry| RouterState::register(entry, callback.clone()))
}
