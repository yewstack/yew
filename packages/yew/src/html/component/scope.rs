//! Component scope module

use super::lifecycle::{
    CompStateInner, ComponentState, CreateRunner, DestroyRunner, RenderRunner, Rendered,
    UpdateEvent, UpdateRunner,
};
use super::BaseComponent;
use crate::callback::Callback;
use crate::context::{ContextHandle, ContextProvider};
use crate::html::NodeRef;
use crate::scheduler::{self, Shared};
use crate::virtual_dom::{insert_node, VComp, VNode};
use gloo_utils::document;
use std::any::TypeId;
use std::cell::{Ref, RefCell};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::{fmt, iter};
use web_sys::{Element, Node};

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

/// Untyped scope used for accessing parent scope
#[derive(Clone)]
pub struct AnyScope {
    type_id: TypeId,
    parent: Option<Rc<AnyScope>>,
    state: Shared<Option<ComponentState>>,
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
            parent: scope.parent,
            state: scope.state,
        }
    }
}

impl AnyScope {
    #[cfg(all(test, target_arch = "wasm32"))]
    pub(crate) fn test() -> Self {
        Self {
            type_id: TypeId::of::<()>(),
            parent: None,
            state: Rc::new(RefCell::new(None)),
        }
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
    pub fn downcast<COMP: BaseComponent>(self) -> Scope<COMP> {
        let state = self.state.borrow();

        state
            .as_ref()
            .map(|m| {
                m.inner
                    .as_any()
                    .downcast_ref::<CompStateInner<COMP>>()
                    .unwrap()
                    .context
                    .link()
                    .clone()
            })
            .unwrap()
    }

    pub(crate) fn find_parent_scope<C: BaseComponent>(&self) -> Option<Scope<C>> {
        let expected_type_id = TypeId::of::<C>();
        iter::successors(Some(self), |scope| scope.get_parent())
            .filter(|scope| scope.get_type_id() == &expected_type_id)
            .cloned()
            .map(AnyScope::downcast::<C>)
            .next()
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

pub(crate) trait Scoped {
    fn to_any(&self) -> AnyScope;
    fn root_vnode(&self) -> Option<Ref<'_, VNode>>;
    fn destroy(&mut self, parent_to_detach: bool);
    fn shift_node(&self, parent: Element, next_sibling: NodeRef);
}

impl<COMP: BaseComponent> Scoped for Scope<COMP> {
    fn to_any(&self) -> AnyScope {
        self.clone().into()
    }

    fn root_vnode(&self) -> Option<Ref<'_, VNode>> {
        let state_ref = self.state.borrow();

        // check that component hasn't been destroyed
        state_ref.as_ref()?;

        Some(Ref::map(state_ref, |state_ref| {
            state_ref
                .as_ref()
                .unwrap()
                .rendered
                .root_vnode()
                .unwrap_or(VNode::EMPTY)
        }))
    }

    /// Process an event to destroy a component
    fn destroy(&mut self, parent_to_detach: bool) {
        scheduler::push_component_destroy(DestroyRunner {
            state: self.state.clone(),
            parent_to_detach,
        });
        // Not guaranteed to already have the scheduler started
        scheduler::start();
    }

    fn shift_node(&self, parent: Element, next_sibling: NodeRef) {
        scheduler::push_component_update(UpdateRunner {
            state: self.state.clone(),
            event: UpdateEvent::Shift(parent, next_sibling),
        });
    }
}

/// A context which allows sending messages to a component.
pub struct Scope<COMP: BaseComponent> {
    _marker: PhantomData<COMP>,
    parent: Option<Rc<AnyScope>>,
    pub(crate) pending_messages: MsgQueue<COMP::Message>,
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
            pending_messages: self.pending_messages.clone(),
            parent: self.parent.clone(),
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

    /// Returns the linked component if available
    pub fn get_component(&self) -> Option<impl Deref<Target = COMP> + '_> {
        self.state.try_borrow().ok().and_then(|state_ref| {
            state_ref.as_ref()?;
            Some(Ref::map(state_ref, |state| {
                &state
                    .as_ref()
                    .unwrap()
                    .inner
                    .as_any()
                    .downcast_ref::<CompStateInner<COMP>>()
                    .unwrap()
                    .component
            }))
        })
    }

    pub(crate) fn new(parent: Option<AnyScope>) -> Self {
        let parent = parent.map(Rc::new);
        let state = Rc::new(RefCell::new(None));
        let pending_messages = MsgQueue::new();

        Scope {
            _marker: PhantomData,
            pending_messages,
            state,
            parent,

            id: VComp::next_id(),
        }
    }

    /// Mounts a component with `props` to the specified `element` in the DOM.
    pub(crate) fn mount_in_place(
        &self,
        parent: Element,
        next_sibling: NodeRef,
        node_ref: NodeRef,
        props: Rc<COMP::Properties>,
    ) {
        #[cfg(debug_assertions)]
        crate::virtual_dom::vcomp::log_event(self.id, "create placeholder");
        let placeholder = {
            let placeholder: Node = document().create_text_node("").into();
            insert_node(&placeholder, &parent, next_sibling.get().as_ref());
            node_ref.set(Some(placeholder.clone()));
            VNode::VRef(placeholder)
        };

        let rendered = Rendered::Render {
            root_node: placeholder,
            node_ref,
            next_sibling,
            parent,
        };

        scheduler::push_component_create(
            self.id,
            CreateRunner {
                rendered,
                props,
                scope: self.clone(),
            },
            RenderRunner {
                state: self.state.clone(),
            },
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
        crate::virtual_dom::vcomp::log_event(self.id, "reuse");

        self.push_update(UpdateEvent::Properties(props, node_ref, next_sibling));
    }

    fn push_update(&self, event: UpdateEvent) {
        scheduler::push_component_update(UpdateRunner {
            state: self.state.clone(),
            event,
        });
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
    pub fn send_message_batch(&self, mut messages: Vec<COMP::Message>) {
        let msg_len = messages.len();

        // The queue was empty, so we queue the update
        if self.pending_messages.append(&mut messages) == msg_len {
            self.push_update(UpdateEvent::Message);
        }
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
        self.to_any().context(callback)
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;
    use futures::channel::oneshot;

    impl<COMP: BaseComponent> Scope<COMP> {
        pub(crate) async fn render_to_string(
            &self,
            w: &mut String,
            props: Rc<COMP::Properties>,
            hydratable: bool,
        ) {
            let (tx, rx) = oneshot::channel();

            let rendered = Rendered::Ssr { sender: Some(tx) };

            scheduler::push_component_create(
                self.id,
                CreateRunner {
                    rendered,
                    props,
                    scope: self.clone(),
                },
                RenderRunner {
                    state: self.state.clone(),
                },
            );
            scheduler::start();

            if hydratable {
                #[cfg(debug_assertions)]
                w.push_str(&format!("<!--<[{}]>-->", std::any::type_name::<COMP>()));

                #[cfg(not(debug_assertions))]
                w.push_str("<!--<[]>-->");
            }

            let html = rx.await.unwrap();

            let self_any_scope = self.to_any();
            html.render_to_string(w, &self_any_scope, hydratable).await;

            if hydratable {
                #[cfg(debug_assertions)]
                w.push_str(&format!("<!--</[{}]>-->", std::any::type_name::<COMP>()));

                #[cfg(not(debug_assertions))]
                w.push_str("<!--</[]>-->");
            }

            scheduler::push_component_destroy(DestroyRunner {
                state: self.state.clone(),
                parent_to_detach: false,
            });

            scheduler::start();
        }
    }
}
#[cfg_attr(documenting, doc(cfg(any(target_arch = "wasm32", feature = "tokio"))))]
#[cfg(any(target_arch = "wasm32", feature = "tokio"))]
mod feat_io {
    use std::future::Future;

    use super::*;
    use crate::io_coop::spawn_local;

    impl<COMP: BaseComponent> Scope<COMP> {
        /// This method creates a [`Callback`] which returns a Future which
        /// returns a message to be sent back to the component's event
        /// loop.
        ///
        /// # Panics
        /// If the future panics, then the promise will not resolve, and
        /// will leak.
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

        /// This method processes a Future that returns a message and sends it back to the component's
        /// loop.
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

        /// Registers a Future that resolves to multiple messages.
        /// # Panics
        /// If the future panics, then the promise will not resolve, and will leak.
        pub fn send_future_batch<F>(&self, future: F)
        where
            F: Future<Output = Vec<COMP::Message>> + 'static,
        {
            let link = self.clone();
            let js_future = async move {
                let messages: Vec<COMP::Message> = future.await;
                link.send_message_batch(messages);
            };
            spawn_local(js_future);
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use crate::virtual_dom::Fragment;

    impl<COMP: BaseComponent> Scope<COMP> {
        /// Hydrates the component.
        ///
        /// Returns a pending NodeRef of the next sibling.
        ///
        /// # Note
        ///
        /// This method is expected to collect all the elements belongs to the current component
        /// immediately.
        ///
        /// We don't remove the comment node at the moment as it's needed to maintain the
        /// structure.
        pub(crate) fn hydrate_in_place(
            &self,
            parent: Element,
            fragment: &mut Fragment,
            node_ref: NodeRef,
            props: Rc<COMP::Properties>,
        ) {
            // This is very helpful to see which component is failing during hydration
            // which means this component may not having a stable layout / differs between
            // client-side and server-side.
            #[cfg(all(debug_assertions, feature = "trace_hydration"))]
            gloo::console::trace!(
                "queuing hydration of: {}(ID: {})",
                std::any::type_name::<COMP>(),
                self.id
            );

            let fragment =
                Fragment::collect_between(fragment, &parent, "<[", "</[", "]>", "component");
            node_ref.set(fragment.front().cloned());
            let next_sibling = NodeRef::default();

            let rendered = Rendered::Hydration {
                parent,
                node_ref,
                next_sibling,
                fragment,
            };

            scheduler::push_component_create(
                self.id,
                CreateRunner {
                    rendered,
                    props,
                    scope: self.clone(),
                },
                RenderRunner {
                    state: self.state.clone(),
                },
            );

            // Not guaranteed to already have the scheduler started
            scheduler::start();
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
