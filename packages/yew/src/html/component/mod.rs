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

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// The [`Component`]'s context. This contains component's [`Scope`] and and props and
/// is passed to every lifecycle method.
#[derive(Debug)]
pub struct Context<COMP: Component> {
    pub(crate) scope: Scope<COMP>,
    pub(crate) props: Rc<COMP::Properties>,
}

impl<COMP: Component> Context<COMP> {
    /// The component scope
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

/// Components are the basic building blocks of the UI in a Yew app. Each Component
/// chooses how to display itself using received props and self-managed state.
/// Components can be dynamic and interactive by declaring messages that are
/// triggered and handled asynchronously. This async update mechanism is inspired by
/// Elm and the actor model used in the Actix framework.
pub trait Component: Sized + 'static {
    /// Messages are used to make Components dynamic and interactive. Simple
    /// Component's can declare their Message type to be `()`. Complex Component's
    /// commonly use an enum to declare multiple Message types.
    type Message: 'static;

    /// The Component's properties.
    ///
    /// When the parent of a Component is re-rendered, it will either be re-created or
    /// receive new properties in the context passed to the `changed` lifecycle method.
    type Properties: Properties;

    /// Called when component is created.
    fn create(ctx: &Context<Self>) -> Self;

    /// Called when a new message is sent to the component via it's scope.
    ///
    /// Components handle messages in their `update` method and commonly use this method
    /// to update their state and (optionally) re-render themselves.
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> ShouldRender {
        false
    }

    /// Called when properties passed to the component change
    fn changed(&mut self, _ctx: &Context<Self>) -> ShouldRender {
        true
    }

    /// Components define their visual layout using a JSX-style syntax through the use of the
    /// `html!` procedural macro. The full guide to using the macro can be found in [Yew's
    /// documentation](https://yew.rs/concepts/html).
    fn view(&self, ctx: &Context<Self>) -> Html;

    /// The `rendered` method is called after each time a Component is rendered but
    /// before the browser updates the page.
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    /// Called right before a Component is unmounted.
    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
