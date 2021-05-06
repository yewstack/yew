//! This module contains the `App` struct, which is used to bootstrap
//! a component in an isolated scope.

use crate::html::{Component, NodeRef, Scope, Scoped};
use crate::utils::document;
use web_sys::Element;

/// An instance of an application.
#[derive(Debug)]
pub struct AppHandle<COMP: Component> {
    /// `Scope` holder
    pub(crate) scope: Scope<COMP>,
}

impl<COMP> Default for AppHandle<COMP>
where
    COMP: Component,
{
    fn default() -> Self {
        AppHandle::new()
    }
}

impl<COMP> AppHandle<COMP>
where
    COMP: Component,
{
    /// Creates a new `App` with a component in a context.
    pub(crate) fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let scope = Scope::new(None);
        AppHandle { scope }
    }

    /// The main entry point of a Yew program which also allows passing properties. It works
    /// similarly to the `program` function in Elm. You should provide an initial model, `update`
    /// function which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub(crate) fn mount_with_props(element: Element, props: COMP::Properties) -> Self {
        clear_element(&element);
        let app = Self::new();
        app.scope
            .mount_in_place(element, NodeRef::default(), NodeRef::default(), props);

        app
    }

    /// Alias to `mount_with_props("body", ...)`.
    pub(crate) fn mount_to_body_with_props(props: COMP::Properties) -> Self {
        // Bootstrap the component for `Window` environment only (not for `Worker`)
        let element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");

        Self::mount_with_props(element, props)
    }

    /// Alternative to `mount_with_props` which replaces the body element with a component which
    /// has a body element at the root of the HTML generated by its `view` method. Use this method
    /// when you need to manipulate the body element. For example, adding/removing app-wide
    /// CSS classes of the body element.
    pub(crate) fn mount_as_body_with_props(props: COMP::Properties) -> Self {
        let html_element = document()
            .query_selector("html")
            .expect("can't get html node for rendering")
            .expect("can't unwrap html node");
        let body_element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        html_element
            .remove_child(&body_element)
            .expect("can't remove body child");

        Self::mount_with_props(html_element, props)
    }

    /// Schedule the app for destruction
    pub fn destroy(mut self) {
        self.scope.destroy()
    }
}

impl<COMP> AsRef<Scope<COMP>> for AppHandle<COMP>
where
    COMP: Component,
{
    fn as_ref(&self) -> &Scope<COMP> {
        &self.scope
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}
