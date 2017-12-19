use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

/*
pub trait Message {}

impl<T: ConcreteEvent> Fn(T) -> Self for Message {
}
*/

use stdweb;

use stdweb::web::{INode, IElement, Element, document};
use stdweb::web::event::{IMouseEvent, IKeyboardEvent};
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;

// Diff updates text only!

type Ref<'a, MSG> = &'a Child<MSG>;

fn diff_node<'a, MSG>(
    list: &mut Vec<Patch<'a, MSG>>,
    _a: Ref<'a, MSG>,
    _b: Ref<'a, MSG>) {
}

// Index of Vec is an index of patch
fn diff<'a, MSG>(a: Ref<'a, MSG>, b: Ref<'a, MSG>) -> Vec<Patch<'a, MSG>> {
    let mut list = Vec::new();
    let mut patch = Patch::new(&a);
    diff_node(&mut list, a, b);
    list
}

struct Patch<'a, MSG: 'a> {
    nref: Ref<'a, MSG>,
    actions: Vec<Action>,
}

impl<'a, MSG: 'a> Patch<'a, MSG> {
    fn new(nref: Ref<'a, MSG>) -> Self {
        let actions = Vec::new();
        Patch { nref, actions }
    }
}

enum Action {
    PutNode,
    SetValue(String),
    SetAttribute(String, String),
}

pub fn program<M, MSG, U, V>(mut model: M, update: U, view: V)
where
    M: 'static,
    MSG: 'static,
    U: Fn(&mut M, MSG) + 'static,
    V: Fn(&M) -> Html<MSG> + 'static,
{
    stdweb::initialize();
    // No messages at start
    let messages = Rc::new(RefCell::new(Vec::new()));
    let body = document().query_selector("body").unwrap();
    let mut onscreen_scene = Child::from(view(&model));
    onscreen_scene.render(messages.clone(), &body);

    let mut callback = move || {
        let mut borrowed = messages.borrow_mut();
        for msg in borrowed.drain(..) {
            update(&mut model, msg);
        }
        let offscreen_scene = Child::from(view(&model));
        {
            let _patch = diff(&onscreen_scene, &offscreen_scene);
        }
        onscreen_scene = offscreen_scene;
        while body.has_child_nodes() {
            body.remove_child(&body.last_child().unwrap()).unwrap();
        }
        onscreen_scene.render(messages.clone(), &body);
    };
    // Initial call for first rendering
    callback();
    js! {
        let callback = @{callback};
        window.yew_loop = function() {
            callback();
        }
    };
    stdweb::event_loop();
}

pub type Html<MSG> = VNode<MSG>;

pub trait Listener<MSG> {
    fn kind(&self) -> &'static str;
    fn attach(&mut self, element: &Element, messages: Messages<MSG>);
}

impl<MSG> fmt::Debug for Listener<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

type Messages<MSG> = Rc<RefCell<Vec<MSG>>>;
type Listeners<MSG> = Vec<Box<Listener<MSG>>>;
type Attributes = HashMap<&'static str, String>;
type Classes = Vec<&'static str>;

trait Render<MSG> {
    fn render(&mut self, messages: Messages<MSG>, element: &Element);
}

pub enum Child<MSG> {
    VNode(VNode<MSG>),
    VText(VText),
}


impl<MSG, T: ToString> From<T> for Child<MSG> {
    fn from(value: T) -> Self {
        Child::VText(VText::new(value))
    }
}

impl<MSG> fmt::Debug for Child<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Child::VNode(ref vnode) => vnode.fmt(f),
            &Child::VText(ref vtext) => vtext.fmt(f),
        }
    }
}


impl<MSG> Render<MSG> for Child<MSG> {
    fn render(&mut self, messages: Messages<MSG>, element: &Element) {
        match *self {
            Child::VNode(ref mut vnode) => vnode.render(messages, element),
            Child::VText(ref mut vtext) => vtext.render(messages, element),
        }
    }
}

impl<MSG> From<VText> for Child<MSG> {
    fn from(vtext: VText) -> Self {
        Child::VText(vtext)
    }
}

impl<MSG> From<VNode<MSG>> for Child<MSG> {
    fn from(vnode: VNode<MSG>) -> Self {
        Child::VNode(vnode)
    }
}

pub struct VText {
    text: String,
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

impl VText {
    pub fn new<T: ToString>(text: T) -> Self {
        VText { text: text.to_string() }
    }
}

impl<MSG> Render<MSG> for VText {
    fn render(&mut self, _: Messages<MSG>, element: &Element) {
        let child_element = document().create_text_node(&self.text);
        element.append_child(&child_element);
    }
}


pub struct VNode<MSG> {
    tag: &'static str,
    listeners: Listeners<MSG>,
    attributes: Attributes,
    childs: Vec<Child<MSG>>,
    classes: Classes,
    value: Option<String>,
}

impl<MSG> fmt::Debug for VNode<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VNode {{ tag: {} }}", self.tag)
    }
}

