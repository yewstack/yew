//! [AppHandle] contains the state Yew keeps to bootstrap a component in an isolated scope.

use std::ops::Deref;
use std::rc::Rc;

use web_sys::Element;

use crate::dom_bundle::{BSubtree, DomSlot, DynamicDomSlot};
use crate::html::{BaseComponent, Scope, Scoped};

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
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        name = "mount",
        skip(props),
    )]
    pub(crate) fn mount_with_props(host: Element, props: Rc<COMP::Properties>) -> Self {
        clear_element(&host);
        let app = Self {
            scope: Scope::new(None),
        };
        let hosting_root = BSubtree::create_root(&host);
        app.scope.mount_in_place(
            hosting_root,
            host,
            DomSlot::at_end(),
            DynamicDomSlot::new_debug_trapped(),
            props,
        );

        app
    }

    /// Update the properties of the app's root component.
    ///
    /// This can be an alternative to sending and handling messages. The existing component will be
    /// reused and have its properties updates. This will presumably trigger a re-render, refer to
    /// the [`changed`] lifecycle for details.
    ///
    /// [`changed`]: crate::Component::changed
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip_all,
    )]
    pub fn update(&mut self, new_props: COMP::Properties) {
        self.scope.reuse(Rc::new(new_props), DomSlot::at_end())
    }

    /// Schedule the app for destruction
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip_all,
    )]
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

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::Fragment;

    impl<COMP> AppHandle<COMP>
    where
        COMP: BaseComponent,
    {
        #[tracing::instrument(
            level = tracing::Level::DEBUG,
            name = "hydrate",
            skip(props),
        )]
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
                DynamicDomSlot::new_debug_trapped(),
                Rc::clone(&props),
            );
            #[cfg(debug_assertions)] // Fix trapped next_sibling at the root
            app.scope.reuse(props, DomSlot::at_end());

            // We remove all remaining nodes, this mimics the clear_element behaviour in
            // mount_with_props.
            for node in fragment.iter() {
                host.remove_child(node).unwrap();
            }

            app
        }
    }
}
