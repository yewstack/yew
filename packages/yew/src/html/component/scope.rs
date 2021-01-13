//! Component scope module

use super::{
    lifecycle::{
        ComponentLifecycleEvent, ComponentRunnable, ComponentState, CreateEvent, UpdateEvent,
    },
    Component,
};
use crate::callback::Callback;
use crate::html::NodeRef;
use crate::scheduler::{scheduler, Shared};
use crate::utils::document;
use crate::virtual_dom::{insert_node, VNode};
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use web_sys::{Element, Node};

/// Untyped scope used for accessing parent scope
#[derive(Debug, Clone)]
pub struct AnyScope {
    pub(crate) type_id: TypeId,
    pub(crate) parent: Option<Rc<AnyScope>>,
    pub(crate) state: Rc<dyn Any>,
}

impl<COMP: Component> From<Scope<COMP>> for AnyScope {
    fn from(scope: Scope<COMP>) -> Self {
        AnyScope {
            type_id: TypeId::of::<COMP>(),
            parent: scope.parent,
            state: Rc::new(scope.state),
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
    pub fn downcast<COMP: Component>(self) -> Scope<COMP> {
        Scope {
            parent: self.parent,
            state: self
                .state
                .downcast_ref::<Shared<Option<ComponentState<COMP>>>>()
                .expect("unexpected component type")
                .clone(),
        }
    }
}

pub(crate) trait Scoped {
    fn to_any(&self) -> AnyScope;
    fn root_vnode(&self) -> Option<Ref<'_, VNode>>;
    fn destroy(&mut self);
}

impl<COMP: Component> Scoped for Scope<COMP> {
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
        self.process(ComponentLifecycleEvent::Destroy);
    }
}

/// A context which allows sending messages to a component.
pub struct Scope<COMP: Component> {
    parent: Option<Rc<AnyScope>>,
    state: Shared<Option<ComponentState<COMP>>>,
}

impl<COMP: Component> fmt::Debug for Scope<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Scope<_>")
    }
}

impl<COMP: Component> Clone for Scope<COMP> {
    fn clone(&self) -> Self {
        Scope {
            parent: self.parent.clone(),
            state: self.state.clone(),
        }
    }
}

impl<COMP: Component> Scope<COMP> {
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
        Scope { parent, state }
    }

    /// Mounts a component with `props` to the specified `element` in the DOM.
    pub(crate) fn mount_in_place(
        self,
        parent: Element,
        next_sibling: NodeRef,
        node_ref: NodeRef,
        props: COMP::Properties,
    ) -> Scope<COMP> {
        let placeholder = {
            let placeholder: Node = document().create_text_node("").into();
            insert_node(&placeholder, &parent, next_sibling.get());
            node_ref.set(Some(placeholder.clone()));
            VNode::VRef(placeholder)
        };

        self.schedule(UpdateEvent::First.into());
        self.process(ComponentLifecycleEvent::Create(CreateEvent {
            parent,
            next_sibling,
            placeholder,
            node_ref,
            props,
            scope: self.clone(),
        }));

        self
    }

    pub(crate) fn reuse(&self, props: COMP::Properties, node_ref: NodeRef, next_sibling: NodeRef) {
        self.process(UpdateEvent::Properties(props, node_ref, next_sibling).into());
    }

    pub(crate) fn process(&self, event: ComponentLifecycleEvent<COMP>) {
        let scheduler = scheduler();
        scheduler.component.push(
            event.as_runnable_type(),
            Box::new(ComponentRunnable {
                state: self.state.clone(),
                event,
            }),
        );
        scheduler.start();
    }

    fn schedule(&self, event: ComponentLifecycleEvent<COMP>) {
        let scheduler = &scheduler().component;
        scheduler.push(
            event.as_runnable_type(),
            Box::new(ComponentRunnable {
                state: self.state.clone(),
                event,
            }),
        );
    }

    /// Send a message to the component.
    ///
    /// Please be aware that currently this method synchronously
    /// schedules a call to the [Component](Component) interface.
    pub fn send_message<T>(&self, msg: T)
    where
        T: Into<COMP::Message>,
    {
        self.process(UpdateEvent::Message(msg.into()).into());
    }

    /// Send a batch of messages to the component.
    ///
    /// This is useful for reducing re-renders of the components
    /// because the messages are handled together and the view
    /// function is called only once if needed.
    ///
    /// Please be aware that currently this method synchronously
    /// schedules calls to the [Component](Component) interface.
    pub fn send_message_batch(&self, messages: Vec<COMP::Message>) {
        // There is no reason to schedule empty batches.
        // This check is especially handy for the batch_callback method.
        if messages.is_empty() {
            return;
        }

        self.process(UpdateEvent::MessageBatch(messages).into());
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
        closure.into()
    }

    /// Creates a `Callback` from an `FnOnce` which will send a message
    /// to the linked component's update method when invoked.
    ///
    /// Please be aware that currently the result of this callback
    /// will synchronously schedule calls to the
    /// [Component](Component) interface.
    pub fn callback_once<F, IN, M>(&self, function: F) -> Callback<IN>
    where
        M: Into<COMP::Message>,
        F: FnOnce(IN) -> M + 'static,
    {
        let scope = self.clone();
        let closure = move |input| {
            let output = function(input);
            scope.send_message(output);
        };
        Callback::once(closure)
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

    /// Creates a `Callback` from an `FnOnce` which will send a batch of messages back
    /// to the linked component's update method when invoked.
    ///
    /// The callback function's return type is generic to allow for dealing with both
    /// `Option` and `Vec` nicely. `Option` can be used when dealing with a callback that
    /// might not need to send an update.
    ///
    /// ```ignore
    /// link.batch_callback_once(|_| vec![Msg::A, Msg::B]);
    /// link.batch_callback_once(|_| Some(Msg::A));
    /// ```
    ///
    /// Please be aware that currently the results of these callbacks
    /// will synchronously schedule calls to the
    /// [Component](Component) interface.
    pub fn batch_callback_once<F, IN, OUT>(&self, function: F) -> Callback<IN>
    where
        F: FnOnce(IN) -> OUT + 'static,
        OUT: SendAsMessage<COMP>,
    {
        let scope = self.clone();
        let closure = move |input| {
            let messages = function(input);
            messages.send(&scope);
        };
        Callback::once(closure)
    }
}

/// Defines a message type that can be sent to a component.
/// Used for the return value of closure given to [Scope::batch_callback](struct.Scope.html#method.batch_callback).
pub trait SendAsMessage<COMP: Component> {
    /// Sends the message to the given component's scope.
    /// See [Scope::batch_callback](struct.Scope.html#method.batch_callback).
    fn send(self, scope: &Scope<COMP>);
}

impl<COMP> SendAsMessage<COMP> for Option<COMP::Message>
where
    COMP: Component,
{
    fn send(self, scope: &Scope<COMP>) {
        if let Some(msg) = self {
            scope.send_message(msg);
        }
    }
}

impl<COMP> SendAsMessage<COMP> for Vec<COMP::Message>
where
    COMP: Component,
{
    fn send(self, scope: &Scope<COMP>) {
        scope.send_message_batch(self);
    }
}
