//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

mod listener;

use crate::callback::Callback;
use crate::scheduler::{scheduler, Runnable, Shared};
use crate::virtual_dom::{VDiff, VNode};
pub use listener::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use stdweb::web::{Element, Node};
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

/// A type which expected as a result of `view` function implementation.
pub type Html<MSG> = VNode<MSG>;

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component: Sized + 'static {
    /// Control message type which `update` loop get.
    type Message: 'static;
    /// Properties type of component implementation.
    type Properties: Properties;
    /// Initialization routine which could use a context.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self;
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Message) -> ShouldRender;
    /// Called when the component's parent component re-renders and the
    /// component's place in the DOM tree remains unchanged. If the component's
    /// place in the DOM tree changes, calling this method is unnecessary as the
    /// component is recreated from scratch. It defaults
    /// to true if not implemented.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    /// Called for finalization on the final point of the component's lifetime.
    fn destroy(&mut self) {} // TODO Replace with `Drop`
}

/// Should be rendered relative to context and component environment.
pub trait Renderable<COMP: Component> {
    /// Called by rendering loop.
    fn view(&self) -> Html<COMP>;
}

/// Trait for building properties for a component
pub trait Properties {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

/// Builder for when a component has no properties
pub struct EmptyBuilder;

impl Properties for () {
    type Builder = EmptyBuilder;

    fn builder() -> Self::Builder {
        EmptyBuilder
    }
}

impl EmptyBuilder {
    /// Build empty properties
    pub fn build(self) {}
}

/// Updates for a `Components` instance. Used by scope sender.
pub(crate) enum ComponentUpdate<COMP: Component> {
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps properties for a component.
    Properties(COMP::Properties),
}

/// Link to component's scope for creating callbacks.
pub struct ComponentLink<COMP: Component> {
    scope: Scope<COMP>,
}

impl<COMP> ComponentLink<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// Create link for a scope.
    fn connect(scope: &Scope<COMP>) -> Self {
        ComponentLink {
            scope: scope.clone(),
        }
    }

    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> COMP::Message + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input);
            scope.clone().send_message(output);
        };
        closure.into()
    }

    /// This method sends a message to this component immediately.
    pub fn send_self(&mut self, msg: COMP::Message) {
        self.scope.send_message(msg);
    }
}

enum ComponentState<COMP: Component> {
    Empty,
    Ready(ReadyState<COMP>),
    Created(CreatedState<COMP>),
    Processing,
    Destroyed,
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
    env: Scope<COMP>,
    element: Element,
    occupied: Option<NodeCell>,
    props: COMP::Properties,
    link: ComponentLink<COMP>,
    ancestor: Option<VNode<COMP>>,
}

impl<COMP: Component> ReadyState<COMP> {
    fn create(self) -> CreatedState<COMP> {
        CreatedState {
            component: COMP::create(self.props, self.link),
            env: self.env,
            element: self.element,
            last_frame: self.ancestor,
            occupied: self.occupied,
        }
    }
}

struct CreatedState<COMP: Component> {
    env: Scope<COMP>,
    element: Element,
    component: COMP,
    last_frame: Option<VNode<COMP>>,
    occupied: Option<NodeCell>,
}

impl<COMP: Component + Renderable<COMP>> CreatedState<COMP> {
    fn update(mut self) -> Self {
        let mut next_frame = self.component.view();
        let node = next_frame.apply(&self.element, None, self.last_frame, &self.env);
        if let Some(ref mut cell) = self.occupied {
            *cell.borrow_mut() = node;
        }

        Self {
            env: self.env,
            component: self.component,
            last_frame: Some(next_frame),
            element: self.element,
            occupied: self.occupied,
        }
    }
}

/// A context which contains a bridge to send a messages to a loop.
/// Mostly services uses it.
pub struct Scope<COMP: Component> {
    shared_state: Shared<ComponentState<COMP>>,
}

