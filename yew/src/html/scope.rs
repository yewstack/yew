use super::{Callback, Component, NodeRef, Renderable};
use crate::scheduler::{scheduler, ComponentRunnableType, Runnable, Shared};
use crate::virtual_dom::{VDiff, VNode};
use cfg_if::cfg_if;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::Element;
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::Element;
    }
}

#[cfg(feature = "dev")]
use crate::dev::{
    messages::{ComponentEvent, DebugComponent},
    Debugger,
};

/// Updates for a `Component` instance. Used by scope sender.
pub(crate) enum ComponentUpdate<COMP: Component> {
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps batch of messages for a component.
    MessageBatch(Vec<COMP::Message>),
    /// Wraps properties and new node ref for a component.
    Properties(COMP::Properties, NodeRef),
}

/// Untyped scope used for accessing parent scope
#[derive(Debug, Clone)]
pub struct AnyScope {
    type_id: TypeId,
    parent: Option<Rc<AnyScope>>,
    state: Rc<dyn Any>,
}

impl Default for AnyScope {
    fn default() -> Self {
        Self {
            type_id: TypeId::of::<()>(),
            parent: None,
            state: Rc::new(()),
        }
    }
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
                .downcast_ref::<Shared<ComponentState<COMP>>>()
                .expect("unexpected component type")
                .clone(),
        }
    }
}

/// A context which allows sending messages to a component.
pub struct Scope<COMP: Component> {
    shared_state: Shared<ComponentState<COMP>>,
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
            state_ref.component()?;
            Some(Ref::map(state_ref, |this| this.component().unwrap()))
        })
    }

    pub(crate) fn new(parent: Option<AnyScope>) -> Self {
        let parent = parent.map(Rc::new);
        let state = Rc::new(RefCell::new(ComponentState::Empty));
        Scope { parent, state }
    }

    /// Mounts a component with `props` to the specified `element` in the DOM.
    pub(crate) fn mount_in_place(
        self,
        element: Element,
        ancestor: Option<VNode>,
        node_ref: NodeRef,
        props: COMP::Properties,
    ) -> Scope<COMP> {
        let mut scope = self;
        let ready_state = ReadyState {
            element,
            node_ref,
            scope: scope.clone(),
            props,
            ancestor,
        };
        *scope.state.borrow_mut() = ComponentState::Ready(ready_state);
        scope.create();
        scope
    }

    /// Schedules a task to create and render a component and then mount it to the DOM
    pub(crate) fn create(&mut self) {
        let state = self.state.clone();
        let create = CreateComponent { state };
        scheduler().push_comp(ComponentRunnableType::Create, Box::new(create));
        self.rendered(true);
    }

    /// Schedules a task to send a message or new props to a component
    pub(crate) fn update(&self, update: ComponentUpdate<COMP>) {
        let update = UpdateComponent {
            state: self.state.clone(),
            update,
        };
        scheduler().push_comp(ComponentRunnableType::Update, Box::new(update));
        self.rendered(false);
    }

    /// Schedules a task to call the rendered method on a component
    pub(crate) fn rendered(&self, first_render: bool) {
        let state = self.state.clone();
        let rendered = RenderedComponent {
            state,
            first_render,
        };
        scheduler().push_comp(ComponentRunnableType::Rendered, Box::new(rendered));
    }

    /// Schedules a task to destroy a component
    pub(crate) fn destroy(&mut self) {
        let state = self.state.clone();
        let destroy = DestroyComponent { state };
        scheduler().push_comp(ComponentRunnableType::Destroy, Box::new(destroy));
    }

    /// Send a message to the component
    pub fn queue_message<T>(&self, msg: T)
    where
        T: Into<COMP::Message>,
    {
        self.update(ComponentUpdate::Message(msg.into()));
        self.rendered(false);
    }

    /// Send a batch of messages to the component
    pub fn queue_message_batch(&self, messages: Vec<COMP::Message>) {
        self.update(ComponentUpdate::MessageBatch(messages));
        self.rendered(false);
    }

    /// Creates a `Callback` which will send a message to the linked component's
    /// update method when invoked.
    pub fn callback<F, IN, M>(&self, function: F) -> Callback<IN>
    where
        M: Into<COMP::Message>,
        F: Fn(IN) -> M + 'static,
    {
        let scope = self.clone();
        let closure = move |input| {
            let output = function(input);
            scope.queue_message(output);
        };
        closure.into()
    }

    /// Creates a `Callback` from a FnOnce which will send a message to the linked
    /// component's update method when invoked.
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

    /// Creates a `Callback` which will send a batch of messages back to the linked
    /// component's update method when invoked.
    pub fn batch_callback<F, IN>(&self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> Vec<COMP::Message> + 'static,
    {
        let scope = self.clone();
        let closure = move |input| {
            let messages = function(input);
            scope.queue_message_batch(messages);
        };
        closure.into()
    }
}

enum ComponentState<COMP: Component> {
    Empty,
    Ready(ReadyState<COMP>),
    Created(CreatedState<COMP>),
    Processing,
    Destroyed,
}

impl<COMP: Component> ComponentState<COMP> {
    fn component(&self) -> Option<&COMP> {
        match self {
            ComponentState::Created(state) => Some(&state.component),
            _ => None,
        }
    }
}

