//! Component scope module

use std::any::{Any, TypeId};
#[cfg(feature = "csr")]
use std::cell::RefCell;
use std::rc::Rc;
use std::{fmt, iter};

#[cfg(feature = "csr")]
use super::lifecycle::ComponentState;
use super::BaseComponent;
use crate::callback::Callback;
use crate::context::{ContextHandle, ContextProvider, ContextStore};
#[cfg(feature = "hydration")]
use crate::html::RenderMode;

struct ScopeInner {
    id: usize,
    type_id: TypeId,

    #[cfg(feature = "csr")]
    pub(crate) state: RefCell<Option<ComponentState>>,

    parent: Option<Scope>,
}

/// Untyped scope used for accessing parent scope
#[derive(Clone)]
pub struct Scope {
    inner: Rc<ScopeInner>,
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AnyScope<_>")
    }
}

impl Scope {
    pub(crate) fn id(&self) -> usize {
        self.inner.id
    }

    /// Schedules a render.
    pub(crate) fn schedule_render(&self) {
        #[cfg(feature = "csr")]
        {
            use crate::scheduler;

            let scope = self.clone();
            scheduler::push(move || ComponentState::run_render(&scope));
        }
    }

    /// Returns the parent scope
    pub fn parent(&self) -> Option<&Scope> {
        self.inner.parent.as_ref()
    }

    pub(crate) fn state_cell(&self) -> &RefCell<Option<ComponentState>> {
        &self.inner.state
    }

    /// Returns the type of the linked component
    pub fn type_id(&self) -> TypeId {
        self.inner.type_id
    }

    /// Attempts checks the component type of current scope
    ///
    /// Returns [`None`] if the self value can't be cast into the target type.
    pub(crate) fn is_scope_of<COMP: BaseComponent>(&self) -> bool {
        self.type_id() == TypeId::of::<COMP>()
    }

    /// Attempts to find a parent scope of a certain type
    ///
    /// Returns [`None`] if no parent scope with the specified type was found.
    pub(crate) fn find_parent_scope<COMP: BaseComponent>(&self) -> Option<&Scope> {
        iter::successors(Some(self), |scope| scope.parent()).find(|m| m.is_scope_of::<COMP>())
    }

    /// Accesses a value provided by a parent `ContextProvider` component of the
    /// same type.
    pub fn context<T: Clone + PartialEq + 'static>(
        &self,
        callback: Callback<T>,
    ) -> Option<(T, ContextHandle<T>)> {
        let scope = self.find_parent_scope::<ContextProvider<T>>()?;
        let store = ContextStore::<T>::get(scope)?;
        Some(ContextStore::subscribe_consumer(store, callback))
    }
}

/// A context which allows sending messages to a component.
// pub(crate) struct Scope<COMP: BaseComponent> {
//     _marker: PhantomData<COMP>,
//     parent: Option<Rc<AnyScope>>,

//     #[cfg(feature = "csr")]
//     pub(crate) state: Rc<RefCell<Option<ComponentState>>>,

//     pub(crate) id: usize,
// }

// impl<COMP: BaseComponent> fmt::Debug for Scope<COMP> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str("Scope<_>")
//     }
// }

// impl<COMP: BaseComponent> Clone for Scope<COMP> {
//     fn clone(&self) -> Self {
//         Scope {
//             _marker: PhantomData,

//             parent: self.parent.clone(),

//             #[cfg(feature = "csr")]
//             state: self.state.clone(),

