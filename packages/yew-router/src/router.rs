//! Router Component.

use std::marker::PhantomData;

use crate::{AnyHistory, BrowserHistory, History, Location, Routable};
use gloo::console;
use std::rc::Rc;
use yew::prelude::*;

/// Wraps `Rc` around `Fn` so it can be passed as a prop.
pub struct RenderFn<R>(Rc<dyn Fn(&R) -> Html>);

impl<R> RenderFn<R> {
    /// Creates a new [`RenderFn`]
    ///
    /// It is recommended that you use [`Router::render`] instead
    pub fn new(value: impl Fn(&R) -> Html + 'static) -> Self {
        Self(Rc::new(value))
    }
    pub fn render(&self, route: &R) -> Html {
        (self.0)(route)
    }
}

impl<T> Clone for RenderFn<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> PartialEq for RenderFn<T> {
    fn eq(&self, other: &Self) -> bool {
        // https://github.com/rust-lang/rust-clippy/issues/6524
        #[allow(clippy::vtable_address_comparisons)]
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Props for [`Router`]
#[derive(Properties)]
pub struct RouterProps<R> {
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: RenderFn<R>,
}

impl<R> Clone for RouterProps<R> {
    fn clone(&self) -> Self {
        Self {
            render: self.render.clone(),
        }
    }
}

impl<R> PartialEq for RouterProps<R> {
    fn eq(&self, other: &Self) -> bool {
        self.render.eq(&other.render)
    }
}

#[derive(Clone)]
pub struct HistoryState<R, H>
where
    R: Routable + 'static,
    H: History<R> + 'static,
{
    pub(crate) history: H,
    _phantom: PhantomData<R>,
    ctr: u32,
}

impl<R, H> PartialEq for HistoryState<R, H>
where
    R: Routable + 'static,
    H: History<R> + 'static,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

/// The router component.
///
/// When a route can't be matched, it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
#[function_component(Router)]
pub fn router<R>(props: &RouterProps<R>) -> Html
where
    R: Routable + 'static,
{
    let history: UseStateHandle<BrowserHistory<R>> = use_state(BrowserHistory::new);
    let ctr = use_state(|| 0);

    use_effect_with_deps(
        |(ctr, history)| {
            let ctr = ctr.to_owned();
            let listener = history.listen(move || {
                ctr.set(*ctr + 1);
            });

            || {
                let _listener = listener;
            }
        },
        (ctr.clone(), history.clone()),
    );

    let location = history.location();
    let route = location.route();

    let state = HistoryState {
        history: (*history).clone(),
        _phantom: PhantomData,
        ctr: *ctr,
    };

    let any_state = HistoryState {
        history: AnyHistory::Browser((*history).clone()),
        _phantom: PhantomData,
        ctr: *ctr,
    };

    let children = match route.clone() {
        Some(route) => props.render.render(&route),
        None => {
            console::warn!("no route matched");
            Html::default()
        }
    };

    html! {
        <ContextProvider<HistoryState<R, BrowserHistory<R>>> context={state}>
            <ContextProvider<HistoryState<R, AnyHistory<R>>> context={any_state}>
                {children}
            </ContextProvider<HistoryState<R, AnyHistory<R>>>>
        </ContextProvider<HistoryState<R, BrowserHistory<R>>>>
    }
}
