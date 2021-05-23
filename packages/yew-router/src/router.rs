//! Router Component.

use crate::{attach_route_listener, current_route, Routable, RouteListener};
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
pub enum Msg<R> {
    UpdateRoute(Option<R>),
}

/// The router component.
///
/// When a route can't be matched, it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
pub struct Router<R: Routable + 'static> {
    props: RouterProps<R>,
    #[allow(dead_code)] // only exists to drop listener on component drop
    route_listener: RouteListener,
    route: Option<R>,
}

impl<R> Component for Router<R>
where
    R: Routable + 'static,
{
    type Message = Msg<R>;
    type Properties = RouterProps<R>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_listener = attach_route_listener(link.callback(Msg::UpdateRoute));

        Self {
            props,
            route_listener,
            route: current_route(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(route) => {
                self.route = route;
                true
            }
        }
    }

    fn change(&mut self, mut props: Self::Properties) -> bool {
        std::mem::swap(&mut self.props, &mut props);
        props != self.props
    }

    fn view(&self) -> Html {
        match &self.route {
            Some(route) => (self.props.render.0)(route),
            None => {
                weblog::console_log!("no route matched");
                html! {}
            }
        }
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
