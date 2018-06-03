//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::{Element, EventListenerHandle, INode, Node};
use stdweb::web::html_element::SelectElement;
use virtual_dom::{Listener, VDiff, VNode};
use callback::Callback;
use scheduler::{Scheduler, Runnable};
use Shared;

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component<CTX>: Sized + 'static {
    /// Control message type which `update` loop get.
    type Message: 'static;
    /// Properties type of component implementation.
    /// It sould be serializable because it's sent to dynamicaly created
    /// component (layed under `VComp`) and must be restored for a component
    /// with unknown type.
    type Properties: Clone + PartialEq + Default;
    /// Initialization routine which could use a context.
    fn create(props: Self::Properties, link: ComponentLink<CTX, Self>) -> Self;
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Message) -> ShouldRender;
    /// This method called when properties changes, and once when component created.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!("you should implement `change` method for a component with properties")
    }
    /// Called for finalization on the final point of the component's lifetime.
    fn destroy(&mut self) { } // TODO Replace with `Drop`
}

/// Should be rendered relative to context and component environment.
pub trait Renderable<CTX, COMP: Component<CTX>> {
    /// Called by rendering loop.
    fn view(&self) -> Html<CTX, COMP>;
}

/// Update message for a `Components` instance. Used by scope sender.
pub(crate) enum ComponentUpdate<CTX, COMP: Component<CTX>> {
    /// Creating an instance of the component
    Create(ComponentLink<CTX, COMP>),
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps properties for a component.
    Properties(COMP::Properties),
    /// Removes the component
    Destroy,
}

/// Link to component's scope for creating callbacks.
pub struct ComponentLink<CTX, COMP: Component<CTX>> {
    scope: Scope<CTX, COMP>,
}

impl<CTX, COMP> ComponentLink<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    /// Create link for a scope.
    fn connect(scope: &Scope<CTX, COMP>) -> Self {
        ComponentLink {
            scope: scope.clone(),
        }
    }

    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&self, function: F) -> Callback<IN>
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
}

/// A context which contains a bridge to send a messages to a loop.
/// Mostly services uses it.
pub struct Scope<CTX, COMP: Component<CTX>> {
    shared_component: Shared<Option<ComponentRunnable<CTX, COMP>>>,
    scheduler: Scheduler<CTX>,
}

impl<CTX, COMP: Component<CTX>> Clone for Scope<CTX, COMP> {
    fn clone(&self) -> Self {
        Scope {
            shared_component: self.shared_component.clone(),
            scheduler: self.scheduler.clone(),
        }
    }
}

impl<CTX, COMP> Scope<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    /// Send the message and schedule an update.
    pub(crate) fn send(&mut self, update: ComponentUpdate<CTX, COMP>) {
        let envelope = ComponentEnvelope {
            shared_component: self.shared_component.clone(),
            message: Some(update),
        };
        let runnable: Box<Runnable<CTX>> = Box::new(envelope);
        self.scheduler.put_and_try_run(runnable);
    }

    /// Send message to a component.
    pub fn send_message(&mut self, message: COMP::Message) {
        let update = ComponentUpdate::Message(message);
        self.send(update);
    }
}

impl<CTX, COMP> Scope<CTX, COMP>
where
    COMP: Component<CTX>,
{
    /// Return an instance of a scheduler with a same pool of the app.
    pub fn scheduler(&self) -> Scheduler<CTX> {
        self.scheduler.clone()
    }
}

/// Holder for the element.
pub type NodeCell = Rc<RefCell<Option<Node>>>;

impl<CTX, COMP> Scope<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    pub(crate) fn new(scheduler: Scheduler<CTX>) -> Self {
        let shared_component = Rc::new(RefCell::new(None));
        Scope { shared_component, scheduler }
    }

    // TODO Consider to use &Node instead of Element as parent
    /// Mounts elements in place of previous node (ancestor).
    pub(crate) fn mount_in_place(
        self,
        element: Element,
        ancestor: Option<VNode<CTX, COMP>>,
        occupied: Option<NodeCell>,
        init_props: Option<COMP::Properties>,
    ) -> Scope<CTX, COMP> {
        let runnable = ComponentRunnable {
            env: self.clone(),
            component: None,
            last_frame: None,
            element,
            ancestor,
            occupied,
            init_props,
            destroyed: false,
        };
        let mut scope = self.clone();
        *scope.shared_component.borrow_mut() = Some(runnable);
        let link = ComponentLink::connect(&scope);
        scope.send(ComponentUpdate::Create(link));
        scope
    }
}