//             id: self.id,
//         }
//     }
// }

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt::Write;

    use super::*;
    use crate::html::RenderError;
    #[cfg(feature = "hydration")]
    use crate::html::RenderMode;
    use crate::platform::fmt::BufWriter;
    use crate::virtual_dom::Collectable;
    use crate::Context;

    impl Scope {
        pub(crate) async fn render_into_stream<COMP>(
            &self,
            w: &mut BufWriter,
            props: Rc<COMP::Properties>,
            hydratable: bool,
        ) where
            COMP: BaseComponent,
        {
            // Rust's Future implementation is stack-allocated and incurs zero runtime-cost.
            //
            // If the content of this channel is ready before it is awaited, it is
            // similar to taking the value from a mutex lock.

            let context = Context {
                scope: self.clone(),
                props: props as Rc<dyn Any>,
                #[cfg(feature = "hydration")]
                creation_mode: RenderMode::Ssr,
                #[cfg(feature = "hydration")]
                prepared_state: None,
            };

            let component = COMP::create(&context);

            let collectable = Collectable::for_component::<COMP>();

            if hydratable {
                collectable.write_open_tag(w);
            }

            let html = loop {
                match component.render(context.props().as_ref()) {
                    Ok(m) => break m,
                    Err(RenderError::Suspended(e)) => e.await,
                }
            };

            html.render_into_stream(w, self, hydratable).await;

            if let Some(prepared_state) = component.prepare_state() {
                let _ = w.write_str(r#"<script type="application/x-yew-comp-state">"#);
                let _ = w.write_str(&prepared_state);
                let _ = w.write_str(r#"</script>"#);
            }

            if hydratable {
                collectable.write_close_tag(w);
            }
        }
    }
}

#[cfg(any(feature = "ssr", feature = "csr"))]
mod feat_csr_ssr {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static COMP_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

    impl Scope {
        /// Crate a scope with an optional parent scope
        pub(crate) fn new<COMP>(parent: Option<Scope>) -> Self
        where
            COMP: BaseComponent,
        {
            Scope {
                inner: Rc::new(ScopeInner {
                    type_id: TypeId::of::<COMP>(),

                    #[cfg(feature = "csr")]
                    state: RefCell::new(None),
                    parent,

                    id: COMP_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                }),
            }
        }
    }
}

#[cfg(feature = "csr")]
mod feat_csr {
    use web_sys::Element;

    use super::*;
    use crate::dom_bundle::{BSubtree, Bundle};
    use crate::html::component::lifecycle::Realized;
    use crate::html::NodeRef;
    use crate::{Context, FunctionComponent};

    impl Scope {
        #[cfg(test)]
        pub(crate) fn test() -> Self {
            Self {
                inner: Rc::new(ScopeInner {
                    id: 0,
                    type_id: TypeId::of::<()>(),
                    state: RefCell::default(),
                    parent: None,
                }),
            }
        }

        pub(crate) fn reuse(&self, props: Rc<dyn Any>, next_sibling: NodeRef) {
            ComponentState::run_update_props(self, Some(props), Some(next_sibling));
        }

        /// Mounts a component with `props` to the specified `element` in the DOM.
        pub(crate) fn mount(
            &self,
            root: BSubtree,
            parent: Element,
            next_sibling: NodeRef,
            internal_ref: NodeRef,
            create_component: fn(&Context) -> FunctionComponent,
            props: Rc<dyn Any>,
        ) {
            let bundle = Bundle::new();
            internal_ref.link(next_sibling.clone());
            let stable_next_sibling = NodeRef::default();
            stable_next_sibling.link(next_sibling);

            let state = Realized::Bundle(bundle);

            let context = Context {
                scope: self.clone(),
                props: props as Rc<dyn Any>,
                #[cfg(feature = "hydration")]
                creation_mode: RenderMode::Render,
                #[cfg(feature = "hydration")]
                prepared_state: None,
            };

            let component = create_component(&context);

            ComponentState::run_create(
                context,
                component,
                state,
                root,
                parent,
                stable_next_sibling,
                internal_ref,
            );
        }
    }

    pub(crate) trait Scoped {
        fn to_any(&self) -> Scope;
        /// Shift the node associated with this scope to a new place
        fn shift_node(&self, parent: Element, next_sibling: NodeRef);
        /// Process an event to destroy a component
        fn destroy(self, parent_to_detach: bool);
        fn destroy_boxed(self: Box<Self>, parent_to_detach: bool);
    }

    impl Scoped for Scope {
        fn to_any(&self) -> Scope {
            self.clone()
        }

        /// Process an event to destroy a component
        fn destroy(self, parent_to_detach: bool) {
            ComponentState::run_destroy(&self, parent_to_detach);
        }

        fn destroy_boxed(self: Box<Self>, parent_to_detach: bool) {
            self.destroy(parent_to_detach);
        }

        fn shift_node(&self, parent: Element, next_sibling: NodeRef) {
            ComponentState::run_shift(self, parent, next_sibling);
        }
    }
}

#[cfg(feature = "csr")]
pub(crate) use feat_csr::*;

#[cfg(feature = "hydration")]
mod feat_hydration {
    use wasm_bindgen::JsCast;
    use web_sys::{Element, HtmlScriptElement};

    use super::*;
    use crate::dom_bundle::{BSubtree, Fragment};
    use crate::html::component::lifecycle::Realized;
    use crate::html::NodeRef;
    use crate::virtual_dom::Collectable;
    use crate::{Context, FunctionComponent};

    impl Scope {
        /// Hydrates the component.
        ///
        /// Returns a pending NodeRef of the next sibling.
        ///
        /// # Note
        ///
        /// This method is expected to collect all the elements belongs to the current component
        /// immediately.
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn hydrate(
            &self,
            root: BSubtree,
            parent: Element,
            fragment: &mut Fragment,
            internal_ref: NodeRef,
            create_component: fn(&Context) -> FunctionComponent,
            props: Rc<dyn Any>,
            create_collectable: fn() -> Collectable,
        ) {
            let collectable = create_collectable();

            let mut fragment = Fragment::collect_between(fragment, &collectable, &parent);
            match fragment.front().cloned() {
                front @ Some(_) => internal_ref.set(front),
                None =>
                {
                    #[cfg(debug_assertions)]
                    internal_ref.link(NodeRef::new_debug_trapped())
                }
            }

            let prepared_state = match fragment
                .back()
                .cloned()
                .and_then(|m| m.dyn_into::<HtmlScriptElement>().ok())
            {
                Some(m) if m.type_() == "application/x-yew-comp-state" => {
                    fragment.pop_back();
                    parent.remove_child(&m).unwrap();
                    Some(m.text().unwrap())
                }
                _ => None,
            };

            let state = Realized::Fragement(fragment);

            let scope = self.to_any();

            let context = Context {
                scope,
                props: props as Rc<dyn Any>,
                creation_mode: RenderMode::Hydration,
                prepared_state,
            };

            let component = create_component(&context);

            ComponentState::run_create(
                context,
                component,
                state,
                root,
                parent,
                NodeRef::new_debug_trapped(),
                internal_ref,
            );
        }
    }
}
