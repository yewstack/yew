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
    let mut callback = move || {
        let mut borrowed = messages.borrow_mut();
        for msg in borrowed.drain(..) {
            update(&mut model, msg);
        }
        let html = view(&model);
        let body = document().query_selector("body").unwrap();
        while body.has_child_nodes() {
            body.remove_child(&body.last_child().unwrap()).unwrap();
        }
        html.render(messages.clone(), &body);
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

pub type Html<MSG> = Node<MSG>;

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
pub type Tags<MSG> = Vec<Node<MSG>>;
type Classes = Vec<&'static str>;

pub enum Node<MSG> {
    Tag {
        tag: &'static str,
        listeners: Listeners<MSG>,
        attributes: Attributes,
        childs: Vec<Node<MSG>>,
        classes: Classes,
    },
    Text { text: String },
}

impl<T: ToString, MSG> From<T> for Node<MSG> {
    fn from(value: T) -> Self {
        Node::new_text(value)
    }
}

impl<MSG> fmt::Debug for Node<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Node::Tag { ref tag, .. } => write!(f, "Node::Tag {{ tag: {} }}", tag),
            &Node::Text { ref text, .. } => write!(f, "Node::Text {{ text: {} }}", text),
        }
    }
}

impl<MSG> Node<MSG> {
    pub fn new(tag: &'static str) -> Self {
        Node::Tag {
            tag: tag,
            classes: Vec::new(),
            attributes: HashMap::new(),
            listeners: Vec::new(),
            childs: Vec::new(),
        }
    }

    pub fn new_text<T: ToString>(text: T) -> Self {
        Node::Text { text: text.to_string() }
    }

    pub fn tag(&self) -> Option<&'static str> {
        if let &Node::Tag { tag, .. } = self {
            Some(tag)
        } else {
            None
        }
    }

    pub fn add_child(&mut self, node: Node<MSG>) {
        match self {
            &mut Node::Tag { ref mut childs, .. } => {
                childs.push(node);
            }
            &mut Node::Text { .. } => {
                panic!("attempt to add child to text node");
            }
        }
    }

    pub fn add_classes(&mut self, class: &'static str) {
        match self {
            &mut Node::Tag { ref mut classes, .. } => {
                classes.push(class);
            }
            &mut Node::Text { .. } => {
                panic!("attempt to set class to text node");
            }
        }
    }

    pub fn add_attribute<T: ToString>(&mut self, name: &'static str, value: T) {
        match self {
            &mut Node::Tag { ref mut attributes, .. } => {
                attributes.insert(name, value.to_string());
            }
            &mut Node::Text { .. } => {
                panic!("attempt to set attribute to text node");
            }
        }
    }

    pub fn add_listener(&mut self, listener: Box<Listener<MSG>>) {
        match self {
            &mut Node::Tag { ref mut listeners, .. } => {
                listeners.push(listener);
            }
            &mut Node::Text { .. } => {
                panic!("attempt to add listener to text node");
            }
        }
    }

    fn render(self, messages: Messages<MSG>, element: &Element) {
        match self {
            Node::Tag {
                tag,
                classes,
                attributes,
                mut listeners,
                mut childs,
            } => {
                let child_element = document().create_element(tag);
                for (name, value) in attributes {
                    set_attribute(&child_element, name, &value);
                }
                for class in classes {
                    child_element.class_list().add(&class);
                }
                for mut listener in listeners.drain(..) {
                    listener.attach(&child_element, messages.clone());
                }
                for child in childs.drain(..) {
                    child.render(messages.clone(), &child_element);
                }
                element.append_child(&child_element);
            }
            Node::Text { text } => {
                let child_element = document().create_text_node(&text);
                element.append_child(&child_element);
            }
        }
    }
}

macro_rules! impl_action {
    ($($action:ident($event:ident : $type:ident) -> ($($arg:ident : $ret:ident),*) => $convert:expr)*) => {$(
        pub mod $action {
            use stdweb::web::{IEventTarget, Element};
            use stdweb::web::event::$type;
            use super::*;

            pub struct Wrapper<T>(Option<T>);

            impl<F, MSG> From<F> for Wrapper<F>
            where
                MSG: 'static,
                F: Fn($($ret),*) -> MSG + 'static,
            {
                fn from(handler: F) -> Self {
                    Wrapper(Some(handler))
                }
            }

            impl<T, MSG> Listener<MSG> for Wrapper<T>
            where
                MSG: 'static,
                T: Fn($($ret),*) -> MSG + 'static,
            {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&mut self, element: &Element, messages: Messages<MSG>) {
                    let handler = self.0.take().unwrap();
                    let this = element.clone();
                    let sender = move |event: $type| {
                        let ($($arg,)*): ($($ret,)*) = $convert(&this, event);
                        let msg = handler($($arg),*);
                        messages.borrow_mut().push(msg);
                        js! { yew_loop(); }
                    };
                    element.add_event_listener(sender);
                }
            }
        }
    )*};
}

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(event: ClickEvent) -> () => |_, _| { () }
    ondoubleclick(event: DoubleClickEvent) -> () => |_, _| { () }
    /* TODO Add PR to https://github.com/koute/stdweb
    onmousedown(event: MouseDownEvent) -> () => |_, _| { () }
    onmouseup(event: MouseUpEvent) -> () => |_, _| { () }
    onmouseenter(event: MouseEnterEvent) -> () => |_, _| { () }
    onmouseleave(event: MouseLeaveEvent) -> () => |_, _| { () }
    onmouseover(event: MouseOverEvent) -> () => |_, _| { () }
    onmouseout(event: MouseOutEvent) -> () => |_, _| { () }
    */
    oninput(event: InputEvent) -> (data: String) => |this: &Element, _| {
        use stdweb::web::html_element::InputElement;
        use stdweb::unstable::TryInto;
        let input: InputElement = this.clone().try_into().unwrap();
        (input.value().into_string().unwrap_or_else(|| "".into()),)
    }
}

// stdweb doesn't have methods to work with attributes
// this is workaround from: https://github.com/koute/stdweb/issues/16#issuecomment-325195854
fn set_attribute(element: &Element, name: &str, value: &str) {
    js!( @{element}.setAttribute( @{name}, @{value} ); );
}