struct ComponentRunnable<CTX, COMP: Component<CTX>> {
    env: Scope<CTX, COMP>,
    component: Option<COMP>,
    last_frame: Option<VNode<CTX, COMP>>,
    element: Element,
    ancestor: Option<VNode<CTX, COMP>>,
    occupied: Option<NodeCell>,
    init_props: Option<COMP::Properties>,
    destroyed: bool,
}

/// Wraps a component reference and a message to hide it under `Runnable` trait.
/// It's necessary to schedule a processing of a message.
struct ComponentEnvelope<CTX, COMP>
where
    COMP: Component<CTX>,
{
    shared_component: Shared<Option<ComponentRunnable<CTX, COMP>>>,
    message: Option<ComponentUpdate<CTX, COMP>>,
}

impl<CTX, COMP> Runnable<CTX> for ComponentEnvelope<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    fn run<'a>(&mut self, _: &mut CTX) {
        let mut component = self.shared_component.borrow_mut();
        let this = component.as_mut().expect("shared component not set");
        if this.destroyed {
            return;
        }
        let mut should_update = false;
        let upd = self.message.take().expect("component's envelope called twice");
        // This loop pops one item, because the following
        // updates could try to borrow the same cell
        // Important! Don't use `while let` here, because it
        // won't free the lock.
        let env = this.env.clone();
        match upd {
            ComponentUpdate::Create(link) => {
                let props = this.init_props.take().unwrap_or_default();
                this.component = Some(COMP::create(props, link));
                // No messages at start
                let current_frame = this.component.as_ref().unwrap().view();
                this.last_frame = Some(current_frame);
                // First-time rendering the tree
                let node = this.last_frame.as_mut()
                    .unwrap()
                    .apply(this.element.as_node(), None, this.ancestor.take(), &env);
                if let Some(ref mut cell) = this.occupied {
                    *cell.borrow_mut() = node;
                }
            }
            ComponentUpdate::Message(msg) => {
                should_update |= this.component.as_mut()
                    .expect("component was not created to process messages")
                    .update(msg);
            }
            ComponentUpdate::Properties(props) => {
                should_update |= this.component.as_mut()
                    .expect("component was not created to process properties")
                    .change(props);
            }
            ComponentUpdate::Destroy => {
                // TODO this.component.take() instead of destroyed
                this.component.as_mut().unwrap().destroy();
                this.destroyed = true;
            }
        }
        if should_update {
            let mut next_frame = this.component.as_ref().unwrap().view();
            // Re-rendering the tree
            let node =
                next_frame.apply(this.element.as_node(), None, this.last_frame.take(), &env);
            if let Some(ref mut cell) = this.occupied {
                *cell.borrow_mut() = node;
            }
            this.last_frame = Some(next_frame);
        }
    }
}

/// A type which expected as a result of `view` function implementation.
pub type Html<CTX, MSG> = VNode<CTX, MSG>;

