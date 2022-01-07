//! Component scope module

use super::{
    lifecycle::{
        ComponentState, CreateRunner, DestroyRunner, RenderRunner, RenderedRunner, UpdateEvent,
        UpdateRunner,
    },
    BaseComponent,
};
use crate::callback::Callback;
use crate::context::{ContextHandle, ContextProvider};
use crate::html::NodeRef;
use crate::scheduler::{self, Shared};
use crate::virtual_dom::{insert_node, VNode};
use gloo_utils::document;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::{fmt, iter};
use web_sys::{Element, Node};

/// Untyped scope used for accessing parent scope
#[derive(Debug, Clone)]
pub struct AnyScope {
    type_id: TypeId,
    parent: Option<Rc<AnyScope>>,
    state: Rc<dyn Any>,

    // Used for debug logging
    #[cfg(debug_assertions)]
    pub(crate) vcomp_id: u64,
}

impl<COMP: BaseComponent> From<Scope<COMP>> for AnyScope {
    fn from(scope: Scope<COMP>) -> Self {
        AnyScope {
            type_id: TypeId::of::<COMP>(),
            parent: scope.parent,
            state: scope.state,

            #[cfg(debug_assertions)]
            vcomp_id: scope.vcomp_id,
        }
    }
}

impl AnyScope {
    #[cfg(test)]
    pub(crate) fn test() -> Self {
        Self {
            type_id: TypeId::of::<()>(),
            parent: None,
            state: Rc::new(()),

            #[cfg(debug_assertions)]
            vcomp_id: 0,
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
        let state = self
            .state
            .downcast::<RefCell<Option<ComponentState<COMP>>>>()
            .expect("unexpected component type");

        #[cfg(debug_assertions)]
        let vcomp_id = state
            .borrow()
            .as_ref()
            .map(|s| s.vcomp_id)
            .unwrap_or_default();

        Scope {
            parent: self.parent,
            state,

            #[cfg(debug_assertions)]
            vcomp_id,
        }
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
    fn destroy(&mut self);
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
            &state_ref.as_ref().unwrap().root_node
        }))
    }

    /// Process an event to destroy a component
    fn destroy(&mut self) {
        scheduler::push_component_destroy(DestroyRunner {
            state: self.state.clone(),
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
    parent: Option<Rc<AnyScope>>,
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,

    // Used for debug logging
    #[cfg(debug_assertions)]
    pub(crate) vcomp_id: u64,
}

impl<COMP: BaseComponent> fmt::Debug for Scope<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Scope<_>")
    }
}

impl<COMP: BaseComponent> Clone for Scope<COMP> {
    fn clone(&self) -> Self {
        Scope {
            parent: self.parent.clone(),
            state: self.state.clone(),

            #[cfg(debug_assertions)]
            vcomp_id: self.vcomp_id,
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
                state.as_ref().unwrap().component.as_ref()
            }))
        })
    }

    pub(crate) fn new(parent: Option<AnyScope>) -> Self {
        let parent = parent.map(Rc::new);
        let state = Rc::new(RefCell::new(None));

        #[cfg(debug_assertions)]
        let vcomp_id = parent.as_ref().map(|p| p.vcomp_id).unwrap_or_default();

        Scope {
            state,
            parent,

            #[cfg(debug_assertions)]
            vcomp_id,
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
        crate::virtual_dom::vcomp::log_event(self.vcomp_id, "create placeholder");
        let placeholder = {
            let placeholder: Node = document().create_text_node("").into();
            insert_node(&placeholder, &parent, next_sibling.get().as_ref());
            node_ref.set(Some(placeholder.clone()));
            VNode::VRef(placeholder)
        };

        scheduler::push_component_create(
            CreateRunner {
                parent: Some(parent),
                next_sibling,
                placeholder,
                node_ref,
                props,
                scope: self.clone(),
                #[cfg(feature = "ssr")]
                html_sender: None,
            },
            RenderRunner {
                state: self.state.clone(),
            },
            RenderedRunner {
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
        crate::virtual_dom::vcomp::log_event(self.vcomp_id, "reuse");

        self.push_update(UpdateEvent::Properties(props, node_ref, next_sibling));
    }

    fn push_update(&self, event: UpdateEvent<COMP>) {
        scheduler::push_component_update(UpdateRunner {
            state: self.state.clone(),
            event,
        });
        // Not guaranteed to already have the scheduler started
        scheduler::start();
    }

    /// Send a message to the component.
    ///
    /// Please be aware that currently this method synchronously
    /// schedules a call to the [Component](crate::html::Component) interface.
    pub fn send_message<T>(&self, msg: T)
    where
        T: Into<COMP::Message>,
    {
        self.push_update(UpdateEvent::Message(msg.into()));
    }

    /// Send a batch of messages to the component.
    ///
    /// This is useful for reducing re-renders of the components
    /// because the messages are handled together and the view
    /// function is called only once if needed.
    ///
    /// Please be aware that currently this method synchronously
    /// schedules calls to the [Component](crate::html::Component) interface.
    pub fn send_message_batch(&self, messages: Vec<COMP::Message>) {
        // There is no reason to schedule empty batches.
        // This check is especially handy for the batch_callback method.
        if messages.is_empty() {
            return;
        }

        self.push_update(UpdateEvent::MessageBatch(messages));
    }

    /// Creates a `Callback` which will send a message to the linked
    /// component's update method when invoked.
    ///
    /// Please be aware that currently the result of this callback
    /// synchronously schedules a call to the [Component](Component)
    /// interface.
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
    ///
    /// Please be aware that currently the results of these callbacks
    /// will synchronously schedule calls to the
    /// [Component](Component) interface.
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
        pub(crate) async fn render_to_string(&self, w: &mut String, props: Rc<COMP::Properties>) {
            let (tx, rx) = oneshot::channel();

            scheduler::push_component_create(
                CreateRunner {
                    parent: None,
                    next_sibling: NodeRef::default(),
                    placeholder: VNode::default(),
                    node_ref: NodeRef::default(),
                    props,
                    scope: self.clone(),
                    html_sender: Some(tx),
                },
                RenderRunner {
                    state: self.state.clone(),
                },
                RenderedRunner {
                    state: self.state.clone(),
                },
            );
            scheduler::start();

            let html = rx.await.unwrap();

            let self_any_scope = self.to_any();
            html.render_to_string(w, &self_any_scope).await;

            scheduler::push_component_destroy(DestroyRunner {
                state: self.state.clone(),
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
