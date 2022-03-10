//! [AppHandle] contains the state Yew keeps to bootstrap a component in an isolated scope.

use super::{BundleRoot, ComponentRenderState, Scoped};
use crate::html::{IntoComponent, NodeRef, Scope};
use std::ops::Deref;
use std::rc::Rc;
use web_sys::Element;

/// An instance of an application.
#[derive(Debug)]
pub struct AppHandle<ICOMP: IntoComponent> {
    /// `Scope` holder
    pub(crate) scope: Scope<<ICOMP as IntoComponent>::Component>,
}

impl<ICOMP> AppHandle<ICOMP>
where
    ICOMP: IntoComponent,
{
    /// The main entry point of a Yew program which also allows passing properties. It works
    /// similarly to the `program` function in Elm. You should provide an initial model, `update`
    /// function which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub(crate) fn mount_with_props(host: Element, props: Rc<ICOMP::Properties>) -> Self {
        clear_element(&host);
        let app = Self {
            scope: Scope::new(None),
        };
        let node_ref = NodeRef::default();
        let hosting_root = BundleRoot::create_root(&host);
        let initial_render_state =
            ComponentRenderState::new(hosting_root, host, NodeRef::default(), &node_ref);
        app.scope
            .mount_in_place(initial_render_state, node_ref, props);

        app
    }

    /// Schedule the app for destruction
    pub fn destroy(self) {
        self.scope.destroy(false)
    }
}

impl<ICOMP> Deref for AppHandle<ICOMP>
where
    ICOMP: IntoComponent,
{
    type Target = Scope<<ICOMP as IntoComponent>::Component>;

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
