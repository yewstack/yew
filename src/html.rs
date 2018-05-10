//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use stdweb::web::event::{BlurEvent, IKeyboardEvent, IMouseEvent};
use stdweb::web::{Element, EventListenerHandle, INode, Node};
use stdweb::web::html_element::SelectElement;
use virtual_dom::{Listener, VDiff, VNode};
use callback::Callback;
use scheduler::{Scheduler, RunnableIndex, Runnable, WillDestroy};

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
    fn create(props: Self::Properties, context: &mut Env<CTX, Self>) -> Self;
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Message, context: &mut Env<CTX, Self>) -> ShouldRender;
    /// This method called when properties changes, and once when component created.
    fn change(&mut self, _: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        unimplemented!("you should implement `change` method for a component with properties")
    }
}

/// Should be rendered relative to context and component environment.
pub trait Renderable<CTX, COMP: Component<CTX>> {
    /// Called by rendering loop.
    fn view(&self) -> Html<CTX, COMP>;
}

/// Update message for a `Components` instance. Used by scope sender.
pub(crate) enum ComponentUpdate<CTX, COMP: Component<CTX>> {
    /// Creating an instance of the component
    Create,
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps properties for a component.
    Properties(COMP::Properties),
    /// Removes the component
    Destroy,
}

/// A reference to environment of a component (scope) which contains
/// shared reference to a context.
pub struct ContextMut<'a, CTX: 'a> {
    context: RefMut<'a, CTX>,
}

impl<'a, CTX: 'a> Deref for ContextMut<'a, CTX> {
    type Target = CTX;

    fn deref(&self) -> &CTX {
        &self.context
    }
}

impl<'a, CTX: 'a> DerefMut for ContextMut<'a, CTX> {
    fn deref_mut(&mut self) -> &mut CTX {
        &mut self.context
    }
}

/// A reference to environment of a component (scope) which contains
/// shared reference to a context and a sender to a scope's loop.
pub struct Env<'a, CTX: 'a, COMP: Component<CTX>> {
    context: &'a mut CTX,
    activator: &'a mut Activator<CTX, COMP>,
}

impl<'a, CTX: 'a, COMP: Component<CTX>> Deref for Env<'a, CTX, COMP> {
     type Target = CTX;

     fn deref(&self) -> &CTX {
         &self.context
     }
 }

impl<'a, CTX: 'a, COMP: Component<CTX>> DerefMut for Env<'a, CTX, COMP> {
    fn deref_mut(&mut self) -> &mut CTX {
        &mut self.context
    }
}

impl<'a, CTX: 'static, COMP: Component<CTX>> Env<'a, CTX, COMP> {
    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> COMP::Message + 'static,
    {
        let activator = self.activator.clone();
        let closure = move |input| {
            let output = function(input);
            activator.clone().send_message(output);
        };
        closure.into()
    }
}

/// Holds a reference to a scope, could put a message into the queue
/// of the scope and activate processing (try borrow and call routine).
pub struct Activator<CTX, COMP: Component<CTX>> {
    index: Rc<RefCell<Option<RunnableIndex>>>,
    scheduler: Scheduler<CTX>,
    queue: Rc<RefCell<VecDeque<ComponentUpdate<CTX, COMP>>>>,
}

impl<CTX, COMP: Component<CTX>> Clone for Activator<CTX, COMP> {
    fn clone(&self) -> Self {
        Activator {
            index: self.index.clone(),
            scheduler: self.scheduler.clone(),
            queue: self.queue.clone(),
        }
    }
}

impl<CTX, COMP: Component<CTX>> Activator<CTX, COMP> {
    /// Send the message and schedule an update.
    pub(crate) fn send(&mut self, update: ComponentUpdate<CTX, COMP>) {
        // Queue should never bew blocked with an intersection
        self.queue.try_borrow_mut()
            .expect("internal message routing accident")
            .push_back(update);
        let idx = self.index.borrow().as_ref()
            .cloned()
            .expect("index was not set");
        self.scheduler.put_and_try_run(idx);
    }

    /// Send message to a component.
    pub fn send_message(&mut self, message: COMP::Message) {
        let update = ComponentUpdate::Message(message);
        self.send(update);
    }

    /// Return an instance of a scheduler with a same pool of the app.
    pub fn scheduler(&self) -> Scheduler<CTX> {
        self.scheduler.clone()
    }
}

/// A context which contains a bridge to send a messages to a loop.
/// Mostly services uses it.
pub(crate) struct Scope<CTX, COMP: Component<CTX>> {
    env: Activator<CTX, COMP>,
}

/// Holder for the element.
pub type NodeCell = Rc<RefCell<Option<Node>>>;

impl<CTX, COMP> Scope<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    pub(crate) fn new(scheduler: Scheduler<CTX>) -> Self {
        let index = Rc::new(RefCell::new(None));
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let env = Activator { index, scheduler, queue };
        Scope { env }
    }

    pub(crate) fn activator(&self) -> Activator<CTX, COMP> {
        self.env.clone()
    }

    // TODO Consider to use &Node instead of Element as parent
    /// Mounts elements in place of previous node (ancestor).
    pub fn mount_in_place(
        self,
        element: Element,
        ancestor: Option<VNode<CTX, COMP>>,
        occupied: Option<NodeCell>,
        init_props: Option<COMP::Properties>,
    ) -> Activator<CTX, COMP> {
        let runnable = ScopeRunnable {
            env: self.env.clone(),
            component: None,
            last_frame: None,
            element,
            ancestor,
            occupied,
            init_props,
        };
        let mut activator = self.env.clone();
        let idx = activator.scheduler.register(runnable);
        *activator.index.borrow_mut() = Some(idx);
        activator.send(ComponentUpdate::Create);
        activator
    }
}

