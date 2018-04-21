//! This module contains `App` sctruct which used to bootstrap
//! a component in an isolated scope.

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::{document, Element, INode, IParentNode};
use html::{Scope, ScopeBuilder, Env, Component, Renderable, SharedContext};

/// An application instance.
pub struct App<CTX, COMP: Component<CTX>> {
    /// `Scope` holder
    scope: Option<Scope<CTX, COMP>>,
    /// Environment of the created scope
    env: Env<CTX, COMP>,
}

impl<CTX, COMP> App<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    /// Creates a new `App` with a component in a context.
    pub fn new(context: CTX) -> Self {
        let context = Rc::new(RefCell::new(context));
        App::reuse(context)
    }

    /// Creates isolated `App` instance, but reuse the context.
    pub fn reuse(context: SharedContext<CTX>) -> Self {
        let builder = ScopeBuilder::new();
        let scope = builder.build(context);
        let env = scope.get_env();
        App {
            scope: Some(scope),
            env,
        }
    }

    /// Alias to `mount("body", ...)`.
    pub fn mount_to_body(self) {
        let element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        self.mount(element);
    }

    /// The main entrypoint of a yew program. It works similar as `program`
    /// function in Elm. You should provide an initial model, `update` function
    /// which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub fn mount(mut self, element: Element) {
        clear_element(&element);
        self.scope.take()
            .expect("can't mount the same app twice")
            .mount_in_place(element, None, None, None);
    }

    /// Returns an environment.
    pub fn get_env(&self) -> Env<CTX, COMP> {
        self.env.clone()
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}

