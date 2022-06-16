//! [AppHandle] contains the state Yew keeps to bootstrap a component in an isolated scope.

use std::ops::Deref;
use std::rc::Rc;

use web_sys::Element;

use crate::dom_bundle::BSubtree;
use crate::html::{BaseComponent, NodeRef, Scope, Scoped};

/// An instance of an application.
#[cfg_attr(documenting, doc(cfg(feature = "csr")))]
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
    pub(crate) fn mount_with_props(host: Element, props: Rc<COMP::Properties>) -> Self {
        clear_element(&host);
        let app = Self {
            scope: Scope::new(None),
        };
        let hosting_root = BSubtree::create_root(&host);
        app.scope.mount_in_place(
            hosting_root,
            host,
            NodeRef::default(),
            NodeRef::default(),
            props,
        );

        app
    }

    /// Schedule the app for destruction
    pub fn destroy(self) {
        self.scope.destroy(false)
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
fn clear_element(host: &Element) {
    while let Some(child) = host.last_child() {
        host.remove_child(&child).expect("can't remove a child");
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::Fragment;

    impl<COMP> AppHandle<COMP>
    where
        COMP: BaseComponent,
    {
        pub(crate) fn hydrate_with_props(host: Element, props: Rc<COMP::Properties>) -> Self {
            let app = Self {
                scope: Scope::new(None),
            };

            let mut fragment = Fragment::collect_children(&host);
            let hosting_root = BSubtree::create_root(&host);

            app.scope.hydrate_in_place(
                hosting_root,
                host.clone(),
                &mut fragment,
                NodeRef::default(),
                Rc::clone(&props),
            );
            #[cfg(debug_assertions)] // Fix trapped next_sibling at the root
            app.scope.reuse(props, NodeRef::default());

            // We remove all remaining nodes, this mimics the clear_element behaviour in
            // mount_with_props.
            for node in fragment.iter() {
                host.remove_child(node).unwrap();
            }

            app
        }
    }
}
