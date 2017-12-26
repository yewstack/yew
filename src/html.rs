use stdweb;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver, channel};
use stdweb::web::{INode, EventListenerHandle, document};
use stdweb::web::event::{IMouseEvent, IKeyboardEvent};
use virtual_dom::{VNode, VTag, Messages, Listener};

fn clear_body() {
    let body = document().query_selector("body").unwrap();
    while body.has_child_nodes() {
        body.remove_child(&body.last_child().unwrap()).unwrap();
    }
}

pub struct ContextSender<MSG> {
    tx: Sender<MSG>,
}

impl<MSG> ContextSender<MSG> {
    pub fn send(&mut self, msg: MSG) {
        self.tx.send(msg).expect("Context lost the receiver!");
        schedule_update();
    }
}

pub struct Context<MSG> {
    tx: Sender<MSG>,
    rx: Receiver<MSG>,
}

impl<MSG> Context<MSG> {
    fn new() -> Self {
        let (tx, rx) = channel();
        Context { tx, rx }
    }

    pub fn sender(&mut self) -> ContextSender<MSG> {
        ContextSender {
            tx: self.tx.clone(),
        }
    }
}

pub fn program<M, MSG, U, V>(mut model: M, update: U, view: V)
where
    M: 'static,
    MSG: 'static,
    U: Fn(&mut Context<MSG>, &mut M, MSG) + 'static,
    V: Fn(&M) -> Html<MSG> + 'static,
{
    stdweb::initialize();
    clear_body();
    let body = document().query_selector("body").unwrap();
    // No messages at start
    let messages = Rc::new(RefCell::new(Vec::new()));
    let mut last_frame = VNode::from(view(&model));
    last_frame.apply(&body, None, messages.clone());
    let mut last_frame = Some(last_frame);
    let mut context = Context::new();

    let mut callback = move || {
        debug!("Yew Loop Callback");
        let mut borrowed = messages.borrow_mut();
        borrowed.extend(context.rx.try_iter());
        for msg in borrowed.drain(..) {
            update(&mut context, &mut model, msg);
        }
        let mut next_frame = VNode::from(view(&model));
        debug!("Do apply");
        next_frame.apply(&body, last_frame.take(), messages.clone());
        last_frame = Some(next_frame);
    };
    // Initial call for first rendering
    callback();
    js! {
        var callback = @{callback};
        window.yew_loop = function() {
            callback();
        }
    };
    stdweb::event_loop();
}

pub type Html<MSG> = VTag<MSG>;

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

                fn attach(&mut self, element: &Element, messages: Messages<MSG>)
                    -> EventListenerHandle {
                    let handler = self.0.take().unwrap();
                    let this = element.clone();
                    let sender = move |event: $type| {
                        debug!("Event handler: {}", stringify!($type));
                        event.stop_propagation();
                        let handy_event: $ret = $convert(&this, event);
                        let msg = handler(handy_event);
                        messages.borrow_mut().push(msg);
                        schedule_update();
                    };
                    element.add_event_listener(sender)
                }
            }
        }
    )*};
}

/// Use `ContextSender::send` to emit it implicit
fn schedule_update() {
    js! {
        // Schedule to call the loop handler
        // IMPORTANT! If call loop function immediately
        // it stops handling other messages and the first
        // one will be fired.
        setTimeout(yew_loop);
    }
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InputData {
    pub value: String,
}

#[derive(Debug)]
pub struct KeyData {
    pub key: String,
}

impl<T: IKeyboardEvent> From<T> for KeyData {
    fn from(event: T) -> Self {
        KeyData { key: event.key() }
    }
}

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
        Href { link: link.to_owned() }
    }
}

impl ToString for Href {
    fn to_string(&self) -> String {
        self.link.to_owned()
    }
}