macro_rules! impl_action {
    ($($action:ident($event:ident : $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        pub mod $action {
            use stdweb::web::{IEventTarget, Element};
            use stdweb::web::event::{IEvent, $type};
            use super::*;

            /// A wrapper for a callback.
            /// Listener extracted from here when attached.
            pub struct Wrapper<F>(Option<F>);

            /// And event type which keeps the returned type.
            pub type Event = $ret;

            impl<F, MSG> From<F> for Wrapper<F>
            where
                MSG: 'static,
                F: Fn($ret) -> MSG + 'static,
            {
                fn from(handler: F) -> Self {
                    Wrapper(Some(handler))
                }
            }

            impl<T, CTX, COMP> Listener<CTX, COMP> for Wrapper<T>
            where
                T: Fn($ret) -> COMP::Message + 'static,
                CTX: 'static,
                COMP: Component<CTX> + Renderable<CTX, COMP>,
            {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&mut self, element: &Element, mut activator: Scope<CTX, COMP>)
                    -> EventListenerHandle {
                    let handler = self.0.take().expect("tried to attach listener twice");
                    let this = element.clone();
                    let listener = move |event: $type| {
                        debug!("Event handler: {}", stringify!($type));
                        event.stop_propagation();
                        let handy_event: $ret = $convert(&this, event);
                        let msg = handler(handy_event);
                        activator.send_message(msg);
                    };
                    element.add_event_listener(listener)
                }
            }
        }
    )*};
}

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(event: ClickEvent) -> ClickEvent => |_, event| { event }
    ondoubleclick(event: DoubleClickEvent) -> DoubleClickEvent => |_, event| { event }
    onkeypress(event: KeyPressEvent) -> KeyPressEvent => |_, event| { event }
    onkeydown(event: KeyDownEvent) -> KeyDownEvent => |_, event| { event }
    onkeyup(event: KeyUpEvent) -> KeyUpEvent => |_, event| { event }
    onmousemove(event: MouseMoveEvent) -> MouseMoveEvent => |_, event| { event }
    onmousedown(event: MouseDownEvent) -> MouseDownEvent => |_, event| { event }
    onmouseup(event: MouseUpEvent) -> MouseUpEvent => |_, event| { event }
    onmouseover(event: MouseOverEvent) -> MouseOverEvent => |_, event| { event }
    onmouseout(event: MouseOutEvent) -> MouseOutEvent => |_, event| { event }
    /* TODO Add PR to https://github.com/koute/stdweb
    onmouseenter(event: MouseEnterEvent) -> () => |_, _| { () }
    onmouseleave(event: MouseLeaveEvent) -> () => |_, _| { () }
    */
    onblur(event: BlurEvent) -> BlurEvent => |_, event| { event }
    oninput(event: InputEvent) -> InputData => |this: &Element, _| {
        use stdweb::web::html_element::{InputElement, TextAreaElement};
        use stdweb::unstable::TryInto;
        let value = match this.clone().try_into() {
            Ok(input) => {
                let input: InputElement = input;
                input.raw_value()
            }
            Err(_e) => {
                match this.clone().try_into() {
                    Ok(tae) => {
                        let tae: TextAreaElement = tae;
                        tae.value()
                    }
                    Err(_e) => {
                        panic!("only an InputElement or TextAreaElement can have an oninput event listener");
                    }
                }
            }
        };
        InputData { value }
    }
    onchange(event: ChangeEvent) -> ChangeData => |this: &Element, _| {
        use stdweb::web::html_element::{InputElement, TextAreaElement, SelectElement};
        use stdweb::unstable::TryInto;
        match this.node_name().as_ref() {
            "INPUT" => {
                let input: InputElement = this.clone().try_into().unwrap();
                ChangeData::Value(input.raw_value())
            }
            "TEXTAREA" => {
                let tae: TextAreaElement = this.clone().try_into().unwrap();
                ChangeData::Value(tae.value())
            }
            "SELECT" => {
                let se: SelectElement = this.clone().try_into().unwrap();
                ChangeData::Select(se)
            }
            _ => {
                panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
            }
        }
    }
}

/// A type representing data from `oninput` event.
#[derive(Debug)]
pub struct InputData {
    /// Inserted characters. Contains value from
    /// [InputEvent](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data).
    pub value: String,
}

// There is no '.../Web/API/ChangeEvent/data' (for onchange) similar to
// https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data (for oninput).
// ChangeData actually contains the value of the InputElement/TextAreaElement
// after `change` event occured or contains the SelectElement (see more at the
// variant ChangeData::Select)

/// A type representing change of value(s) of an element after committed by user
/// ([onchange event](https://developer.mozilla.org/en-US/docs/Web/Events/change)).
#[derive(Debug)]
pub enum ChangeData {
    /// Value of the element in cases of `<input>`, `<textarea>`
    Value(String),
    /// SelectElement in case of `<select>` element. You can use one of methods of SelectElement
    /// to collect your required data such as: `value`, `selected_index`, `selected_indices` or
    /// `selected_values`. You can also iterate throught `selected_options` yourself.
    Select(SelectElement),
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