struct ScopeRunnable<CTX, COMP: Component<CTX>> {
    env: Activator<CTX, COMP>,
    component: Option<COMP>,
    last_frame: Option<VNode<CTX, COMP>>,
    element: Element,
    ancestor: Option<VNode<CTX, COMP>>,
    occupied: Option<NodeCell>,
    init_props: Option<COMP::Properties>,
}

impl<CTX, COMP> Runnable<CTX> for ScopeRunnable<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    fn run<'a>(&mut self, context: &'a mut CTX) -> WillDestroy {
        let mut will_destroy = false;
        let mut should_update = false;
        // Important! Don't clone it outside and move here, becase index
        // attached after this closure created!
        let upd = self.env.queue.borrow_mut()
            .pop_front()
            .expect("update message must be in a queue when routine scheduled");
        // This loop pops one item, because the following
        // updates could try to borrow the same cell
        // Important! Don't use `while let` here, because it
        // won't free the lock.
        let env = self.env.clone();
        let mut context = Env {
            context: context,
            activator: &mut self.env,
        };
        match upd {
            ComponentUpdate::Create => {
                let props = self.init_props.take().unwrap_or_default();
                self.component = Some(COMP::create(props, &mut context));
                // No messages at start
                let current_frame = self.component.as_ref().unwrap().view();
                self.last_frame = Some(current_frame);
                // First-time rendering the tree
                let node = self.last_frame.as_mut()
                    .unwrap()
                    .apply(self.element.as_node(), None, self.ancestor.take(), &env);
                if let Some(ref mut cell) = self.occupied {
                    *cell.borrow_mut() = node;
                }
            }
            ComponentUpdate::Message(msg) => {
                should_update |= self.component.as_mut().unwrap().update(msg, &mut context);
            }
            ComponentUpdate::Properties(props) => {
                should_update |= self.component.as_mut().unwrap().change(props, &mut context);
            }
            ComponentUpdate::Destroy => {
                will_destroy = true;
            }
        }
        if should_update {
            let mut next_frame = self.component.as_ref().unwrap().view();
            // Re-rendering the tree
            let node =
                next_frame.apply(self.element.as_node(), None, self.last_frame.take(), &env);
            if let Some(ref mut cell) = self.occupied {
                *cell.borrow_mut() = node;
            }
            self.last_frame = Some(next_frame);
        }
        will_destroy
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

            impl<T, CTX: 'static, COMP: Component<CTX>> Listener<CTX, COMP> for Wrapper<T>
            where
                T: Fn($ret) -> COMP::Message + 'static,
            {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&mut self, element: &Element, mut activator: Activator<CTX, COMP>)
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
    onclick(event: ClickEvent) -> MouseData => |_, event| { MouseData::from(event) }
    ondoubleclick(event: DoubleClickEvent) -> MouseData => |_, event| { MouseData::from(event) }
    onkeypress(event: KeyPressEvent) -> KeyData => |_, event| { KeyData::from(event) }
    onmousemove(event: MouseMoveEvent) -> MouseData => |_, event| { MouseData::from(event) }
    /* TODO Add PR to https://github.com/koute/stdweb
    onmousedown(event: MouseDownEvent) -> () => |_, _| { () }
    onmouseup(event: MouseUpEvent) -> () => |_, _| { () }
    onmouseenter(event: MouseEnterEvent) -> () => |_, _| { () }
    onmouseleave(event: MouseLeaveEvent) -> () => |_, _| { () }
    onmouseover(event: MouseOverEvent) -> () => |_, _| { () }
    onmouseout(event: MouseOutEvent) -> () => |_, _| { () }
    */
    onblur(event: BlurEvent) -> BlurData => |_, event| {
        BlurData::from(event)
    }
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

/// A type representing data from `onclick` and `ondoubleclick` event.
#[derive(Debug)]
pub struct MouseData {
    /// The screenX is a read-only property of the
    /// [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)
    /// property which provides the horizontal coordinate (offset)
    /// of the mouse pointer in global (screen) coordinates.
    pub screen_x: i32,
    /// The screenY is a read-only property of the
    /// [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)
    /// property which provides the vertical coordinate (offset)
    /// of the mouse pointer in global (screen) coordinates.
    pub screen_y: i32,
    /// The clientX is a read-only property of the
    /// [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)
    /// interface which provides the horizontal coordinate within
    /// the application's client area at which the event occurred
    pub client_x: i32,
    /// The clientY is a read-only property of the
    /// [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)
    /// interface which provides the vertical coordinate within
    /// the application's client area at which the event occurred
    pub client_y: i32,
}

impl<T: IMouseEvent> From<T> for MouseData {
    fn from(event: T) -> Self {
        MouseData {
            screen_x: event.screen_x(),
            screen_y: event.screen_y(),
            client_x: event.client_x(),
            client_y: event.client_y(),
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

/// A type representing data from `onkeypress` event.
#[derive(Debug)]
pub struct KeyData {
    /// Value of a pressed key. Contains key name from
    /// [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key).
    pub key: String,
}

impl<T: IKeyboardEvent> From<T> for KeyData {
    fn from(event: T) -> Self {
        KeyData { key: event.key() }
    }
}

/// A type representing `onblur` event.
#[derive(Debug)]
pub struct BlurData;

impl From<BlurEvent> for BlurData {
    fn from(_: BlurEvent) -> Self {
        BlurData
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
