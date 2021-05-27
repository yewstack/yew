//! Router Component.

use crate::Routable;
use gloo::events::EventListener;
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
    props: RouterProps<R>,
    #[allow(dead_code)] // only exists to drop listener on component drop
    route_listener: EventListener,
}

impl<R> Component for Router<R>
where
    R: Routable + 'static,
{
    type Message = Msg;
    type Properties = RouterProps<R>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_listener = EventListener::new(&yew::utils::window(), "popstate", move |_| {
            link.send_message(Msg::ReRender)
        });

        Self {
            props,
            route_listener,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ReRender => true,
        }
    }

    fn change(&mut self, mut props: Self::Properties) -> bool {
        std::mem::swap(&mut self.props, &mut props);
        props != self.props
    }

    fn view(&self) -> Html {
        let pathname = yew::utils::window().location().pathname().unwrap();
        let route = R::recognize(&pathname);

        match route {
            Some(route) => (self.props.render.0)(&route),
            None => {
                weblog::console_warn!("no route matched");
                html! {}
            }
        }
    }

    fn destroy(&mut self) {
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
