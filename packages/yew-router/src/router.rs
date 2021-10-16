//! Router Component.

use std::marker::PhantomData;

use crate::{BrowserHistory, History, Location, Routable};
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

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

#[derive(Clone)]
pub(crate) struct RouterState<H, R>
where
    H: History<R>,
    R: Routable + 'static,
{
    pub(crate) history: H,
    _phantom: PhantomData<R>,
    ctr: u32,
}

impl<H, R> PartialEq for RouterState<H, R>
where
    H: History<R>,
    R: Routable + 'static,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

pub(crate) fn use_router_state<H, R>() -> Option<RouterState<H, R>>
where
    H: History<R> + 'static,
    R: Routable + 'static,
{
    use_context::<RouterState<H, R>>()
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

    let state = RouterState {
        history: (*history).clone(),
        _phantom: PhantomData,
        ctr: *ctr,
    };

    let location = history.location();

    let route = location.route();

    let children = match route {
        Some(route) => props.render.render(&route),
        None => {
            console::warn!("no route matched");
            Html::default()
        }
    };

    html! {
        <ContextProvider<RouterState<BrowserHistory<R>, R>> context={state}>
            {children}
        </ContextProvider<RouterState<BrowserHistory<R>, R>>>
    }
}

// pub struct _Router<R: Routable + 'static> {
//     #[allow(dead_code)] // only exists to drop listener on component drop
//     route_listener: EventListener,
//     _data: PhantomData<R>,
// }

// impl<R> Component for _Router<R>
// where
//     R: Routable + 'static,
// {
//     type Message = Msg;
//     type Properties = RouterProps<R>;

//     fn create(ctx: &Context<Self>) -> Self {
//         let link = ctx.link().clone();
//         let route_listener = EventListener::new(&yew::utils::window(), "popstate", move |_| {
//             link.send_message(Msg::ReRender)
//         });

//         Self {
//             route_listener,
//             _data: PhantomData,
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::ReRender => true,
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let pathname = yew::utils::window().location().pathname().unwrap();
//         let route = R::recognize(&pathname);

//         match route {
//             Some(route) => (ctx.props().render.0)(&route),
//             None => {
//                 console::warn!("no route matched");
//                 html! {}
//             }
//         }
//     }

//     fn destroy(&mut self, _ctx: &Context<Self>) {
//         R::cleanup();
//     }
// }
