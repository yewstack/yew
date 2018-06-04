//! This module contains `App` sctruct which used to bootstrap
//! a component in an isolated scope.

use stdweb::web::{document, Element, INode, IParentNode};
use html::{Scope, Component, Renderable};
use scheduler::Scheduler;

/// An application instance.
pub struct App<COMP: Component> {
    /// `Scope` holder
    scope: Scope<COMP>,
}

impl<COMP> App<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// Creates a new `App` with a component in a context.
    pub fn new() -> Self {
        let scheduler = Scheduler::new();
        App::reuse(&scheduler)
    }

    /// Creates isolated `App` instance, but reuse the context.
    pub fn reuse(scheduler: &Scheduler) -> Self {
        let scope = Scope::new(scheduler.clone());
        App { scope }
    }

    /// Alias to `mount("body", ...)`.
    pub fn mount_to_body(self) -> Scope<COMP> {
        // Bootstrap the component for `Window` environment only (not for `Worker`)
        let element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        self.mount(element)
    }

    /// The main entrypoint of a yew program. It works similar as `program`
    /// function in Elm. You should provide an initial model, `update` function
    /// which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub fn mount(self, element: Element) -> Scope<COMP> {
        clear_element(&element);
        self.scope.mount_in_place(element, None, None, None)
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}

