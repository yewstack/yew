//! Component scope module

#[cfg(any(feature = "csr", feature = "ssr"))]
use crate::scheduler::Shared;
#[cfg(any(feature = "csr", feature = "ssr"))]
use std::cell::RefCell;

#[cfg(any(feature = "csr", feature = "ssr"))]
use super::lifecycle::{ComponentState, UpdateEvent, UpdateRunner};
use super::BaseComponent;

use crate::callback::Callback;
use crate::context::{ContextHandle, ContextProvider};
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::{fmt, iter};

/// Untyped scope used for accessing parent scope
#[derive(Clone)]
pub struct AnyScope {
    type_id: TypeId,
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
            type_id: TypeId::of::<COMP>(),
            parent: scope.parent.clone(),
            typed_scope: Rc::new(scope),
        }
    }
}

impl AnyScope {
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
    pub(crate) pending_messages: MsgQueue<COMP::Message>,

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

            #[cfg(any(feature = "csr", feature = "ssr"))]
            pending_messages: self.pending_messages.clone(),
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

    /// Creates a `Callback` which will send a message to the linked
    /// component's update method when invoked.
    pub fn callback<F, IN, M>(&self, function: F) -> Callback<IN>
    where
        M: Into<COMP::Message>,
        F: Fn(IN) -> M + 'static,
    {
        let scope = self.clone();
        let closure = move |input| {
            let output = function(input);
            scope.send_message(output);
        };
        Callback::from(closure)
    }

    /// Creates a `Callback` which will send a batch of messages back
    /// to the linked component's update method when invoked.
    ///
    /// The callback function's return type is generic to allow for dealing with both
    /// `Option` and `Vec` nicely. `Option` can be used when dealing with a callback that
    /// might not need to send an update.
    ///
    /// ```ignore
    /// link.batch_callback(|_| vec![Msg::A, Msg::B]);
    /// link.batch_callback(|_| Some(Msg::A));
    /// ```
    pub fn batch_callback<F, IN, OUT>(&self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> OUT + 'static,
        OUT: SendAsMessage<COMP>,
    {
        let scope = self.clone();
        let closure = move |input| {
            let messages = function(input);
            messages.send(&scope);
        };
        closure.into()
    }

    /// Accesses a value provided by a parent `ContextProvider` component of the
    /// same type.
    pub fn context<T: Clone + PartialEq + 'static>(
        &self,
        callback: Callback<T>,
    ) -> Option<(T, ContextHandle<T>)> {
        AnyScope::from(self.clone()).context(callback)
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;
    use crate::scheduler;
    use futures::channel::oneshot;

    use crate::html::component::lifecycle::{
        ComponentRenderState, CreateRunner, DestroyRunner, RenderRunner,
    };

    use crate::virtual_dom::Collectable;

    impl<COMP: BaseComponent> Scope<COMP> {
        pub(crate) async fn render_to_string(
            self,
            w: &mut String,
            props: Rc<COMP::Properties>,
            hydratable: bool,
        ) {
            let (tx, rx) = oneshot::channel();
            let state = ComponentRenderState::Ssr { sender: Some(tx) };

            scheduler::push_component_create(
                self.id,
                Box::new(CreateRunner {
                    initial_render_state: state,
                    props,
                    scope: self.clone(),
                }),
                Box::new(RenderRunner {
                    state: self.state.clone(),
                }),
            );
            scheduler::start();

            #[cfg(debug_assertions)]
            let collectable = Collectable::Component(std::any::type_name::<COMP>());

            #[cfg(not(debug_assertions))]
            let collectable = Collectable::Component;

            if hydratable {
                collectable.write_open_tag(w);
            }

            let html = rx.await.unwrap();

            let self_any_scope = AnyScope::from(self.clone());
            html.render_to_string(w, &self_any_scope, hydratable).await;

            if hydratable {
                collectable.write_close_tag(w);
            }

            scheduler::push_component_destroy(Box::new(DestroyRunner {
                state: self.state.clone(),

                #[cfg(feature = "csr")]
                parent_to_detach: false,
            }));
            scheduler::start();
        }
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
mod feat_no_csr_ssr {
    use super::*;

    // Skeleton code to provide public methods when no renderer are enabled.
    impl<COMP: BaseComponent> Scope<COMP> {
        /// Returns the linked component if available
        pub fn get_component(&self) -> Option<impl Deref<Target = COMP> + '_> {
            Option::<&COMP>::None
        }

        /// Send a message to the component.
        pub fn send_message<T>(&self, _msg: T)
        where
            T: Into<COMP::Message>,
        {
        }

        /// Send a batch of messages to the component.
        ///
        /// This is slightly more efficient than calling [`send_message`](Self::send_message)
        /// in a loop.
        pub fn send_message_batch(&self, _messages: Vec<COMP::Message>) {}
    }
}

#[cfg(any(feature = "ssr", feature = "csr"))]
mod feat_csr_ssr {
    use super::*;
    use crate::scheduler::{self, Shared};
    use std::cell::Ref;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug)]
    pub(crate) struct MsgQueue<Msg>(Shared<Vec<Msg>>);

