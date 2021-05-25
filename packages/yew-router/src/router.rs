//! Router Component.

use std::cell::RefCell;
use std::rc::Rc;

use yew::context::ContextHandle;
use yew::html::AnyScope;
use yew::prelude::*;
use yew_functional::{get_current_scope, use_hook};

use crate::context::RoutingContext;
use crate::services::router::{RouterAction, RouterListener, RouterService};
use crate::Routable;

pub struct Router<T: Routable>(Rc<InnerRouter<T>>);

impl<T: Routable> Clone for Router<T> {
    fn clone(&self) -> Self {
        Router(self.0.clone())
    }
}

enum InnerRouter<T: Routable> {
    // Used when we're mounted inside a routing context
    Context {
        current: RefCell<Rc<RoutingContext<T>>>,
        _handle: ContextHandle<Rc<RoutingContext<T>>>,
    },
    // Used when we're directly interacting with the router service
    Service {
        // Lazily initialize this to avoid unnecessary work
        current: RefCell<Option<Rc<T>>>,
        _listener: RouterListener<T>,
    },
}

#[derive(Debug)]
pub struct RouterUpdate<T: Routable>(InnerRouterUpdate<T>);

#[derive(Debug)]
enum InnerRouterUpdate<T: Routable> {
    // Used when we're mounted inside a routing context
    Context(Rc<RoutingContext<T>>),
    // Used when we're directly interacting with the router agent
    Service(Rc<T>),
}

impl<T: Routable> Router<T> {
    pub fn new(link: impl Into<AnyScope>, callback: Callback<RouterUpdate<T>>) -> Self {
        let link = link.into();
        Self(Rc::new(
            if let Some((current, handle)) =
                link.context(callback.reform(|ctx| RouterUpdate(InnerRouterUpdate::Context(ctx))))
            {
                InnerRouter::Context {
                    current: RefCell::new(current),
                    _handle: handle,
                }
            } else {
                let current = RefCell::new(None);
                let _listener = RouterService::register(
                    callback.reform(|routable| RouterUpdate(InnerRouterUpdate::Service(routable))),
                );
                InnerRouter::Service { current, _listener }
            },
        ))
    }
    pub fn update(&self, update: RouterUpdate<T>) {
        match (&*self.0, update.0) {
            (InnerRouter::Service { current, .. }, InnerRouterUpdate::Service(routable)) => {
                *current.borrow_mut() = Some(routable);
            }
            (InnerRouter::Context { current, .. }, InnerRouterUpdate::Context(ctx)) => {
                *current.borrow_mut() = ctx;
            }
            _ => unreachable!(),
        }
    }
    pub fn route(&self) -> Rc<T> {
        match &*self.0 {
            InnerRouter::Service { current, .. } => {
                let mut current = current.borrow_mut();
                if let Some(route) = &*current {
                    route.clone()
                } else {
                    let route = RouterService::current::<T>();
                    *current = Some(route.clone());
                    route
                }
            }
            InnerRouter::Context { current, .. } => current.borrow().route.clone(),
        }
    }
    pub fn dispatch(&self, action: RouterAction<T>) {
        match &*self.0 {
            InnerRouter::Service { .. } => RouterService::dispatch(action),
            InnerRouter::Context { current, .. } => current.borrow().onroute.emit(action),
        }
    }
    pub fn dispatcher<U>(
        &self,
        action_fn: impl Fn(U) -> Option<RouterAction<T>> + 'static,
    ) -> Callback<U> {
        let router: Router<T> = self.clone();
        (move |args| {
            if let Some(action) = action_fn(args) {
                router.dispatch(action)
            }
        })
        .into()
    }
    pub fn push(&self, routable: T) {
        self.dispatch(RouterAction::Push(routable));
    }
    pub fn replace(&self, routable: T) {
        self.dispatch(RouterAction::Replace(routable));
    }
}

pub fn use_router<T: Routable>() -> Router<T> {
    struct UseRouterState<T2: Routable> {
        router: Option<Router<T2>>,
    }

    let scope = get_current_scope()
        .expect("No current Scope. `use_router` can only be called inside function components");

    use_hook(
        move || UseRouterState { router: None },
        |state: &mut UseRouterState<T>, updater| {
            if state.router.is_none() {
                let callback = move |update: RouterUpdate<T>| {
                    updater.callback(|state: &mut UseRouterState<T>| {
                        if let Some(router) = &state.router {
                            router.update(update);
                            true
                        } else {
                            false
                        }
                    });
                };
                state.router = Some(Router::new(scope, callback.into()))
            }

            state.router.clone().unwrap()
        },
        |state| {
            state.router = None;
        },
    )
}
