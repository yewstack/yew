use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use stdweb;

use stdweb::Reference;
use stdweb::web::{INode, IElement, Node, Element, TextNode, document};
use stdweb::web::event::{IMouseEvent, IKeyboardEvent};
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;

macro_rules! debug {
    ($($e:expr),*) => {
        if cfg!(debug) {
            println!($($e,)*);
        }
    };
}

macro_rules! warn {
    ($($e:expr),*) => {
        eprintln!($($e,)*);
    };
}

fn clear_body() {
    let body = document().query_selector("body").unwrap();
    while body.has_child_nodes() {
        body.remove_child(&body.last_child().unwrap()).unwrap();
    }
}

pub fn program<M, MSG, U, V>(mut model: M, update: U, view: V)
where
    M: 'static,
    MSG: 'static,
    U: Fn(&mut M, MSG) + 'static,
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

    let mut callback = move || {
        debug!("Yew Loop Callback");
        let mut borrowed = messages.borrow_mut();
        for msg in borrowed.drain(..) {
            update(&mut model, msg);
        }
        let mut next_frame = VNode::from(view(&model));
        debug!("Do apply");
        next_frame.apply(&body, last_frame.take(), messages.clone());
        last_frame = Some(next_frame);
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

pub type Html<MSG> = VTag<MSG>;

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
type Classes = HashSet<&'static str>;

/// Bind virtual element to a DOM reference.
pub enum VNode<MSG> {
    VTag {
        reference: Option<Element>,
        vtag: VTag<MSG>,
    },
    VText {
        reference: Option<TextNode>, // TODO Replace with TextNode
        vtext: VText,
    },
}


impl<MSG, T: ToString> From<T> for VNode<MSG> {
    fn from(value: T) -> Self {
        VNode::VText {
            reference: None,
            vtext: VText::new(value),
        }
    }
}

impl<MSG> fmt::Debug for VNode<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VNode::VTag { ref vtag, .. } => vtag.fmt(f),
            &VNode::VText { ref vtext, .. } => vtext.fmt(f),
        }
    }
}

impl<MSG> VNode<MSG> {
    fn remove<T: INode>(self, parent: &T) {
        let opt_ref: Option<Node> = {
            match self {
                VNode::VTag { reference, .. } => reference.map(Node::from),
                VNode::VText { reference, .. } => reference.map(Node::from),
            }
        };
        if let Some(node) = opt_ref {
            if let Err(_) = parent.remove_child(&node) {
                warn!("Node not found to remove: {:?}", node);
            }
        }
    }

    fn apply<T: INode>(&mut self, parent: &T, last: Option<VNode<MSG>>, messages: Messages<MSG>) {
        match *self {
            VNode::VTag { ref mut vtag, ref mut reference } => {
                let left = vtag;
                let mut right = None;
                match last {
                    Some(VNode::VTag { mut vtag, reference: Some(mut element) }) => {
                        // Copy reference from right to left (as is)
                        right = Some(vtag);
                        *reference = Some(element);
                    }
                    Some(VNode::VText { reference: Some(old), .. }) => {
                        let mut element = document().create_element(left.tag);
                        parent.replace_child(&element, &old);
                        left.render(&mut element, None, messages.clone());
                        *reference = Some(element);
                    }
                    Some(VNode::VTag { reference: None, .. }) |
                    Some(VNode::VText { reference: None, .. }) |
                    None => {
                        let mut element = document().create_element(left.tag);
                        parent.append_child(&element);
                        left.render(&mut element, None, messages.clone());
                        *reference = Some(element);
                    }
                }
                let element_mut = reference.as_mut().expect("vtag must be here");
                // Update parameters
                let mut rights = {
                    if let Some(ref mut right) = right {
                        right.childs.drain(..).map(Some).collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    }
                };
                // TODO Consider to use: &mut Messages here;
                left.render(element_mut, right, messages.clone());
                let mut lefts = left.childs.iter_mut().map(Some).collect::<Vec<_>>();
                // Process children
                let diff = lefts.len() as i32 - rights.len() as i32;
                if diff > 0 {
                    for _ in 0..diff {
                        rights.push(None);
                    }
                } else if diff < 0 {
                    for _ in 0..-diff {
                        lefts.push(None);
                    }
                }
                for pair in lefts.into_iter().zip(rights) {
                    match pair {
                        (Some(left), right) => {
                            left.apply(element_mut, right, messages.clone());
                        }
                        (None, Some(right)) => {
                            right.remove(element_mut);
                        }
                        (None, None) => {
                            panic!("redundant iterations during diff");
                        }
                    }
                }
                //vtag.apply(parent, reference, last, messages);
            }
            VNode::VText { ref mut vtext, ref mut reference }  => {
                match last {
                    Some(VNode::VTag { .. }) => {
                    }
                    Some(VNode::VText { .. }) => {
                        // TODO Replace the node
                    }
                    Some(VNode::VTag { reference: None, .. }) |
                    Some(VNode::VText { reference: None, .. }) |
                    None => {
                        let element = document().create_text_node(&vtext.text);
                        parent.append_child(&element);
                        *reference = Some(element);
                    }
                }
                //vtext.apply(parent, reference, last, messages);
            }
        }
    }
}

