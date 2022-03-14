//! [AppHandle] contains the state Yew keeps to bootstrap a component in an isolated scope.

use crate::html::Scoped;
use crate::html::{IntoComponent, NodeRef, Scope};
use std::ops::Deref;
use std::rc::Rc;
use web_sys::Element;

/// An instance of an application.
#[derive(Debug)]
#[cfg_attr(documenting, doc(cfg(feature = "render")))]
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
    pub(crate) fn mount_with_props(element: Element, props: Rc<ICOMP::Properties>) -> Self {
        clear_element(&element);
        let app = Self {
            scope: Scope::new(None),
        };

        app.scope
            .mount_in_place(element, NodeRef::default(), NodeRef::default(), props);

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

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use crate::dom_bundle::Fragment;

    impl<ICOMP> AppHandle<ICOMP>
    where
        ICOMP: IntoComponent,
    {
        pub(crate) fn hydrate_with_props(element: Element, props: Rc<ICOMP::Properties>) -> Self {
            let app = Self {
                scope: Scope::new(None),
            };

            let mut fragment = Fragment::collect_children(&element);

            app.scope
                .hydrate_in_place(element.clone(), &mut fragment, NodeRef::default(), props);

            // We remove all remaining nodes, this mimics the clear_element behaviour in
            // mount_with_props.
            for node in fragment.iter() {
                element.remove_child(node).unwrap();
            }

            app
        }
    }
}
