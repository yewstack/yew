//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use stdweb::Value;
use stdweb::web::event::{BlurEvent, IKeyboardEvent, IMouseEvent};
use stdweb::web::{document, Element, EventListenerHandle, INode, IParentNode, Node};
use virtual_dom::{Listener, VDiff, VNode};

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
    /// Wraps messages for a component.
    Message(COMP::Msg),
    /// Wraps properties for a component.
    Properties(COMP::Properties),
}

/// Internal alias for sender.
pub(crate) type ComponentSender<CTX, COMP> = Sender<ComponentUpdate<CTX, COMP>>;

/// Universal callback wrapper.
/// <aside class="warning">
/// Use callbacks carefully, because it you call it from `update` loop
/// of `Components` (even from JS) it will delay a message until next.
/// Callbacks should be used from JS callbacks or `setTimeout` calls.
/// </aside>
/// `Rc` wrapper used to make it clonable.
#[must_use]
pub struct Callback<IN>(Rc<Fn(IN)>);

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}

impl<IN> Clone for Callback<IN> {
    fn clone(&self) -> Self {
        Callback(self.0.clone())
    }
}

impl<IN> PartialEq for Callback<IN> {
    fn eq(&self, other: &Callback<IN>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<IN> Callback<IN> {
    /// This method calls the actual callback.
    pub fn emit(&self, value: IN) {
        (self.0)(value);
    }
}

impl<IN: 'static> Callback<IN> {
    /// Changes input type of the callback to another.
    /// Works like common `map` method but in an opposite direction.
    pub fn reform<F, T>(self, func: F) -> Callback<T>
    where
        F: Fn(T) -> IN + 'static,
    {
        let func = move |input| {
            let output = func(input);
            self.clone().emit(output);
        };
        Callback::from(func)
    }
}

/// Shared reference to a context.
pub type SharedContext<CTX> = Rc<RefCell<CTX>>;

/// A reference to environment of a component (scope) which contains
/// shared reference to a context and a sender to a scope's loop.
pub struct Env<'a, CTX: 'a, COMP: Component<CTX>> {
    context: RefMut<'a, CTX>,
    sender: &'a mut ScopeSender<CTX, COMP>,
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
        F: Fn(IN) -> COMP::Msg + 'static,
    {
        let sender = self.sender.clone();
        let closure = move |input| {
            let output = function(input);
            let update = ComponentUpdate::Message(output);
            sender.clone().send(update);
        };
        closure.into()
    }
}

/// This type holds a reference to a context instance and
/// sender to send messages to a component attached to a scope.
/// An instance could be dereferenced as a context.
pub struct ScopeEnv<CTX, COMP: Component<CTX>> {
    context: SharedContext<CTX>,
    sender: ScopeSender<CTX, COMP>,
}

impl<CTX, COMP: Component<CTX>> Clone for ScopeEnv<CTX, COMP> {
    fn clone(&self) -> Self {
        ScopeEnv {
            context: self.context.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl<CTX, COMP: Component<CTX>> ScopeEnv<CTX, COMP> {
    /// Clones sender.
    pub fn sender(&self) -> ScopeSender<CTX, COMP> {
        self.sender.clone()
    }

    /// Clones shared context.
    pub fn context(&self) -> SharedContext<CTX> {
        self.context.clone()
    }
}

impl<CTX: 'static, COMP: Component<CTX>> ScopeEnv<CTX, COMP> {
    /// Returns reference to a scope data.
    pub fn get_ref<'a>(&'a mut self) -> Env<'a, CTX, COMP> {
        Env {
            context: self.context.borrow_mut(),
            sender: &mut self.sender,
        }
    }
}

/// This struct keeps a sender to a context to send a messages to a loop
/// and to schedule the next update call.
pub struct ScopeSender<CTX, COMP: Component<CTX>> {
    tx: ComponentSender<CTX, COMP>,
    bind: Value,
}

impl<CTX, COMP: Component<CTX>> Clone for ScopeSender<CTX, COMP> {
    fn clone(&self) -> Self {
        ScopeSender {
            tx: self.tx.clone(),
            bind: self.bind.clone(),
        }
    }
}

impl<CTX, COMP: Component<CTX>> ScopeSender<CTX, COMP> {
    /// Send the message and schedule an update.
    pub fn send(&mut self, update: ComponentUpdate<CTX, COMP>) {
        if let Ok(()) = self.tx.send(update) {
            let bind = &self.bind;
            js! { @(no_return)
                // Schedule to call the loop handler
                // IMPORTANT! If call loop function immediately
                // it stops handling other messages and the first
                // one will be fired.
                var bind = @{bind};
                // Put bind holder instad of callback function, because
                // scope could be dropped and `loop` function will be changed
                window._yew_schedule_(bind);
            }
        } else {
            eprintln!(
                "Can't send message to a component. Receiver lost! \
                 Maybe Task lives longer than a component instance."
            );
        }
    }
}

pub(crate) struct ScopeBuilder<CTX, COMP: Component<CTX>> {
    tx: ComponentSender<CTX, COMP>,
    rx: Receiver<ComponentUpdate<CTX, COMP>>,
    bind: Value,
}

impl<CTX, COMP: Component<CTX>> ScopeBuilder<CTX, COMP> {
    pub fn new() -> Self {
        let bind = js! {
            return { "loop": function() { } };
        };
        let (tx, rx) = channel();
        ScopeBuilder { tx, rx, bind }
    }

    /// Lightweight sender for sending properties updates from `VComp`.
    pub fn sender(&self) -> ScopeSender<CTX, COMP> {
        ScopeSender {
            tx: self.tx.clone(),
            bind: self.bind.clone(),
        }
    }

