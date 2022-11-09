//! [AppHandle] contains the state Yew keeps to bootstrap a component in an isolated scope.

use std::marker::PhantomData;
use std::rc::Rc;

use web_sys::Element;

use crate::dom_bundle::BSubtree;
use crate::html::{BaseComponent, NodeRef, Scope, Scoped};
use crate::scheduler;

/// An instance of an application.
#[cfg(feature = "csr")]
#[derive(Debug)]
pub struct AppHandle<COMP: BaseComponent> {
    /// `Scope` holder
    pub(crate) scope: Scope,
    _marker: PhantomData<COMP>,
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
            scope: Scope::new::<COMP>(None),
            _marker: PhantomData,
        };
        let hosting_root = BSubtree::create_root(&host);

        {
            let scope = app.scope.clone();
            scheduler::push(move || {
                scope.to_any().mount(
                    hosting_root,
                    host,
                    NodeRef::default(),
                    NodeRef::default(),
                    |ctx| COMP::create(ctx),
                    props,
                );
            });
        }

        app
    }

    /// Schedule the app for destruction
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip_all,
    )]
    pub fn destroy(self) {
        let scope = self.scope;
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
    use crate::virtual_dom::Collectable;

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
                scope: Scope::new::<COMP>(None),
                _marker: PhantomData,
            };

            let mut fragment = Fragment::collect_children(&host);
            let hosting_root = BSubtree::create_root(&host);

            let scope = app.scope.clone();

            scheduler::push(move || {
                scope.to_any().hydrate(
                    hosting_root,
                    host.clone(),
                    &mut fragment,
                    NodeRef::default(),
                    |ctx| COMP::create(ctx),
                    props.clone(),
                    || Collectable::for_component::<COMP>(),
                );
                #[cfg(debug_assertions)] // Fix trapped next_sibling at the root
                scope.to_any().reuse(props, NodeRef::default());

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