    impl<Msg> MsgQueue<Msg> {
        pub fn new() -> Self {
            MsgQueue(Rc::default())
        }

        pub fn push(&self, msg: Msg) -> usize {
            let mut inner = self.0.borrow_mut();
            inner.push(msg);

            inner.len()
        }

        pub fn append(&self, other: &mut Vec<Msg>) -> usize {
            let mut inner = self.0.borrow_mut();
            inner.append(other);

            inner.len()
        }

        pub fn drain(&self) -> Vec<Msg> {
            let mut other_queue = Vec::new();
            let mut inner = self.0.borrow_mut();

            std::mem::swap(&mut *inner, &mut other_queue);

            other_queue
        }
    }

    impl<Msg> Clone for MsgQueue<Msg> {
        fn clone(&self) -> Self {
            MsgQueue(self.0.clone())
        }
    }

    static COMP_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

    impl<COMP: BaseComponent> Scope<COMP> {
        /// Crate a scope with an optional parent scope
        pub(crate) fn new(parent: Option<AnyScope>) -> Self {
            let parent = parent.map(Rc::new);

            let state = Rc::new(RefCell::new(None));

            let pending_messages = MsgQueue::new();

            Scope {
                _marker: PhantomData,

                pending_messages,

                state,
                parent,

                id: COMP_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            }
        }

        /// Returns the linked component if available
        pub fn get_component(&self) -> Option<impl Deref<Target = COMP> + '_> {
            self.state.try_borrow().ok().and_then(|state_ref| {
                state_ref.as_ref()?;
                // TODO: Replace unwrap with Ref::filter_map once it becomes stable.
                Some(Ref::map(state_ref, |state| {
                    state
                        .as_ref()
                        .and_then(|m| m.downcast_comp_ref::<COMP>())
                        .unwrap()
                }))
            })
        }

        pub(super) fn push_update(&self, event: UpdateEvent) {
            scheduler::push_component_update(Box::new(UpdateRunner {
                state: self.state.clone(),
                event,
            }));
            // Not guaranteed to already have the scheduler started
            scheduler::start();
        }

        /// Send a message to the component.
        pub fn send_message<T>(&self, msg: T)
        where
            T: Into<COMP::Message>,
        {
            // We are the first message in queue, so we queue the update.
            if self.pending_messages.push(msg.into()) == 1 {
                self.push_update(UpdateEvent::Message);
            }
        }

        /// Send a batch of messages to the component.
        ///
        /// This is slightly more efficient than calling [`send_message`](Self::send_message)
        /// in a loop.
        pub fn send_message_batch(&self, mut messages: Vec<COMP::Message>) {
            let msg_len = messages.len();

            // The queue was empty, so we queue the update
            if self.pending_messages.append(&mut messages) == msg_len {
                self.push_update(UpdateEvent::Message);
            }
        }
    }
}

