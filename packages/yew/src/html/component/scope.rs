//! Component scope module

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::{fmt, iter};

#[cfg(any(feature = "csr", feature = "ssr"))]
use super::lifecycle::ComponentState;
use super::lifecycle::RenderRunner;
use super::BaseComponent;
use crate::callback::Callback;
use crate::context::{ContextHandle, ContextProvider};
use crate::scheduler;
#[cfg(any(feature = "csr", feature = "ssr"))]
use crate::scheduler::Shared;

thread_local! {
    static PROPS: RefCell<HashMap<usize, Rc<dyn Any>>> = RefCell::default();
}

/// Untyped scope used for accessing parent scope
#[derive(Clone)]
pub struct AnyScope {
    id: usize,
    type_id: TypeId,

    #[cfg(any(feature = "csr", feature = "ssr"))]
    pub(crate) state: Shared<Option<ComponentState>>,

    parent: Option<Rc<AnyScope>>,
    typed_scope: Rc<dyn Any>,
}

impl fmt::Debug for AnyScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AnyScope<_>")
    }
}

impl<COMP: BaseComponent> From<Scope<COMP>> for AnyScope {
    fn from(scope: Scope<COMP>) -> Self {
        AnyScope {
            id: scope.id,
            type_id: TypeId::of::<COMP>(),

            #[cfg(any(feature = "csr", feature = "ssr"))]
            state: scope.state.clone(),

            parent: scope.parent.clone(),
            typed_scope: Rc::new(scope),
        }
    }
}

impl AnyScope {
    pub(crate) fn get_id(&self) -> usize {
        self.id
    }

    /// Schedules a render.
    pub(crate) fn schedule_render(&self) {
        let runner = RenderRunner {
            state: self.state.clone(),
        };

        scheduler::push_component_render(self.id, move || runner.run());
        scheduler::start();
    }

    pub(crate) fn any_props(&self) -> Option<Rc<dyn Any>> {
        PROPS.with(|m| m.borrow().get(&self.get_id()).cloned())
    }

    /// Returns the parent scope
    pub fn get_parent(&self) -> Option<&AnyScope> {
        self.parent.as_deref()
    }

    /// Returns the type of the linked component
    pub fn get_type_id(&self) -> &TypeId {
        &self.type_id
    }

    /// Attempts to downcast into a typed scope
    ///
    /// # Panics
    ///
    /// If the self value can't be cast into the target type.
    pub fn downcast<COMP: BaseComponent>(&self) -> Scope<COMP> {
        self.try_downcast::<COMP>().unwrap()
    }

    /// Attempts to downcast into a typed scope
    ///
    /// Returns [`None`] if the self value can't be cast into the target type.
    pub fn try_downcast<COMP: BaseComponent>(&self) -> Option<Scope<COMP>> {
        self.typed_scope.downcast_ref::<Scope<COMP>>().cloned()
    }

    /// Attempts to find a parent scope of a certain type
    ///
    /// Returns [`None`] if no parent scope with the specified type was found.
    pub fn find_parent_scope<COMP: BaseComponent>(&self) -> Option<Scope<COMP>> {
        iter::successors(Some(self), |scope| scope.get_parent())
            .find_map(AnyScope::try_downcast::<COMP>)
    }

    /// Accesses a value provided by a parent `ContextProvider` component of the
    /// same type.
    pub fn context<T: Clone + PartialEq + 'static>(
        &self,
        callback: Callback<T>,
    ) -> Option<(T, ContextHandle<T>)> {
        let scope = self.find_parent_scope::<ContextProvider<T>>()?;
        let scope_clone = scope.clone();
        let component = scope.get_component()?;
        Some(component.subscribe_consumer(callback, scope_clone))
    }
}

/// A context which allows sending messages to a component.
pub struct Scope<COMP: BaseComponent> {
    _marker: PhantomData<COMP>,
    parent: Option<Rc<AnyScope>>,

    #[cfg(any(feature = "csr", feature = "ssr"))]
    pub(crate) state: Shared<Option<ComponentState>>,

    pub(crate) id: usize,
}

impl<COMP: BaseComponent> fmt::Debug for Scope<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Scope<_>")
    }
}

impl<COMP: BaseComponent> Clone for Scope<COMP> {
    fn clone(&self) -> Self {
        Scope {
            _marker: PhantomData,

            parent: self.parent.clone(),

            #[cfg(any(feature = "csr", feature = "ssr"))]
            state: self.state.clone(),

            id: self.id,
        }
    }
}

impl<COMP: BaseComponent> Scope<COMP> {
    /// Returns the parent scope
    pub fn get_parent(&self) -> Option<&AnyScope> {
        self.parent.as_deref()
    }