impl<COMP: Component> Clone for Scope<COMP> {
    fn clone(&self) -> Self {
        Scope {
            shared_state: self.shared_state.clone(),
        }
    }
}

impl<COMP> Scope<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    pub(crate) fn create(&mut self) {
        let shared_state = self.shared_state.clone();
        let create = CreateComponent { shared_state };
        scheduler().put_and_try_run(Box::new(create));
    }

    pub(crate) fn update(&mut self, update: ComponentUpdate<COMP>) {
        let update = UpdateComponent {
            shared_state: self.shared_state.clone(),
            update,
        };
        scheduler().put_and_try_run(Box::new(update));
    }

    pub(crate) fn destroy(&mut self) {
        let shared_state = self.shared_state.clone();
        let destroy = DestroyComponent { shared_state };
        scheduler().put_and_try_run(Box::new(destroy));
    }

    /// Send a message to the component
    pub fn send_message(&mut self, msg: COMP::Message) {
        self.update(ComponentUpdate::Message(msg));
    }
}

/// Holder for the element.
pub type NodeCell = Rc<RefCell<Option<Node>>>;

impl<COMP> Scope<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// visible for testing
    pub fn new() -> Self {
        let shared_state = Rc::new(RefCell::new(ComponentState::Empty));
        Scope { shared_state }
    }

    // TODO Consider to use &Node instead of Element as parent
    /// Mounts elements in place of previous node (ancestor).
    pub(crate) fn mount_in_place(
        self,
        element: Element,
        ancestor: Option<VNode<COMP>>,
        occupied: Option<NodeCell>,
        props: COMP::Properties,
    ) -> Scope<COMP> {
        let mut scope = self.clone();
        let link = ComponentLink::connect(&scope);
        let ready_state = ReadyState {
            env: self.clone(),
            element,
            occupied,
            link,
            props,
            ancestor,
        };
        *scope.shared_state.borrow_mut() = ComponentState::Ready(ready_state);
        scope.create();
        scope
    }
}

struct CreateComponent<COMP>
where
    COMP: Component,
{
    shared_state: Shared<ComponentState<COMP>>,
}

impl<COMP> Runnable for CreateComponent<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    fn run(self: Box<Self>) {
        let current_state = self.shared_state.replace(ComponentState::Processing);
        self.shared_state.replace(match current_state {
            ComponentState::Ready(state) => ComponentState::Created(state.create().update()),
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
    shared_state: Shared<ComponentState<COMP>>,
}

impl<COMP> Runnable for DestroyComponent<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    fn run(self: Box<Self>) {
        match self.shared_state.replace(ComponentState::Destroyed) {
            ComponentState::Created(mut this) => {
                this.component.destroy();
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
    shared_state: Shared<ComponentState<COMP>>,
    update: ComponentUpdate<COMP>,
}

impl<COMP> Runnable for UpdateComponent<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    fn run(self: Box<Self>) {
        let current_state = self.shared_state.replace(ComponentState::Processing);
        self.shared_state.replace(match current_state {
            ComponentState::Created(mut this) => {
                let should_update = match self.update {
                    ComponentUpdate::Message(msg) => this.component.update(msg),
                    ComponentUpdate::Properties(props) => this.component.change(props),
                };
                let next_state = if should_update { this.update() } else { this };
                ComponentState::Created(next_state)
            }
            ComponentState::Destroyed => current_state,
            ComponentState::Processing | ComponentState::Ready(_) | ComponentState::Empty => {
                panic!("unexpected component state: {}", current_state);
            }
        });
    }
}

/// A bridging type for checking `href` attribute value.
#[derive(Debug)]
pub struct Href {
    link: String,
}

impl From<String> for Href {
    fn from(link: String) -> Self {
        Href { link }
    }
}

impl<'a> From<&'a str> for Href {
    fn from(link: &'a str) -> Self {
        Href {
            link: link.to_owned(),
        }
    }
}

impl ToString for Href {
    fn to_string(&self) -> String {
        self.link.to_owned()
    }
}