    /// Return handler to a scope. Warning! Don't use more than one handle!
    pub fn handle(&self) -> ScopeHandle {
        ScopeHandle {
            bind: self.bind.clone(),
        }
    }

    pub fn build(self, context: SharedContext<CTX>) -> Scope<CTX, COMP> {
        Scope {
            tx: self.tx,
            rx: Some(self.rx),
            context: context,
            bind: self.bind,
        }
    }
}

/// A context which contains a bridge to send a messages to a loop.
/// Mostly services uses it.
pub struct Scope<CTX, COMP: Component<CTX>> {
    context: SharedContext<CTX>,
    bind: Value,
    tx: ComponentSender<CTX, COMP>,
    rx: Option<Receiver<ComponentUpdate<CTX, COMP>>>,
}

impl<CTX, COMP> Scope<CTX, COMP>
where
    COMP: Component<CTX>,
{
    /// Creates app with a context.
    pub fn new(context: CTX) -> Self {
        let context = Rc::new(RefCell::new(context));
        Scope::reuse(context)
    }

    /// Creates isolated `App` instance, but reuse the context.
    pub fn reuse(context: SharedContext<CTX>) -> Self {
        let builder = ScopeBuilder::new();
        builder.build(context)
    }

    /// Returns an environment.
    pub fn get_env(&mut self) -> ScopeEnv<CTX, COMP> {
        let sender = ScopeSender {
            tx: self.tx.clone(),
            bind: self.bind.clone(),
        };
        ScopeEnv {
            context: self.context.clone(),
            sender,
        }
    }
}

/// Holder for the element.
pub type NodeCell = Rc<RefCell<Option<Node>>>;

impl<CTX, COMP> Scope<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + Renderable<CTX, COMP>,
{
    /// Alias to `mount("body", ...)`.
    pub fn mount_to_body(self) {
        let element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        self.mount(element)
    }

    /// The main entrypoint of a yew program. It works similar as `program`
    /// function in Elm. You should provide an initial model, `update` function
    /// which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub fn mount(self, element: Element) {
        clear_element(&element);
        self.mount_in_place(element, None, None, None)
    }

    // TODO Consider to use &Node instead of Element as parent
    /// Mounts elements in place of previous node.
    pub fn mount_in_place(
        mut self,
        element: Element,
        obsolete: Option<VNode<CTX, COMP>>,
        mut occupied: Option<NodeCell>,
        init_props: Option<COMP::Properties>,
    ) {
        let mut component = {
            let props = init_props.unwrap_or_default();
            let mut env = self.get_env();
            let mut scope_ref = env.get_ref();
            COMP::create(props, &mut scope_ref)
        };
        // No messages at start
        let mut updates = Vec::new();
        let mut last_frame = VNode::from(component.view());
        // First-time rendering the tree
        let node = last_frame.apply(element.as_node(), None, obsolete, self.get_env());
        if let Some(ref mut cell) = occupied {
            *cell.borrow_mut() = node;
        }
        let mut last_frame = Some(last_frame);
        let rx = self.rx
            .take()
            .expect("application runned without a receiver");
        let bind = self.bind.clone();
        let mut callback = move || {
            let mut should_update = false;
            updates.extend(rx.try_iter());
            {
                let mut env = self.get_env();
                let mut scope_ref = env.get_ref();
                for upd in updates.drain(..) {
                    match upd {
                        ComponentUpdate::Message(msg) => {
                            should_update |= component.update(msg, &mut scope_ref);
                        }
                        ComponentUpdate::Properties(props) => {
                            should_update |= component.change(props, &mut scope_ref);
                        }
                    }
                }
            }
            if should_update {
                let mut next_frame = VNode::from(component.view());
                // Re-rendering the tree
                let node =
                    next_frame.apply(element.as_node(), None, last_frame.take(), self.get_env());
                if let Some(ref mut cell) = occupied {
                    *cell.borrow_mut() = node;
                }
                last_frame = Some(next_frame);
            }
        };
        // Initial call for first rendering
        callback();
        js! { @(no_return)
            var bind = @{bind};
            var callback = @{callback};
            bind.loop = callback;
        }
    }
}

/// This handle keeps the reference to a detached scope to prevent memory leaks.
pub struct ScopeHandle {
    bind: Value,
}

impl ScopeHandle {
    /// Destroy the scope (component's loop).
    pub fn destroy(self) {
        let bind = &self.bind;
        js! { @(no_return)
            var destroy = function() {
                var bind = @{bind};
                bind.loop.drop();
                bind.loop = function() { };
            };
            setTimeout(destroy, 0);
        }
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
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

                fn attach(&mut self, element: &Element, mut sender: ScopeSender<CTX, COMP>)
                    -> EventListenerHandle {
                    let handler = self.0.take().expect("tried to attach listener twice");
                    let this = element.clone();
                    let listener = move |event: $type| {
                        debug!("Event handler: {}", stringify!($type));
                        event.stop_propagation();
                        let handy_event: $ret = $convert(&this, event);
                        let msg = handler(handy_event);
                        let update = ComponentUpdate::Message(msg);
                        sender.send(update);
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
    /* TODO Add PR to https://github.com/koute/stdweb
    onmousedown(event: MouseDownEvent) -> () => |_, _| { () }
    onmouseup(event: MouseUpEvent) -> () => |_, _| { () }
    onmouseenter(event: MouseEnterEvent) -> () => |_, _| { () }
    onmouseleave(event: MouseLeaveEvent) -> () => |_, _| { () }
    onmouseover(event: MouseOverEvent) -> () => |_, _| { () }
    onmouseout(event: MouseOutEvent) -> () => |_, _| { () }
    */
    onblur(event: BlurEvent) -> BlurData => |_, event| {
        let event = BlurEvent::from(event);
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