    /// Accesses a value provided by a parent `ContextProvider` component of the
    /// same type.
    pub fn context<T: Clone + PartialEq + 'static>(
        &self,
        callback: Callback<T>,
    ) -> Option<(T, ContextHandle<T>)> {
        AnyScope::from(self.clone()).context(callback)
    }

    /// Returns the linked component if available
    pub fn get_component(&self) -> Option<impl Deref<Target = COMP> + '_> {
        self.arch_get_component()
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt::Write;

    use super::*;
    use crate::html::component::lifecycle::{
        ComponentRenderState, CreateRunner, DestroyRunner, RenderRunner,
    };
    use crate::platform::fmt::BufWriter;
    use crate::platform::pinned::oneshot;
    use crate::scheduler;
    use crate::virtual_dom::Collectable;

    impl<COMP: BaseComponent> Scope<COMP> {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            props: Rc<COMP::Properties>,
            hydratable: bool,
        ) {
            // Rust's Future implementation is stack-allocated and incurs zero runtime-cost.
            //
            // If the content of this channel is ready before it is awaited, it is
            // similar to taking the value from a mutex lock.
            let (tx, rx) = oneshot::channel();
            let state = ComponentRenderState::Ssr { sender: Some(tx) };

            CreateRunner {
                initial_render_state: state,
                props,
                scope: self.clone().to_any(),
                #[cfg(feature = "hydration")]
                prepared_state: None,
                _marker: PhantomData::<COMP>,
            }
            .run();
            RenderRunner {
                state: self.state.clone(),
            }
            .run();

            let collectable = Collectable::for_component::<COMP>();

            if hydratable {
                collectable.write_open_tag(w);
            }

            let html = rx.await.unwrap();

            let self_any_scope = AnyScope::from(self.clone());
            html.render_into_stream(w, &self_any_scope, hydratable)
                .await;

            if let Some(prepared_state) = self.get_component().unwrap().prepare_state() {
                let _ = w.write_str(r#"<script type="application/x-yew-comp-state">"#);
                let _ = w.write_str(&prepared_state);
                let _ = w.write_str(r#"</script>"#);
            }

            if hydratable {
                collectable.write_close_tag(w);
            }

            DestroyRunner {
                state: self.state.clone(),
                parent_to_detach: false,
            }
            .run();
            scheduler::start();
        }
    }
}

#[cfg(any(feature = "ssr", feature = "csr"))]
mod feat_csr_ssr {
    use std::cell::{Ref, RefCell};
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static COMP_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

    impl<COMP: BaseComponent> Scope<COMP> {
        /// Crate a scope with an optional parent scope
        pub(crate) fn new(parent: Option<AnyScope>) -> Self {
            let parent = parent.map(Rc::new);

            let state = Rc::new(RefCell::new(None));

            Scope {
                _marker: PhantomData,

                state,
                parent,

                id: COMP_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            }
        }

        #[rustversion::before(1.63)]
        #[inline]
        pub(super) fn arch_get_component(&self) -> Option<impl Deref<Target = COMP> + '_> {
            self.state.try_borrow().ok().and_then(|state_ref| {
                state_ref.as_ref()?;
                Some(Ref::map(state_ref, |state| {
                    state
                        .as_ref()
                        .and_then(|m| m.downcast_comp_ref::<COMP>())
                        .unwrap()
                }))
            })
        }

        #[rustversion::since(1.63)]
        #[inline]
        pub(super) fn arch_get_component(&self) -> Option<impl Deref<Target = COMP> + '_> {
            self.state.try_borrow().ok().and_then(|state_ref| {
                // Ref::filter_map is only available since 1.63
                Ref::filter_map(state_ref, |state| {
                    state.as_ref().and_then(|m| m.downcast_comp_ref::<COMP>())
                })
                .ok()
            })
        }
    }
}

#[cfg(feature = "csr")]
mod feat_csr {
    use std::cell::Ref;

    use web_sys::Element;

    use super::*;
    use crate::dom_bundle::{BSubtree, Bundle};
    use crate::html::component::lifecycle::{
        ComponentRenderState, CreateRunner, DestroyRunner, PropsUpdateRunner, RenderRunner,
    };
    use crate::html::NodeRef;

    impl AnyScope {
        #[cfg(test)]
        pub(crate) fn test() -> Self {
            Self {
                id: 0,
                type_id: TypeId::of::<()>(),
                state: Rc::default(),
                parent: None,
                typed_scope: Rc::new(()),
            }
        }
    }

