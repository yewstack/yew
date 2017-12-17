use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/*
pub trait Message {}

impl<T: ConcreteEvent> Fn(T) -> Self for Message {
}
*/

use stdweb;

use stdweb::web::{
    INode,
    IEventTarget,
    IElement,
    Element,
    document,
};

use stdweb::web::event::{
    ClickEvent,
};

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
        println!("Process messages");
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
    fn attach(&mut self, element: &Element, messages: Messages<MSG>);
}

type Messages<MSG> = Rc<RefCell<Vec<MSG>>>;
type Listeners<MSG> = Vec<Box<Listener<MSG>>>;
pub type Tags<MSG> = Vec<Node<MSG>>;
type Classes = Vec<&'static str>;

pub enum Node<MSG> {
    Tag {
        tag: &'static str,
        listeners: Listeners<MSG>,
        childs: Vec<Node<MSG>>,
        classes: Classes,
    },
    Text {
        text: String,
    },
}

impl<MSG> fmt::Debug for Node<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Node::Tag { ref tag, .. } => {
                write!(f, "Node::Tag {{ tag: {} }}", tag)
            }
            &Node::Text { ref text, .. } => {
                write!(f, "Node::Text {{ text: {} }}", text)
            }
        }
    }
}

impl<MSG> Node<MSG> {
    pub fn new(tag: &'static str) -> Self {
        //, classes: Classes, listeners: Listeners<MSG>, childs: Tags<MSG>) -> Self {
        Node::Tag {
            tag: tag,
            classes: Vec::new(),
            listeners: Vec::new(),
            childs: Vec::new(),
        }
    }

    pub fn new_text(text: &'static str) -> Self {
        Node::Text {
            text: text.to_string(),
        }
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

    fn render(self, messages: Messages<MSG>, element: &Element) {
        match self {
            Node::Tag { tag, classes, listeners, mut childs } => {
                let child_element = document().create_element(tag);
                for class in classes {
                    child_element.class_list().add(&class);
                }
                for mut listener in listeners {
                    listener.attach(&child_element, messages.clone());
                }
                for child in childs.drain(..) {
                    child.render(messages.clone(), &child_element);
                }
                element.append_child(&child_element);
            },
            Node::Text { text } => {
                let child_element = document().create_text_node(&text);
                element.append_child(&child_element);
            },
        }
    }
}

/*
pub fn div<MSG>(classes: Classes, listeners: Listeners<MSG>, tags: Tags<MSG>) -> Node<MSG> {
    Node::new("div", classes, listeners, tags)
}

pub fn button<MSG>(classes: Classes, listeners: Listeners<MSG>, tags: Tags<MSG>) -> Node<MSG> {
    Node::new("button", classes, listeners, tags)
}

pub fn text<MSG>(text: &str) -> Node<MSG> {
    Node::Text {
        text: text.to_string(),
    }
}
*/

pub fn onclick<F, MSG>(handler: F) -> Box<Listener<MSG>>
where
    MSG: 'static,
    F: Fn(ClickEvent) -> MSG + 'static
{
    Box::new(OnClick(Some(handler)))
}

struct OnClick<T>(Option<T>);

impl<T, MSG> Listener<MSG> for OnClick<T>
where
    MSG: 'static,
    T: Fn(ClickEvent) -> MSG + 'static,
{
    fn attach(&mut self, element: &Element, messages: Messages<MSG>) {
        let handler = self.0.take().unwrap();
        let sender = move |event| {
            println!("Clicked!");
            let msg = handler(event);
            messages.borrow_mut().push(msg);
            js! {
                yew_loop();
            }
        };
        element.add_event_listener(sender);
    }
}

