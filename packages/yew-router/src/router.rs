//! Router Component.

use std::fmt::{self, Debug};
use std::rc::Rc;

use yew::context::{Context, ContextListener};
use yew::html::AnyScope;
use yew::prelude::*;
use yew_functional::{get_current_scope, use_hook};

use crate::context::RoutingContext;
use crate::top_level::{self, RouterAction, TopLevelListener};
use crate::Routable;

pub struct Router<T: Routable>(Option<Context<RoutingContext<T>>>);

impl<T: Routable> Clone for Router<T> {
    fn clone(&self) -> Self {
        Router(self.0.clone())
    }
}

impl<T: Routable> Debug for Router<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Router").finish()
    }
}

enum RouterListenerInner<T: Routable> {
    Mounted(ContextListener<RoutingContext<T>>),
    TopLevel(TopLevelListener<T>),
}
pub struct RouterListener<T: Routable>(RouterListenerInner<T>);

impl<T: Routable> Debug for RouterListener<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RouterListener").finish()
    }
}

impl<T: Routable> Router<T> {
    pub fn new(link: impl Into<AnyScope>) -> Self {
        let link = link.into();
        Self(link.context())
    }
    pub fn register(&self, callback: Callback<Rc<T>>) -> RouterListener<T> {
        RouterListener(if let Some(context) = &self.0 {
            RouterListenerInner::Mounted(
                context.register(callback.reform(|ctx: RoutingContext<T>| ctx.route)),
            )
        } else {
            RouterListenerInner::TopLevel(top_level::register(callback))
        })
    }
    pub fn current(&self) -> Rc<T> {
        if let Some(context) = &self.0 {
            context.current().route
        } else {
            top_level::current()
        }
    }
    pub fn dispatch(&self, action: RouterAction<T>) {
        if let Some(context) = &self.0 {
            context.current().onroute.emit(action)
        } else {
            top_level::dispatch(action)
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
        router: Option<(Router<T2>, RouterListener<T2>)>,
    }

    let scope = get_current_scope()
        .expect("No current Scope. `use_router` can only be called inside function components");

    use_hook(
        move || UseRouterState { router: None },
        |state: &mut UseRouterState<T>, updater| {
            if state.router.is_none() {
                let router = Router::new(scope);
                let listener = router.register(Callback::from(move |_| {
                    updater.callback(|_: &mut UseRouterState<T>| true);
                }));
                state.router = Some((router, listener));
            }

            state.router.as_ref().unwrap().0.clone()
        },
        |state| {
            state.router = None;
        },
    )
}
