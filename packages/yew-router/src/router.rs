//! Router Component.

use crate::{
    agent::{RouteAgentBridge, RouteRequest},
    route::Route,
    RouteState, Switch,
};
use std::{
    fmt::{self, Debug, Error as FmtError, Formatter},
    rc::Rc,
};
use yew::{html, virtual_dom::VNode, Component, ComponentLink, Html, Properties, ShouldRender};

/// Any state that can be managed by the `Router` must meet the criteria of this trait.
pub trait RouterState: RouteState + PartialEq {}
impl<STATE> RouterState for STATE where STATE: RouteState + PartialEq {}

/// Rendering control flow component.
///
/// # Example
/// ```
/// use yew::{prelude::*, virtual_dom::VNode};
/// use yew_router::{router::Router, Switch};
///
/// pub enum Msg {}
///
/// pub struct Model {}
/// impl Component for Model {
///     //...
/// #   type Message = Msg;
/// #   type Properties = ();
/// #   fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
/// #       Model {}
/// #   }
/// #   fn update(&mut self, msg: Self::Message) -> ShouldRender {
/// #        false
/// #   }
/// #   fn change(&mut self, props: Self::Properties) -> ShouldRender {
/// #        false
/// #   }
///
///     fn view(&self) -> VNode {
///         html! {
///         <Router<S>
///            render = Router::render(|switch: S| {
///                match switch {
///                    S::Variant => html!{"variant route was matched"},
///                }
///            })
///         />
///         }
///     }
/// }
///
/// #[derive(Switch, Clone)]
/// enum S {
///     #[at = "/v"]
///     Variant,
/// }
/// ```
// TODO, can M just be removed due to not having to explicitly deal with callbacks anymore? - Just get rid of M
#[derive(Debug)]
pub struct Router<SW: Switch + Clone + 'static, STATE: RouterState = ()> {
    switch: Option<SW>,
    props: Props<STATE, SW>,
    router_agent: RouteAgentBridge<STATE>,
}

impl<SW, STATE> Router<SW, STATE>
where
    STATE: RouterState,
    SW: Switch + Clone + 'static,
{
    /// Wrap a render closure so that it can be used by the Router.
    /// # Example
    /// ```
    /// # use yew_router::Switch;
    /// # use yew_router::router::{Router, Render};
    /// # use yew::{html, Html};
    /// # #[derive(Switch, Clone)]
    /// # enum S {
    /// #     #[at = "/route"]
    /// #     Variant
    /// # }
    /// # pub enum Msg {}
    ///
    /// # fn dont_execute() {
    /// let render: Render<S> = Router::render(|switch: S| -> Html {
    ///     match switch {
    ///         S::Variant => html! {"Variant"},
    ///     }
    /// });
    /// # }
    /// ```
    pub fn render<F: RenderFn<Router<SW, STATE>, SW> + 'static>(f: F) -> Render<SW, STATE> {
        Render::new(f)
    }

    /// Wrap a redirect function so that it can be used by the Router.
    pub fn redirect<F: RedirectFn<SW, STATE> + 'static>(f: F) -> Option<Redirect<SW, STATE>> {
        Some(Redirect::new(f))
    }
}

/// Message for Router.
#[derive(Debug, Clone)]
pub enum Msg<STATE> {
    /// Updates the route
    UpdateRoute(Route<STATE>),
}

/// Render function that takes a switched route and converts it to HTML
pub trait RenderFn<CTX: Component, SW>: Fn(SW) -> Html {}
impl<T, CTX: Component, SW> RenderFn<CTX, SW> for T where T: Fn(SW) -> Html {}
/// Owned Render function.
#[derive(Clone)]
pub struct Render<SW: Switch + Clone + 'static, STATE: RouterState = ()>(
    pub(crate) Rc<dyn RenderFn<Router<SW, STATE>, SW>>,
);
impl<STATE: RouterState, SW: Switch + Clone> Render<SW, STATE> {
    /// New render function
    fn new<F: RenderFn<Router<SW, STATE>, SW> + 'static>(f: F) -> Self {
        Render(Rc::new(f))
    }
}
impl<STATE: RouterState, SW: Switch + Clone> Debug for Render<SW, STATE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Render").finish()
    }
}

/// Redirection function that takes a route that didn't match any of the Switch variants,
/// and converts it to a switch variant.
pub trait RedirectFn<SW, STATE>: Fn(Route<STATE>) -> SW {}
impl<T, SW, STATE> RedirectFn<SW, STATE> for T where T: Fn(Route<STATE>) -> SW {}
/// Clonable Redirect function
#[derive(Clone)]
pub struct Redirect<SW: Switch + 'static, STATE: RouterState>(
    pub(crate) Rc<dyn RedirectFn<SW, STATE>>,
);
impl<STATE: RouterState, SW: Switch + 'static> Redirect<SW, STATE> {
    fn new<F: RedirectFn<SW, STATE> + 'static>(f: F) -> Self {
        Redirect(Rc::new(f))
    }
}
impl<STATE: RouterState, SW: Switch> Debug for Redirect<SW, STATE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Redirect").finish()
    }
}

/// Properties for Router.
#[derive(Properties, Clone)]
pub struct Props<STATE: RouterState, SW: Switch + Clone + 'static> {
    /// Render function that takes a Switch and produces Html
    pub render: Render<SW, STATE>,
    /// Optional redirect function that will convert the route to a known switch variant if explicit matching fails.
    /// This should mostly be used to handle 404s and redirection.
    /// It is not strictly necessary as your Switch is capable of handling unknown routes using `#[at = "/{*:any}"]`.
    #[prop_or_default]
    pub redirect: Option<Redirect<SW, STATE>>,
}

impl<STATE: RouterState, SW: Switch + Clone> Debug for Props<STATE, SW> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_struct("Props").finish()
    }
}

impl<STATE, SW> Component for Router<SW, STATE>
where
    STATE: RouterState,
    SW: Switch + Clone + 'static,
{
    type Message = Msg<STATE>;
    type Properties = Props<STATE, SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateRoute);
        let mut router_agent = RouteAgentBridge::new(callback);
        router_agent.send(RouteRequest::GetCurrentRoute);

        Router {
            switch: Default::default(), /* This must be updated by immediately requesting a route
                                         * update from the service bridge. */
            props,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateRoute(route) => {
                let mut switch = SW::switch(route.clone());

                if switch.is_none() {
                    if let Some(redirect) = &self.props.redirect {
                        let redirected: SW = (&redirect.0)(route);

                        log::trace!(
                            "Route failed to match, but redirecting route to a known switch."
                        );
                        // Replace the route in the browser with the redirected.
                        self.router_agent
                            .send(RouteRequest::ReplaceRouteNoBroadcast(
                                redirected.clone().into(),
                            ));
                        switch = Some(redirected)
                    }
                }

                self.switch = switch;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        match self.switch.clone() {
            Some(switch) => (&self.props.render.0)(switch),
            None => {
                log::warn!("No route matched, provide a redirect prop to the router to handle cases where no route can be matched");
                html! {"No route matched"}
            }
        }
    }
}
