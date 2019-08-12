//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

mod listener;
mod scope;

pub use listener::*;
pub(crate) use scope::ComponentUpdate;
pub use scope::{NodeCell, Scope};

use crate::callback::Callback;
use crate::virtual_dom::VNode;

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component: Sized + 'static {
    /// Control message type which `update` loop get.
    type Message: 'static;
    /// Properties type of component implementation.
    type Properties: Properties;
    /// Initialization routine which could use a context.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self;
    /// Called after the component has been attached to the VDOM and it is safe to receive messages
    /// from agents. Any changes made to the state that `view` relies on will not be apparent
    /// immediately as this method does not trigger a rerender.
    fn on_mount(&mut self) {}
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Message) -> ShouldRender;
    /// Called when the component's parent component re-renders and the
    /// component's place in the DOM tree remains unchanged. If the component's
    /// place in the DOM tree changes, calling this method is unnecessary as the
    /// component is recreated from scratch. It defaults
    /// to true if not implemented.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    /// Called for finalization on the final point of the component's lifetime.
    fn destroy(&mut self) {} // TODO Replace with `Drop`
}

/// A type which expected as a result of `view` function implementation.
pub type Html<MSG> = VNode<MSG>;

/// Should be rendered relative to context and component environment.
pub trait Renderable<COMP: Component> {
    /// Called by rendering loop.
    fn view(&self) -> Html<COMP>;
}

/// Trait for building properties for a component
pub trait Properties {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

/// Builder for when a component has no properties
pub struct EmptyBuilder;

impl Properties for () {
    type Builder = EmptyBuilder;

    fn builder() -> Self::Builder {
        EmptyBuilder
    }
}

impl EmptyBuilder {
    /// Build empty properties
    pub fn build(self) {}
}

/// Link to component's scope for creating callbacks.
pub struct ComponentLink<COMP: Component> {
    scope: Scope<COMP>,
}

impl<COMP> ComponentLink<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// Create link for a scope.
    fn connect(scope: &Scope<COMP>) -> Self {
        ComponentLink {
            scope: scope.clone(),
        }
    }

    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> COMP::Message + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input);
            scope.clone().send_message(output);
        };
        closure.into()
    }

    /// This method sends a message to this component immediately.
    pub fn send_self(&mut self, msg: COMP::Message) {
        self.scope.send_message(msg);
    }
}

/// A bridging type for checking `href` attribute value.
#[derive(Debug)]
pub struct Href {
    link: String,
}

impl From<String> for Href {
    fn from(link: String) -> Self {
        Href { link }
    }
}

impl<'a> From<&'a str> for Href {
    fn from(link: &'a str) -> Self {
        Href {
            link: link.to_owned(),
        }
    }
}

impl ToString for Href {
    fn to_string(&self) -> String {
        self.link.to_owned()
    }
}
