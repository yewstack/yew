use crate::{service, Routable};
use std::marker::PhantomData;
use yew::prelude::*;

/// Props for [`Link`]
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
        service::push_route(ctx.props().to.clone());
        Html::default()
    }
}
