//! [AppHandle] contains the state Yew keeps to bootstrap a component in an isolated scope.

use std::marker::PhantomData;
use std::rc::Rc;

use web_sys::Element;

use crate::dom_bundle::BSubtree;
use crate::html::{Component, ComponentIntrinsic, NodeRef, Scope};
use crate::scheduler;

/// An instance of an application.
#[cfg(feature = "csr")]
#[derive(Debug)]
pub struct AppHandle<COMP: Component> {
    /// `Scope` holder
    pub(crate) scope: Scope,
    _marker: PhantomData<COMP>,
    destroyed: bool,
}

impl<COMP> AppHandle<COMP>
where
    COMP: Component,
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
    pub(crate) fn mount_with_props(host: Element, props: COMP::Properties) -> Self {
        clear_element(&host);
        let intrinsic = Rc::new(ComponentIntrinsic::<COMP>::new(props));

        let app = Self {
            scope: Scope::new(intrinsic.as_ref(), None),
            _marker: PhantomData,
            destroyed: false,
        };
        let hosting_root = BSubtree::create_root(&host);

        {
            let scope = app.scope.clone();
            scheduler::push(move || {
                scope.mount(
                    intrinsic,
                    hosting_root,
                    host,
                    NodeRef::default(),
                    NodeRef::default(),
                );
            });
        }

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
        let intrinsic = Rc::new(ComponentIntrinsic::<COMP>::new(new_props));
        self.scope.reuse(intrinsic, NodeRef::default())
    }

    /// Schedule the app for destruction
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip_all,
    )]
    pub fn destroy(mut self) {
        self.destroyed = true;
        let scope = self.scope.clone();
        scheduler::push(move || {
            scope.destroy(false);
        });
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
        COMP: Component,
    {
        #[tracing::instrument(
            level = tracing::Level::DEBUG,
            name = "hydrate",
            skip(props),
        )]
        pub(crate) fn hydrate_with_props(host: Element, props: COMP::Properties) -> Self {
            let intrinsic = Rc::new(ComponentIntrinsic::<COMP>::new(props));

            let app = Self {
                scope: Scope::new(intrinsic.as_ref(), None),
                _marker: PhantomData,
                destroyed: false,
            };

            let mut fragment = Fragment::collect_children(&host);
            let hosting_root = BSubtree::create_root(&host);

            let scope = app.scope.clone();

            scheduler::push(move || {
                scope.hydrate(
                    intrinsic.clone(),
                    hosting_root,
                    host.clone(),
                    &mut fragment,
                    NodeRef::default(),
                );

                // We remove all remaining nodes, this mimics the clear_element behaviour in
                // mount_with_props.
                for node in fragment.iter() {
                    host.remove_child(node).unwrap();
                }
            });

            app
        }
    }
}

impl<COMP> Drop for AppHandle<COMP>
where
    COMP: Component,
{
    fn drop(&mut self) {
        if !self.destroyed {
            // We leak scope if hasn't been destroyed.
            // So the application stays running for as long as the page is open.
            Box::leak(Box::new(self.scope.clone()));
        }
    }
}
