//! Components wrapped with context including properties, state, and link

mod children;
mod lifecycle;
mod properties;
mod scope;

use super::Html;
pub use children::*;
pub use properties::*;
pub(crate) use scope::Scoped;
pub use scope::{AnyScope, Scope, SendAsMessage};

/// Should Render
pub type ShouldRender = bool;

/// Context
pub type Context<T> = Scope<T>;

#[allow(missing_docs)]
/// Yew component
pub trait Component: Sized + 'static {
    type Message: 'static;
    type Properties: Properties;

    fn create(props: Self::Properties, ctx: &Context<Self>) -> Self;
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>, _new_props: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html;
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
