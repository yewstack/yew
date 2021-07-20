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
use std::rc::Rc;

/// Should Render
pub type ShouldRender = bool;

/// Context
#[derive(Debug)]
pub struct Context<COMP: Component> {
    pub(crate) scope: Scope<COMP>,
    pub(crate) props: Rc<COMP::Properties>,
}

impl<COMP: Component> Context<COMP> {
    /// The component link
    #[inline]
    pub fn link(&self) -> &Scope<COMP> {
        &self.scope
    }

    /// The component's props
    #[inline]
    pub fn props(&self) -> &COMP::Properties {
        &*self.props
    }
}

#[allow(missing_docs)]
/// Yew component
pub trait Component: Sized + 'static {
    type Message: 'static;
    type Properties: Properties;

    fn create(ctx: &Context<Self>) -> Self;
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> ShouldRender {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html;
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