    fn schedule_props_update(
        state: Shared<Option<ComponentState>>,
        props: Rc<dyn Any>,
        next_sibling: NodeRef,
    ) {
        PropsUpdateRunner {
            state,
            props: Some(props),
            next_sibling: Some(next_sibling),
        }
        .run();
    }

    impl<COMP> Scope<COMP>
    where
        COMP: BaseComponent,
    {
        /// Mounts a component with `props` to the specified `element` in the DOM.
        pub(crate) fn mount_in_place(
            &self,
            root: BSubtree,
            parent: Element,
            next_sibling: NodeRef,
            internal_ref: NodeRef,
            props: Rc<COMP::Properties>,
        ) {
            let bundle = Bundle::new();
            internal_ref.link(next_sibling.clone());
            let stable_next_sibling = NodeRef::default();
            stable_next_sibling.link(next_sibling);

            PROPS.with(|m| m.borrow_mut().insert(self.id, props.clone()));

            let state = ComponentRenderState::Render {
                bundle,
                root,
                internal_ref,
                parent,
                next_sibling: stable_next_sibling,
            };

            CreateRunner {
                initial_render_state: state,
                props,
                scope: self.clone().to_any(),
                #[cfg(feature = "hydration")]
                prepared_state: None,
                _marker: PhantomData::<COMP>,
            }
            .run();
            RenderRunner {
                state: self.state.clone(),
            }
            .run();
        }

        pub(crate) fn reuse(&self, props: Rc<COMP::Properties>, next_sibling: NodeRef) {
            schedule_props_update(self.state.clone(), props, next_sibling)
        }
    }

    pub(crate) trait Scoped {
        fn to_any(&self) -> AnyScope;
        /// Get the render state if it hasn't already been destroyed
        fn render_state(&self) -> Option<Ref<'_, ComponentRenderState>>;
        /// Shift the node associated with this scope to a new place
        fn shift_node(&self, parent: Element, next_sibling: NodeRef);
        /// Process an event to destroy a component
        fn destroy(self, parent_to_detach: bool);
        fn destroy_boxed(self: Box<Self>, parent_to_detach: bool);
    }

    impl<COMP: BaseComponent> Scoped for Scope<COMP> {
        fn to_any(&self) -> AnyScope {
            self.clone().into()
        }

        fn render_state(&self) -> Option<Ref<'_, ComponentRenderState>> {
            let state_ref = self.state.borrow();

            // check that component hasn't been destroyed
            state_ref.as_ref()?;

            Some(Ref::map(state_ref, |state_ref| {
                &state_ref.as_ref().unwrap().render_state
            }))
        }

        /// Process an event to destroy a component
        fn destroy(self, parent_to_detach: bool) {
            DestroyRunner {
                state: self.state,
                parent_to_detach,
            }
            .run()
        }

        fn destroy_boxed(self: Box<Self>, parent_to_detach: bool) {
            self.destroy(parent_to_detach)
        }

        fn shift_node(&self, parent: Element, next_sibling: NodeRef) {
            let mut state_ref = self.state.borrow_mut();
            if let Some(render_state) = state_ref.as_mut() {
                render_state.render_state.shift(parent, next_sibling)
            }
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
    use crate::html::component::lifecycle::{ComponentRenderState, CreateRunner, RenderRunner};
    use crate::html::NodeRef;
    use crate::virtual_dom::Collectable;

    impl<COMP> Scope<COMP>
    where
        COMP: BaseComponent,
    {
        /// Hydrates the component.
        ///
        /// Returns a pending NodeRef of the next sibling.
        ///
        /// # Note
        ///
        /// This method is expected to collect all the elements belongs to the current component
        /// immediately.
        pub(crate) fn hydrate_in_place(
            &self,
            root: BSubtree,
            parent: Element,
            fragment: &mut Fragment,
            internal_ref: NodeRef,
            props: Rc<COMP::Properties>,
        ) {
            // This is very helpful to see which component is failing during hydration
            // which means this component may not having a stable layout / differs between
            // client-side and server-side.
            tracing::trace!(
                component.id = self.id,
                "hydration(type = {})",
                std::any::type_name::<COMP>()
            );

            let collectable = Collectable::for_component::<COMP>();

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

            let state = ComponentRenderState::Hydration {
                parent,
                root,
                internal_ref,
                next_sibling: NodeRef::new_debug_trapped(),
                fragment,
            };

            CreateRunner {
                initial_render_state: state,
                props,
                scope: self.clone().to_any(),
                prepared_state,
                _marker: PhantomData::<COMP>,
            }
            .run();
            RenderRunner {
                state: self.state.clone(),
            }
            .run();
        }
    }
}