impl<MSG> From<VText> for VNode<MSG> {
    fn from(vtext: VText) -> Self {
        VNode::VText {
            reference: None,
            vtext,
        }
    }
}

impl<MSG> From<VTag<MSG>> for VNode<MSG> {
    fn from(vtag: VTag<MSG>) -> Self {
        VNode::VTag {
            reference: None,
            vtag,
        }
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

    fn apply<MSG, T: INode>(&mut self, parent: &T, opposite: Option<Self>, _: Messages<MSG>) {
        /*
        debug!("Render text node!");
        if let Some(_) = this {
            // Check node type and replace if wrong
        } else {
            let element = document().create_text_node(&self.text);
            parent.append_child(&element);
        }
        */
    }
}

pub struct VTag<MSG> {
    tag: &'static str,
    listeners: Listeners<MSG>,
    attributes: Attributes,
    childs: Vec<VNode<MSG>>,
    classes: Classes,
    value: Option<String>,
    kind: Option<String>,
}

impl<MSG> fmt::Debug for VTag<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VTag {{ tag: {} }}", self.tag)
    }
}

enum Mutator<ID, T> {
    Add(ID, T),
    Replace(ID, T),
    Remove(ID),
}

impl<MSG> VTag<MSG> {
    pub fn new(tag: &'static str) -> Self {
        VTag {
            tag: tag,
            classes: Classes::new(),
            attributes: HashMap::new(),
            listeners: Vec::new(),
            childs: Vec::new(),
            value: None,
            kind: None,
        }
    }