impl<COMP: Component> fmt::Display for ComponentState<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ComponentState::Empty => "empty",
            ComponentState::Ready(_) => "ready",
            ComponentState::Created(_) => "created",
            ComponentState::Processing => "processing",
            ComponentState::Destroyed => "destroyed",
        };
        write!(f, "{}", name)
    }
}

struct ReadyState<COMP: Component> {
    element: Element,
    node_ref: NodeRef,
    props: COMP::Properties,
    scope: Scope<COMP>,
    ancestor: Option<VNode>,
}

impl<COMP: Component> ReadyState<COMP> {
    fn create(self) -> CreatedState<COMP> {
        CreatedState {
            rendered: false,
            component: COMP::create(self.props, self.scope.clone()),
            element: self.element,
            last_frame: self.ancestor,
            node_ref: self.node_ref,
            scope: self.scope,
        }
    }
}

struct CreatedState<COMP: Component> {
    rendered: bool,
    element: Element,
    component: COMP,
    last_frame: Option<VNode>,
    node_ref: NodeRef,
    scope: Scope<COMP>,
}

impl<COMP: Component> CreatedState<COMP> {
    /// Called after a component and all of its children have been rendered.
    fn rendered(mut self, first_render: bool) -> Self {
        self.rendered = true;
        self.component.rendered(first_render);
        self
    }

    fn update(mut self) -> Self {
        let mut root = self.component.render();
        if let Some(node) = root.apply(
            &self.scope.clone().into(),
            &self.element,
            None,
            self.last_frame,
        ) {
            self.node_ref.set(Some(node));
        } else if let VNode::VComp(child) = &root {
            // If the root VNode is a VComp, we won't have access to the rendered DOM node
            // because components render asynchronously. In order to bubble up the DOM node
            // from the VComp, we need to link the currently rendering component with its
            // root child component.
            self.node_ref.link(child.node_ref.clone());
        }
        self.last_frame = Some(root);
        self
    }
}

struct RenderedComponent<COMP>
where
    COMP: Component,
{
    state: Shared<ComponentState<COMP>>,
    first_render: bool,
}

impl<COMP> Runnable for RenderedComponent<COMP>
where
    COMP: Component,
{
    fn run(self: Box<Self>) {
        let current_state = self.state.replace(ComponentState::Processing);
        self.state.replace(match current_state {
            ComponentState::Created(s) if !s.rendered => {
                ComponentState::Created(s.rendered(self.first_render))
            }
            ComponentState::Destroyed | ComponentState::Created(_) => current_state,
            ComponentState::Empty | ComponentState::Processing | ComponentState::Ready(_) => {
                panic!("unexpected component state: {}", current_state);
            }
        });
    }
}

struct CreateComponent<COMP>
where
    COMP: Component,
{
    state: Shared<ComponentState<COMP>>,
}

impl<COMP> Runnable for CreateComponent<COMP>
where
    COMP: Component,
{
    fn run(self: Box<Self>) {
        let current_state = self.state.replace(ComponentState::Processing);
        self.state.replace(match current_state {
            ComponentState::Ready(s) => ComponentState::Created(s.create().update()),
            ComponentState::Created(_) | ComponentState::Destroyed => current_state,
            ComponentState::Empty | ComponentState::Processing => {
                panic!("unexpected component state: {}", current_state);
            }
        });
    }
}

struct DestroyComponent<COMP>
where
    COMP: Component,
{
    state: Shared<ComponentState<COMP>>,
}

impl<COMP> Runnable for DestroyComponent<COMP>
where
    COMP: Component,
{
    fn run(self: Box<Self>) {
        match self.state.replace(ComponentState::Destroyed) {
            ComponentState::Created(mut this) => {
                drop(this.component);
                if let Some(last_frame) = &mut this.last_frame {
                    last_frame.detach(&this.element);
                }
            }
            ComponentState::Ready(mut this) => {
                if let Some(ancestor) = &mut this.ancestor {
                    ancestor.detach(&this.element);
                }
            }
            ComponentState::Empty | ComponentState::Destroyed => {}
            s @ ComponentState::Processing => panic!("unexpected component state: {}", s),
        };
    }
}

struct UpdateComponent<COMP>
where
    COMP: Component,
{
    state: Shared<ComponentState<COMP>>,
    update: ComponentUpdate<COMP>,
}

impl<COMP> Runnable for UpdateComponent<COMP>
where
    COMP: Component,
{
    fn run(self: Box<Self>) {
        let current_state = self.state.replace(ComponentState::Processing);
        self.state.replace(match current_state {
            ComponentState::Created(mut this) => {
                let should_update = match self.update {
                    ComponentUpdate::Message(message) => this.component.update(message),
                    ComponentUpdate::MessageBatch(messages) => messages
                        .into_iter()
                        .fold(false, |acc, msg| this.component.update(msg) || acc),
                    ComponentUpdate::Properties(props, node_ref) => {
                        // When components are updated, they receive a new node ref that
                        // must be linked to previous one.
                        node_ref.link(this.node_ref.clone());
                        this.component.change(props)
                    }
                };
                let next_state = if should_update {
                    this.rendered = false;
                    this.update()
                } else {
                    this
                };
                ComponentState::Created(next_state)
            }
            ComponentState::Destroyed => current_state,
            ComponentState::Processing | ComponentState::Ready(_) | ComponentState::Empty => {
                panic!("unexpected component state: {}", current_state);
            }
        });
    }
}