impl<MSG> VNode<MSG> {
    pub fn new(tag: &'static str) -> Self {
        VNode {
            tag: tag,
            classes: Vec::new(),
            attributes: HashMap::new(),
            listeners: Vec::new(),
            childs: Vec::new(),
            value: None,
        }
    }

    pub fn tag(&self) -> &'static str {
        self.tag
    }

    pub fn add_child(&mut self, child: Child<MSG>) {
        self.childs.push(child);
    }

    pub fn add_classes(&mut self, class: &'static str) {
        self.classes.push(class);
    }

    pub fn set_value<T: ToString>(&mut self, value: &T) {
        self.value = Some(value.to_string());
    }

    pub fn add_attribute<T: ToString>(&mut self, name: &'static str, value: T) {
        self.attributes.insert(name, value.to_string());
    }

    pub fn add_listener(&mut self, listener: Box<Listener<MSG>>) {
        self.listeners.push(listener);
    }
}

impl<MSG> Render<MSG> for VNode<MSG> {
    fn render(&mut self, messages: Messages<MSG>, element: &Element) {
        let child_element = document().create_element(self.tag);
        let child_element = {
            let cloned: Result<InputElement, _> = child_element.clone().try_into();
            if let &Some(ref value) = &self.value {
                if let Ok(input_element) = cloned {
                    input_element.set_value(value);
                    input_element.into()
                } else {
                    child_element
                }
            } else {
                child_element
            }
        };
        for (name, value) in self.attributes.iter() {
            set_attribute(&child_element, name, &value);
        }
        for class in self.classes.iter() {
            child_element.class_list().add(&class);
        }
        for mut listener in self.listeners.drain(..) {
            listener.attach(&child_element, messages.clone());
        }
        for mut child in self.childs.drain(..) {
            child.render(messages.clone(), &child_element);
        }
        element.append_child(&child_element);
    }
}

macro_rules! impl_action {
    ($($action:ident($event:ident : $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        pub mod $action {
            use stdweb::web::{IEventTarget, Element};
            use stdweb::web::event::{IEvent, $type};
            use super::*;

            pub struct Wrapper<F>(Option<F>);

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

            impl<T, MSG> Listener<MSG> for Wrapper<T>
            where
                MSG: 'static,
                T: Fn($ret) -> MSG + 'static,
            {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&mut self, element: &Element, messages: Messages<MSG>) {
                    let handler = self.0.take().unwrap();
                    let this = element.clone();
                    let sender = move |event: $type| {
                        event.stop_propagation();
                        let handy_event: $ret = $convert(&this, event);
                        let msg = handler(handy_event);
                        messages.borrow_mut().push(msg);
                        js! {
                            // Schedule to call the loop handler
                            // IMPORTANT! If call loop function immediately
                            // it stops handling other messages and the first
                            // one will be fired.
                            setTimeout(yew_loop);
                        }
                    };
                    element.add_event_listener(sender);
                }
            }
        }
    )*};
}

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(event: ClickEvent) -> MouseData => |_, event| { MouseData::from(event) }
    ondoubleclick(event: DoubleClickEvent) -> MouseData => |_, event| { MouseData::from(event) }
    onkeypress(event: KeypressEvent) -> KeyData => |_, event| { KeyData::from(event) }
    /* TODO Add PR to https://github.com/koute/stdweb
    onmousedown(event: MouseDownEvent) -> () => |_, _| { () }
    onmouseup(event: MouseUpEvent) -> () => |_, _| { () }
    onmouseenter(event: MouseEnterEvent) -> () => |_, _| { () }
    onmouseleave(event: MouseLeaveEvent) -> () => |_, _| { () }
    onmouseover(event: MouseOverEvent) -> () => |_, _| { () }
    onmouseout(event: MouseOutEvent) -> () => |_, _| { () }
    */
    oninput(event: InputEvent) -> InputData => |this: &Element, _| {
        use stdweb::web::html_element::InputElement;
        use stdweb::unstable::TryInto;
        let input: InputElement = this.clone().try_into().unwrap();
        let value = input.value().into_string().unwrap_or_else(|| "".into());
        InputData { value }
    }
}

pub struct MouseData {
    pub screen_x: f64,
    pub screen_y: f64,
    pub client_x: f64,
    pub client_y: f64,
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

pub struct InputData {
    pub value: String,
}

pub struct KeyData {
    pub key: String,
}

impl<T: IKeyboardEvent> From<T> for KeyData {
    fn from(event: T) -> Self {
        KeyData {
            key: event.key(),
        }
    }
}

// stdweb doesn't have methods to work with attributes
// this is workaround from: https://github.com/koute/stdweb/issues/16#issuecomment-325195854
fn set_attribute(element: &Element, name: &str, value: &str) {
    js!( @{element}.setAttribute( @{name}, @{value} ); );
}

