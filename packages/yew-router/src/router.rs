//! Router Component.

use crate::Routable;
use gloo::{console, events::EventListener};
use std::marker::PhantomData;
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

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

/// The router component.
///
/// When a route can't be matched, it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
pub struct Router<R: Routable + 'static> {
    #[allow(dead_code)] // only exists to drop listener on component drop
    route_listener: EventListener,
    _data: PhantomData<R>,
}

impl<R> Component for Router<R>
where
    R: Routable + 'static,
{
    type Message = Msg;
    type Properties = RouterProps<R>;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let route_listener = EventListener::new(&gloo_utils::window(), "popstate", move |_| {
            link.send_message(Msg::ReRender)
        });

        Self {
            route_listener,
            _data: PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReRender => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pathname = gloo_utils::window().location().pathname().unwrap();
        let route = R::recognize(&pathname);

        match route {
            Some(route) => (ctx.props().render.0)(&route),
            None => {
                console::warn!("no route matched");
                html! {}
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        R::cleanup();
    }
}

impl<R> Router<R>
where
    R: Routable + Clone + 'static,
{
    pub fn render<F>(func: F) -> RenderFn<R>
    where
        F: Fn(&R) -> Html + 'static,
    {
        RenderFn::new(func)
    }
}
