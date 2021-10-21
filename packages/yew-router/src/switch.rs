//! The [`Switch`] Component.

use std::marker::PhantomData;
use std::rc::Rc;

use gloo::console;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::prelude::*;
use crate::scope_ext::HistoryHandle;

/// Wraps `Rc` around `Fn` so it can be passed as a prop.
pub struct RenderFn<R>(Rc<dyn Fn(&R) -> Html>);

impl<R> RenderFn<R> {
    /// Creates a new [`RenderFn`]
    ///
    /// It is recommended that you use [`Switch::render`] instead
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

/// Props for [`Switch`]
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps<R>
where
    R: Routable,
{
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: RenderFn<R>,
}

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

/// A Switch that dispatches route among variants of a [`Routable`].
///
/// When a route can't be matched, it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
pub struct Switch<R: Routable + 'static> {
    _listener: HistoryHandle,
    _phantom: PhantomData<R>,
}

impl<R> Component for Switch<R>
where
    R: Routable + 'static,
{
    type Message = Msg;
    type Properties = SwitchProps<R>;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let listener = link
            .add_history_listener(link.callback(move |_| Msg::ReRender))
            .expect_throw("failed to create history handle. Do you have a router registered?");

        Self {
            _listener: listener,
            _phantom: PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReRender => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let route = ctx.link().location().and_then(|m| m.route::<R>());

        let children = match &route {
            Some(ref route) => (ctx.props().render.0)(route),
            None => {
                console::warn!("no route matched");
                Html::default()
            }
        };

        html! {<>{children}</>}
    }
}

impl<R> Switch<R>
where
    R: Routable + Clone + 'static,
{
    /// Creates a [`RenderFn`].
    pub fn render<F>(func: F) -> RenderFn<R>
    where
        F: Fn(&R) -> Html + 'static,
    {
        RenderFn::new(func)
    }
}