    pub fn tag(&self) -> &'static str {
        self.tag
    }

    pub fn add_child(&mut self, child: VNode<MSG>) {
        self.childs.push(child);
    }

    pub fn add_classes(&mut self, class: &'static str) {
        self.classes.insert(class);
    }

    pub fn set_value<T: ToString>(&mut self, value: &T) {
        self.value = Some(value.to_string());
    }

    pub fn set_kind<T: ToString>(&mut self, value: T) {
        self.kind = Some(value.to_string());
    }

    pub fn add_attribute<T: ToString>(&mut self, name: &'static str, value: T) {
        self.attributes.insert(name, value.to_string());
    }

    pub fn add_listener(&mut self, listener: Box<Listener<MSG>>) {
        self.listeners.push(listener);
    }

    fn soakup_classes(&mut self, ancestor: &mut Option<Self>) -> Vec<Mutator<&'static str, ()>> {
        let mut changes = Vec::new();
        if let &mut Some(ref ancestor) = ancestor {
            let to_add = self.classes
                .difference(&ancestor.classes)
                .map(|class| Mutator::Add(*class, ()));
            changes.extend(to_add);
            let to_remove = ancestor.classes
                .difference(&self.classes)
                .map(|class| Mutator::Remove(*class));
            changes.extend(to_remove);
        } else {
            // Add everything
            let to_add = self.classes.iter().map(|class| Mutator::Add(*class, ()));
            changes.extend(to_add);
        }
        changes
    }

    fn soakup_attributes(&mut self, ancestor: &mut Option<Self>) -> Vec<Mutator<String, String>> {
        let mut changes = Vec::new();
        if let &mut Some(ref mut ancestor) = ancestor {
            let left_keys = self.attributes.keys().collect::<HashSet<_>>();
            let right_keys = ancestor.attributes.keys().collect::<HashSet<_>>();
            let to_add = left_keys
                .difference(&right_keys)
                .map(|key| {
                    let value = self.attributes.get(*key).unwrap();
                    Mutator::Add(key.to_string(), value.to_string())
                });
            changes.extend(to_add);
            for key in left_keys.intersection(&right_keys) {
                let left_value = self.attributes.get(*key).unwrap();
                let right_value = ancestor.attributes.get(*key).unwrap();
                if left_value != right_value {
                    let mutator = Mutator::Replace(key.to_string(), left_value.to_string());
                    changes.push(mutator);
                }
            }
            let to_remove = right_keys
                .difference(&left_keys)
                .map(|key| Mutator::Remove(key.to_string()));
            changes.extend(to_remove);
        } else {
            for (key, value) in self.attributes.iter() {
                let mutator = Mutator::Add(key.to_string(), value.to_string());
                changes.push(mutator);
            }
        }
        changes
    }

    fn soakup_kind(&mut self, ancestor: &mut Option<Self>) -> Option<Mutator<String, ()>> {
        match (&self.kind, ancestor.as_mut().and_then(|anc| anc.kind.take())) {
            (&Some(ref left), Some(ref right)) => {
                if left != right {
                    Some(Mutator::Replace(left.to_string(), ()))
                } else {
                    None
                }
            }
            (&Some(ref left), None) => {
                Some(Mutator::Add(left.to_string(), ()))
            }
            (&None, Some(right)) => {
                Some(Mutator::Remove(right))
            }
            (&None, None) => {
                None
            }
        }
    }

    fn soakup_value(&mut self, ancestor: &mut Option<Self>) -> Option<Mutator<String, ()>> {
        match (&self.value, ancestor.as_mut().and_then(|anc| anc.value.take())) {
            (&Some(ref left), Some(ref right)) => {
                if left != right {
                    Some(Mutator::Replace(left.to_string(), ()))
                } else {
                    None
                }
            }
            (&Some(ref left), None) => {
                Some(Mutator::Add(left.to_string(), ()))
            }
            (&None, Some(right)) => {
                Some(Mutator::Remove(right))
            }
            (&None, None) => {
                None
            }
        }
    }

    /*
    fn fill_node(&mut self, this: &Element, messages: Messages<MSG>) {
        debug!("VTag listeners");
        // TODO IMPORTANT! IT DUPLICATES ALL LISTENERS!
        // How to fix? What about to use "global" list of
        // listeners mapping by dom references.
        for mut listener in self.listeners.drain(..) {
            listener.attach(&this, messages.clone());
        }
    }
    */
}

impl<MSG> VTag<MSG> {
    fn render(&mut self, subject: &Element, mut opposite: Option<VTag<MSG>>, messages: Messages<MSG>) {
        // TODO Replace self if tagName differs

        let changes = self.soakup_classes(&mut opposite);
        for change in changes {
            let list = subject.class_list();
            match change {
                Mutator::Add(class, _) | Mutator::Replace(class, _) => {
                    list.add(&class);
                }
                Mutator::Remove(class) => {
                    list.remove(&class);
                }
            }
        }

        let changes = self.soakup_attributes(&mut opposite);
        for change in changes {
            let list = subject.class_list();
            match change {
                Mutator::Add(key, value) | Mutator::Replace(key, value) => {
                    set_attribute(&subject, &key, &value);
                }
                Mutator::Remove(key) => {
                    remove_attribute(&subject, &key);
                }
            }
        }

        if let Some(change) = self.soakup_kind(&mut opposite) {
            let input: Result<InputElement, _> = subject.clone().try_into();
            if let Ok(input) = input {
                match change {
                    Mutator::Add(kind, _) | Mutator::Replace(kind, _) => {
                        input.set_kind(&kind);
                    }
                    Mutator::Remove(kind) => {
                        input.set_kind("");
                    }
                }
            } else {
                panic!("tried to set `type` kind for non input element");
            }
        }

        if let Some(change) = self.soakup_value(&mut opposite) {
            let input: Result<InputElement, _> = subject.clone().try_into();
            if let Ok(input) = input {
                match change {
                    Mutator::Add(kind, _) | Mutator::Replace(kind, _) => {
                        input.set_value(&kind);
                    }
                    Mutator::Remove(kind) => {
                        input.set_value("");
                    }
                }
            } else {
                panic!("tried to set `value` kind for non input element");
            }
        }

        // TODO Actually, it duplicates listeners on every call
        for mut listener in self.listeners.drain(..) {
            listener.attach(&subject, messages.clone());
        }
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
                        debug!("Event handler: {}", stringify!($type));
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

fn remove_attribute(element: &Element, name: &str) {
    js!( @{element}.removeAttribute( @{name} ); );
}

