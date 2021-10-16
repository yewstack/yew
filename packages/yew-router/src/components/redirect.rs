use std::marker::PhantomData;

use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::scope_ext::RouterScopeExt;
use crate::{AnyHistory, History, Routable};

/// Props for [`Redirect`]
#[derive(Properties, Clone, PartialEq)]
pub struct RedirectProps<R: Routable + Clone + PartialEq> {
    /// Route that will be pushed when the component is rendered.
    pub to: R,
}

/// A component that will redirect to specified route when rendered.
pub struct Redirect<R: Routable + Clone + PartialEq + 'static> {
    _data: PhantomData<R>,
}

impl<R> Component for Redirect<R>
where
    R: Routable + Clone + PartialEq + 'static,
{
    type Message = ();
    type Properties = RedirectProps<R>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { _data: PhantomData }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.link()
            .history::<R, AnyHistory<R>>()
            .expect_throw("failed to read history.")
            .push(ctx.props().to.clone());
        Html::default()
    }
}
