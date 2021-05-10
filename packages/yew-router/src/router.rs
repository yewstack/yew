//! Router Component.

use crate::utils::{base_url, build_path_with_base};
use crate::Routable;
use gloo::events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

pub struct RenderFn<R>(Rc<dyn Fn(R) -> Html>);

impl<R> RenderFn<R> {
    pub fn new(value: impl Fn(R) -> Html + 'static) -> Self {
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
#[derive(Properties, Clone, PartialEq)]
pub struct RouterProps<R: Clone> {
    #[prop_or(None)]
    pub not_found_route: Option<String>,
    pub render: RenderFn<R>,
}

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

/// The router component.
///
/// When a route can't be matched, it looks for the `not_found_route` prop.
/// If the said prop is specified, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
pub struct Router<R: Routable + Clone + PartialEq + 'static> {
    props: RouterProps<R>,
    link: ComponentLink<Self>,
    on_popstate_listener: Option<EventListener>,
    router: route_recognizer::Router<String>,
}

impl<R> Component for Router<R>
where
    R: Routable + Clone + PartialEq + 'static,
{
    type Message = Msg;
    type Properties = RouterProps<R>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let base: Option<String> = base_url();

        let router = {
            let mut router = route_recognizer::Router::new();
            R::routes().iter().for_each(|path| {
                let path = match &base {
                    Some(base) if base != "/" => build_path_with_base(path),
                    _ => path.to_string(),
                };
                router.add(&path, path.clone());
            });
            router
        };

        Self {
            props,
            link,
            on_popstate_listener: None,
            router,
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

        let route = {
            let router = &self.router;
            let matched = router.recognize(&pathname.strip_suffix("/").unwrap_or(&pathname));
            match matched {
                Ok(matched) => {
                    R::from_path(matched.handler(), &matched.params().into_iter().collect())
                }
                Err(_) => match self.props.not_found_route.as_ref() {
                    Some(it) => R::from_path(it, &HashMap::new()),
                    None => None,
                },
            }
        };

        match route {
            Some(route) => (self.props.render.0)(route),
            None => html! {},
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let link = self.link.clone();
            self.on_popstate_listener = Some(EventListener::new(
                &yew::utils::window(),
                "popstate",
                move |_| link.send_message(Msg::ReRender),
            ))
        }
    }
}

impl<R> Router<R>
where
    R: Routable + Clone + PartialEq + 'static,
{
    pub fn render<F>(func: F) -> RenderFn<R>
    where
        F: Fn(R) -> Html + 'static,
    {
        RenderFn::new(func)
    }
}
