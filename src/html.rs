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
    Element,
    document,
};

use stdweb::web::event::{
    ClickEvent,
};

fn replace_body(element: Element) {
    let body = document().query_selector("body").unwrap();
    while body.has_child_nodes() {
        body.remove_child(&body.last_child().unwrap()).unwrap();
    }
    body.append_child(&element);
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
    let mut callback = move || {
        println!("Process messages");
        let mut borrowed = messages.borrow_mut();
        for msg in borrowed.drain(..) {
            update(&mut model, msg);
        }
        let html = view(&model);
        let element = html.render(messages.clone());
        replace_body(element);
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

pub type Html<MSG> = Tag<MSG>;

pub trait Listener<MSG> {
    fn attach(&mut self, element: &Element, messages: Messages<MSG>);
}

type Messages<MSG> = Rc<RefCell<Vec<MSG>>>;
type Listeners<MSG> = Vec<Box<Listener<MSG>>>;

pub struct Tag<MSG> {
    tag: &'static str,
    listeners: Listeners<MSG>,
}

impl<MSG> Tag<MSG> {
    fn new(tag: &'static str, listeners: Listeners<MSG>) -> Self {
        Tag { tag, listeners }
    }

    fn render(self, messages: Messages<MSG>) -> Element {
        let element = document().create_element(self.tag);
        for mut listener in self.listeners {
            listener.attach(&element, messages.clone());
        }
        element
    }
}

pub fn div<MSG>(listeners: Vec<Box<Listener<MSG>>>) -> Tag<MSG> {
    Tag::new("div", listeners)
}

pub fn button<MSG>(listeners: Vec<Box<Listener<MSG>>>) -> Tag<MSG> {
    Tag::new("button", listeners)
}

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

