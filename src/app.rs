//! This module contains `App` sctruct which used to bootstrap
//! a component in an isolated scope.

use stdweb::web::{document, Element, INode, IParentNode};
use html::{Scope, Component, Renderable};
use scheduler::Scheduler;

/// An application instance.
pub struct App<CTX, COMP: Component<CTX>> {
    /// `Scope` holder
    scope: Scope<CTX, COMP>,
}

impl<CTX, COMP> App<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    /// Creates a new `App` with a component in a context.
    pub fn new(context: CTX) -> Self {
        let scheduler = Scheduler::new(context);
        App::reuse(&scheduler)
    }

    /// Creates isolated `App` instance, but reuse the context.
    pub fn reuse(scheduler: &Scheduler<CTX>) -> Self {
        let scope = Scope::new(scheduler.clone());
        App { scope }
    }

    /// Alias to `mount("body", ...)`.
    pub fn mount_to_body(self) -> Scope<CTX, COMP> {
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
    pub fn mount(self, element: Element) -> Scope<CTX, COMP> {
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