#[cfg(any(feature = "ssr", feature = "csr"))]
pub(crate) use feat_csr_ssr::*;

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;
    use crate::dom_bundle::{BSubtree, Bundle};
    use crate::html::component::lifecycle::{
        ComponentRenderState, CreateRunner, DestroyRunner, RenderRunner,
    };
    use crate::html::NodeRef;
    use crate::scheduler;
    use std::cell::Ref;
    use web_sys::Element;

    impl AnyScope {
        #[cfg(test)]
        pub(crate) fn test() -> Self {
            Self {
                type_id: TypeId::of::<()>(),
                parent: None,
                typed_scope: Rc::new(()),
            }
        }
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
            node_ref: NodeRef,
            props: Rc<COMP::Properties>,
        ) {
            let bundle = Bundle::new();
            node_ref.link(next_sibling.clone());
            let state = ComponentRenderState::Render {
                bundle,
                root,
                node_ref,
                parent,
                next_sibling,
            };

            scheduler::push_component_create(
                self.id,
                Box::new(CreateRunner {
                    initial_render_state: state,
                    props,
                    scope: self.clone(),
                }),
                Box::new(RenderRunner {
                    state: self.state.clone(),
                }),
            );
            // Not guaranteed to already have the scheduler started
            scheduler::start();
        }

        pub(crate) fn reuse(
            &self,
            props: Rc<COMP::Properties>,
            node_ref: NodeRef,
            next_sibling: NodeRef,
        ) {
            #[cfg(debug_assertions)]
            super::super::log_event(self.id, "reuse");

            self.push_update(UpdateEvent::Properties(props, node_ref, next_sibling));
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
            scheduler::push_component_destroy(Box::new(DestroyRunner {
                state: self.state,
                parent_to_detach,
            }));
            // Not guaranteed to already have the scheduler started
            scheduler::start();
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

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use crate::dom_bundle::{BSubtree, Fragment};
    use crate::html::component::lifecycle::{ComponentRenderState, CreateRunner, RenderRunner};
    use crate::html::NodeRef;
    use crate::scheduler;
    use crate::virtual_dom::Collectable;

    use web_sys::Element;

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
            node_ref: NodeRef,
            props: Rc<COMP::Properties>,
        ) {
            // This is very helpful to see which component is failing during hydration
            // which means this component may not having a stable layout / differs between
            // client-side and server-side.
            #[cfg(all(debug_assertions, feature = "trace_hydration"))]
            gloo::console::trace!(format!(
                "queuing hydration of: {}(ID: {:?})",
                std::any::type_name::<COMP>(),
                self.id
            ));

            #[cfg(debug_assertions)]
            let collectable = Collectable::Component(std::any::type_name::<COMP>());
            #[cfg(not(debug_assertions))]
            let collectable = Collectable::Component;

            let fragment = Fragment::collect_between(fragment, &collectable, &parent);
            node_ref.set(fragment.front().cloned());
            let next_sibling = NodeRef::default();

            let state = ComponentRenderState::Hydration {
                root,
                parent,
                node_ref,
                next_sibling,
                fragment,
            };

            scheduler::push_component_create(
                self.id,
                Box::new(CreateRunner {
                    initial_render_state: state,
                    props,
                    scope: self.clone(),
                }),
                Box::new(RenderRunner {
                    state: self.state.clone(),
                }),
            );

            // Not guaranteed to already have the scheduler started
            scheduler::start();
        }
    }
}
#[cfg(feature = "csr")]
pub(crate) use feat_csr::*;

#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
mod feat_io {
    use std::future::Future;

    use super::*;
    use crate::io_coop::spawn_local;

    impl<COMP: BaseComponent> Scope<COMP> {
        /// This method creates a [`Callback`] which, when emitted, asynchronously awaits the
        /// message returned from the passed function before sending it to the linked component.
        ///
        /// # Panics
        /// If the future panics, then the promise will not resolve, and will leak.
        pub fn callback_future<FN, FU, IN, M>(&self, function: FN) -> Callback<IN>
        where
            M: Into<COMP::Message>,
            FU: Future<Output = M> + 'static,
            FN: Fn(IN) -> FU + 'static,
        {
            let link = self.clone();

            let closure = move |input: IN| {
                let future: FU = function(input);
                link.send_future(future);
            };

            closure.into()
        }

        /// This method asynchronously awaits a [Future] that returns a message and sends it
        /// to the linked component.
        ///
        /// # Panics
        /// If the future panics, then the promise will not resolve, and will leak.
        pub fn send_future<F, M>(&self, future: F)
        where
            M: Into<COMP::Message>,
            F: Future<Output = M> + 'static,
        {
            let link = self.clone();
            let js_future = async move {
                let message: COMP::Message = future.await.into();
                link.send_message(message);
            };
            spawn_local(js_future);
        }

        /// Asynchronously send a batch of messages to a component. This asynchronously awaits the
        /// passed [Future], before sending the message batch to the linked component.
        ///
        /// # Panics
        /// If the future panics, then the promise will not resolve, and will leak.
        pub fn send_future_batch<F>(&self, future: F)
        where
            F: Future + 'static,
            F::Output: SendAsMessage<COMP>,
        {
            let link = self.clone();
            let js_future = async move {
                future.await.send(&link);
            };
            spawn_local(js_future);
        }
    }
}

/// Defines a message type that can be sent to a component.
/// Used for the return value of closure given to [Scope::batch_callback](struct.Scope.html#method.batch_callback).
pub trait SendAsMessage<COMP: BaseComponent> {
    /// Sends the message to the given component's scope.
    /// See [Scope::batch_callback](struct.Scope.html#method.batch_callback).
    fn send(self, scope: &Scope<COMP>);
}

impl<COMP> SendAsMessage<COMP> for Option<COMP::Message>
where
    COMP: BaseComponent,
{
    fn send(self, scope: &Scope<COMP>) {
        if let Some(msg) = self {
            scope.send_message(msg);
        }
    }
}

impl<COMP> SendAsMessage<COMP> for Vec<COMP::Message>
where
    COMP: BaseComponent,
{
    fn send(self, scope: &Scope<COMP>) {
        scope.send_message_batch(self);
    }
}
