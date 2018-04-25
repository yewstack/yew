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
use virtual_dom::{Listener, VDiff, VNode};
use callback::Callback;
use scheduler::{Scheduler, Runnable, RunnableIndex};

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component<CTX>: Sized + 'static {
    /// Control message type which `update` loop get.
    type Msg: 'static;
    /// Properties type of component implementation.
    /// It sould be serializable because it's sent to dynamicaly created
    /// component (layed under `VComp`) and must be restored for a component
    /// with unknown type.
    type Properties: Clone + PartialEq + Default;
    /// Initialization routine which could use a context.
    fn create(props: Self::Properties, context: &mut Env<CTX, Self>) -> Self;
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender;
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
pub enum ComponentUpdate<CTX, COMP: Component<CTX>> {
    /// Creating an instance of the component
    Create,
    /// Wraps messages for a component.
    Message(COMP::Msg),
    /// Wraps properties for a component.
    Properties(COMP::Properties),
    /// Removes the component
    Destroy,
}

/*
/// Shared reference to a context.
pub type SharedContext<CTX> = Rc<RefCell<CTX>>;
*/

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


/*
/// This type holds a reference to a context instance and
/// sender to send messages to a component attached to a scope.
/// An instance could be dereferenced as a context.
pub struct Env<CTX, COMP: Component<CTX>> {
    context: SharedContext<CTX>,
    activator: Activator<CTX, COMP>,
}
*/

//pub type Env<CTX, COMP> = Activator<CTX, COMP>;

/*
impl<CTX, COMP: Component<CTX>> Clone for Env<CTX, COMP> {
    fn clone(&self) -> Self {
        Env {
            context: self.context.clone(),
            activator: self.activator.clone(),
        }
    }
}

impl<CTX, COMP: Component<CTX>> Env<CTX, COMP> {
    /// Clones activator.
    pub fn activator(&self) -> Activator<CTX, COMP> {
        self.activator.clone()
    }

    /// Clones shared context.
    pub(crate) fn context_rc(&self) -> SharedContext<CTX> {
        self.context.clone()
    }

    /// Returns a borrowed reference to a context.
    pub fn context(&self) -> ContextMut<CTX> {
        let context = self.context.try_borrow_mut()
            // This issue could be fixed with a scheduler only
            .expect("can't borrow the context");
        ContextMut { context }
    }
}
*/

impl<'a, CTX: 'static, COMP: Component<CTX>> Env<'a, CTX, COMP> {
    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> COMP::Msg + 'static,
    {
        let activator = self.activator.clone();
        let closure = move |input| {
            let output = function(input);
            activator.clone().send_message(output);
        };
        closure.into()
    }
}

type WillDestroy = bool;

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
    pub fn send(&mut self, update: ComponentUpdate<CTX, COMP>) {
        // Queue should never bew blocked with an intersection
        self.queue.try_borrow_mut()
            .expect("internal message routing accident")
            .push_back(update);
        let mut will_destroy = false;
        let idx = self.index.borrow().as_ref()
            .cloned()
            .expect("index was not set");
        self.scheduler.put_and_try_run(idx);
    }

    /// Send message to a component.
    pub fn send_message(&mut self, message: COMP::Msg) {
        let update = ComponentUpdate::Message(message);
        self.send(update);
    }

    pub fn scheduler(&self) -> Scheduler<CTX> {
        self.scheduler.clone()
    }
}

// TODO Consider to remove this type
/// Builder for new scopes
pub(crate) struct ScopeBuilder<CTX, COMP: Component<CTX>> {
    activator: Activator<CTX, COMP>,
}

impl<CTX, COMP: Component<CTX>> ScopeBuilder<CTX, COMP> {
    /// Prepares a new builder instance
    pub fn new(scheduler: Scheduler<CTX>) -> Self {
        let index = Rc::new(RefCell::new(None));
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let activator = Activator { index, scheduler, queue };
        ScopeBuilder { activator }
    }

    /*
    /// Returns an activator of the scope's loop.
    pub fn activator(&mut self) -> Activator<CTX, COMP> {
        self.activator.clone()
    }
    */

    // TODO Consider removing it
    pub fn build(self) -> (Activator<CTX, COMP>, Scope<CTX, COMP>) {
        let env = self.activator;
        let scope = Scope {
            env: env.clone(),
        };
        // TODO! It's possible to return App here
        // TODO Consider to join ScopeBuilder with App
        (env, scope)
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
    // TODO Consider to use &Node instead of Element as parent
    /// Mounts elements in place of previous node (ancestor).
    pub fn mount_in_place(
        mut self,
        element: Element,
        mut ancestor: Option<VNode<CTX, COMP>>,
        mut occupied: Option<NodeCell>,
        mut init_props: Option<COMP::Properties>,
    ) -> Activator<CTX, COMP> {
        // TODO Move it to a struct which implements BeRunnable (avoid creating closures)
        let mut component = None;
        let mut last_frame = None;
        let mut activator = self.env.clone();
        let routine = {
            let updates = self.env.queue.clone();
            move |context: &mut CTX| {
                let mut will_destroy = false;
                let mut should_update = false;
                // Important! Don't clone it outside and move here, becase index
                // attached after this closure created!
                let env = self.env.clone();
                // This loop pops one item, because the following
                // updates could try to borrow the same cell
                // Important! Don't use `while let` here, because it
                // won't free the lock.
                let mut context = Env {
                    context: context,
                    activator: &mut self.env,
                };
                let upd = updates.borrow_mut()
                    .pop_front()
                    .expect("update message must be in a queue when routine scheduled");
                match upd {
                    ComponentUpdate::Create => {
                        let props = init_props.take().unwrap_or_default();
                        component = Some(COMP::create(props, &mut context));
                        // No messages at start
                        let current_frame = component.as_ref().unwrap().view();
                        last_frame = Some(current_frame);
                        // First-time rendering the tree
                        let node = last_frame.as_mut()
                            .unwrap()
                            .apply(element.as_node(), None, ancestor.take(), &env);
                        if let Some(ref mut cell) = occupied {
                            *cell.borrow_mut() = node;
                        }
                    }
                    ComponentUpdate::Message(msg) => {
                        should_update |= component.as_mut().unwrap().update(msg, &mut context);
                    }
                    ComponentUpdate::Properties(props) => {
                        should_update |= component.as_mut().unwrap().change(props, &mut context);
                    }
                    ComponentUpdate::Destroy => {
                        will_destroy = true;
                    }
                }
                if should_update {
                    let mut next_frame = component.as_ref().unwrap().view();
                    // Re-rendering the tree
                    let node =
                        next_frame.apply(element.as_node(), None, last_frame.take(), &env);
                    if let Some(ref mut cell) = occupied {
                        *cell.borrow_mut() = node;
                    }
                    last_frame = Some(next_frame);
                }
                //will_destroy
            }
        };
        let idx = activator.scheduler.register(routine);
        *activator.index.borrow_mut() = Some(idx);
        activator.send(ComponentUpdate::Create);
        activator
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
                T: Fn($ret) -> COMP::Msg + 'static,
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
