//! The [`Switch`] Component.

use std::marker::PhantomData;

use gloo::console;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::prelude::*;
use crate::scope_ext::LocationHandle;

/// Props for [`Switch`]
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps<R>
where
    R: Routable,
{
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: Callback<R, Html>,
    #[prop_or_default]
    pub pathname: Option<String>,
}

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

/// A Switch that dispatches route among variants of a [`Routable`].
///
/// When a route can't be matched, including when the path is matched but the deserialization fails,
/// it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
pub struct Switch<R: Routable + 'static> {
    _listener: LocationHandle,
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
            .add_location_listener(link.callback(move |_| Msg::ReRender))
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
        let route = ctx
            .props()
            .pathname
            .as_ref()
            .and_then(|p| R::recognize(p))
            .or_else(|| ctx.link().route::<R>());

        let children = match route {
            Some(route) => ctx.props().render.emit(route),
            None => {
                console::warn!("no route matched");
                Html::default()
            }
        };

        html! {<>{children}</>}
    }
}
