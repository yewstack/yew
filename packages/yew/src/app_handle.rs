//! This module contains the `App` struct, which is used to bootstrap
//! a component in an isolated scope.

use std::ops::Deref;

use crate::html::{BaseComponent, NodeRef, Scope, Scoped};
use std::rc::Rc;
use web_sys::Element;

/// An instance of an application.
#[derive(Debug)]
pub struct AppHandle<COMP: BaseComponent> {
    /// `Scope` holder
    pub(crate) scope: Scope<COMP>,
}

impl<COMP> AppHandle<COMP>
where
    COMP: BaseComponent,
{
    /// The main entry point of a Yew program which also allows passing properties. It works
    /// similarly to the `program` function in Elm. You should provide an initial model, `update`
    /// function which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub(crate) fn mount_with_props(element: Element, props: Rc<COMP::Properties>) -> Self {
        clear_element(&element);
        let app = Self {
            scope: Scope::new(None),
        };
        app.scope
            .mount_in_place(element, NodeRef::default(), NodeRef::default(), props);

        app
    }

    /// Schedule the app for destruction
    pub fn destroy(mut self) {
        self.scope.destroy()
    }
}

impl<COMP> Deref for AppHandle<COMP>
where
    COMP: BaseComponent,
{
    type Target = Scope<COMP>;

    fn deref(&self) -> &Self::Target {
        &self.scope
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}
