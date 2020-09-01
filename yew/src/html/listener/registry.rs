// TODO: remove this
#![allow(dead_code)]

use crate::virtual_dom::{Listener, Listeners};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event};

thread_local! {
    /// Dedicated scheduler for all Registry operations to make their execution more compact
    #[allow(unused)]
    static SCHEDULER: RefCell<Scheduler<FIFO>> = Default::default();

    /// Global event listener registry
    static REGISTRY: RefCell<Registry> = Default::default();

    /// Key used to store listener id on element
    static LISTENER_ID_PROP: wasm_bindgen::JsValue = "__yew_listener_id".into();

    static BODY: web_sys::HtmlElement = web_sys::window()
        .expect("no window global")
        .document()
        .expect("no document on window")
        .body()
        .expect("no body on document")
}

#[derive(Clone, Copy, std::hash::Hash, Eq, PartialEq, Debug)]
struct EventDescriptor {
    kind: &'static str,
    flags: u8,
}

impl From<&dyn Listener> for EventDescriptor {
    fn from(l: &dyn Listener) -> Self {
        Self {
            kind: l.kind(),
            flags: l.flags(),
        }
    }
}

/// Prevents recursion by buffering calls in B
// TODO: Reuse this on yew::scheduler to reduce Rc and RefCell spam overhead + enable defer to RAF
#[derive(Default)]
pub struct Scheduler<B>
where
    B: Buffer,
{
    /// Prevents job recursion
    lock: RefCell<()>,

    /// Buffer for jobs that can not be executed at once but should be executed as soon as possible.
    synchronous: RefCell<B>,

    /// Buffer for jobs to be executed on the next animation frame
    // TODO: exec all sync jobs after this completes
    deferred: RefCell<B>,
    //
    // TODO: debounce using (EventDescriptor, listenerID). Need to figure out how to make this
    // generic.
    // TODO: exec all sync jobs after this completes
}

impl<B> Scheduler<B>
where
    B: Buffer,
{
    /// Schedule job with priority p for ASAP execution
    pub fn schedule(&self, p: B::Priority, job: impl FnOnce() + 'static) {
        // Allow only one job to run at a time
        match self.lock.try_borrow_mut() {
            Ok(_) => {
                job();
                Self::run_buffered(&self.synchronous);
            }
            _ => {
                Self::buffer(&self.synchronous, p, job);
            }
        }
    }

    /// Schedule job with priority p to be executed on the next animation frame
    pub fn schedule_deferred(&self, p: B::Priority, job: impl FnOnce() + 'static) {
        Self::buffer(&self.deferred, p, job);

        // TODO: Schedule render task, if none
    }

    fn buffer(dst: &RefCell<B>, p: B::Priority, job: impl FnOnce() + 'static) {
        dst.borrow_mut()
            .buffer(p, Box::new(job) as Box<dyn FnOnce()>);
    }

    /// Run any buffered jobs from src
    fn run_buffered(src: &RefCell<B>) {
        while let Some(job) = src.borrow_mut().next() {
            job();
        }
    }
}

/// Buffers incoming work and instructs Scheduler what to run next
pub trait Buffer: Default {
    /// Enables prioritizing some jobs over others
    type Priority;

    /// Buffer job with priority p for later execution
    fn buffer(&mut self, p: Self::Priority, job: Box<dyn FnOnce()>);

    /// Return next job to execute, if any
    fn next(&mut self) -> Option<Box<dyn FnOnce()>>;
}

#[derive(Default)]
struct FIFO(VecDeque<Box<dyn FnOnce()>>);

impl Buffer for FIFO {
    type Priority = ();

    fn buffer(&mut self, _: Self::Priority, job: Box<dyn FnOnce()>) {
        self.0.push_back(job);
    }

    fn next(&mut self) -> Option<Box<dyn FnOnce()>> {
        self.0.pop_front()
    }
}

/// Global multiplexing event handler registry
#[derive(Default, Debug)]
struct Registry {
    /// Counter for assigning new IDs
    id_counter: u64,

    /// Events with registered handlers that are possibly passive
    handling: HashSet<EventDescriptor>,

    /// Events that have been registered as bubbling at least once
    bubbling: HashSet<EventDescriptor>,

    /// Contains all registered event listeners by listener ID
    by_id: HashMap<u64, HashMap<EventDescriptor, Rc<dyn Listener>>>,

    /// Keep track of all listeners to drop them on registry drop.
    /// The registry is never dropped in production.
    #[cfg(test)]
    registered: Vec<(&'static str, Closure<dyn std::ops::Fn(web_sys::Event)>)>,
}

impl Registry {
    /// Run f with access to global Registry
    fn with<R>(mut f: impl FnMut(&mut Registry) -> R) -> R {
        REGISTRY.with(|r| f(&mut *r.borrow_mut()))
    }

    /// Register all passed listeners under ID
    fn register(&mut self, id: u64, listeners: Vec<Rc<dyn Listener>>) {
        let mut by_id = HashMap::with_capacity(listeners.len());
        for l in listeners.into_iter() {
            // Create global listener, if not yet created
            let key = EventDescriptor::from(&*l);

            if !self.handling.contains(&key) {
                BODY.with(|body| {
                    let cl =
                        Closure::wrap(Box::new(move |e: Event| Registry::handle(&key, e))
                            as Box<dyn Fn(Event)>);
                    AsRef::<web_sys::EventTarget>::as_ref(body)
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            &key.kind[2..],
                            cl.as_ref().unchecked_ref(),
                            &{
                                let mut opts = web_sys::AddEventListenerOptions::new();
                                if key.flags & crate::callback::PASSIVE != 0 {
                                    opts.passive(true);
                                }
                                opts
                            },
                        )
                        .map_err(|e| format!("could not register global listener: {:?}", e))
                        .unwrap();

                    // Never drop the closure as this event handler is static
                    #[cfg(not(test))]
                    cl.forget();
                    #[cfg(test)]
                    self.registered.push((&key.kind[2..], cl));
                });

                self.handling.insert(key);
            }

            if key.flags & crate::callback::HANDLE_BUBBLED != 0 {
                self.bubbling.insert(key);
            }

            by_id.insert(key, l);
        }
        self.by_id.insert(id, by_id);
    }

    /// Unregister any existing listeners for ID
    fn unregister(&mut self, id: &u64) {
        self.by_id.remove(id);
    }

    /// Set unique listener ID onto element and return it
    fn set_listener_id(&mut self, el: &Element) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;

        LISTENER_ID_PROP.with(|prop| {
            if !js_sys::Reflect::set(el, &prop, &js_sys::JsString::from(id.to_string())).unwrap() {
                panic!("failed to set listener ID property");
            }
        });

        id
    }

    /// Handle a global event firing
    fn handle(desc: &EventDescriptor, event: Event) {
        if let Some(l) = event
            .target()
            .map(|el| el.dyn_into::<web_sys::Element>().ok())
            .flatten()
            .map(|el| LISTENER_ID_PROP.with(|prop| js_sys::Reflect::get(&el, &prop).ok()))
            .flatten()
            .map(|v| v.dyn_into().ok())
            .flatten()
            .map(|v: js_sys::JsString| String::from(v).parse().ok())
            .flatten()
            .map(|id: u64| {
                Registry::with(|r| r.by_id.get(&id).map(|s| s.get(desc)).flatten().cloned())
            })
            .flatten()
        {
            // TODO: DEFER + tests
            // TODO: DEBOUNCE + tests
            // TODO: HANDLE_BUBBLED + tests

            l.handle(event);

            if Registry::with(|r| r.bubbling.contains(desc)) {
                // TODO: if not passive, check if default was prevented after each call
                // (including the first call above)
                // TODO: travel up the parents for event bubbling
            }
        }
    }
}

// Enable resetting between tests
#[cfg(test)]
impl Drop for Registry {
    fn drop(&mut self) {
        BODY.with(|body| {
            for (kind, cl) in std::mem::take(&mut self.registered) {
                AsRef::<web_sys::EventTarget>::as_ref(body)
                    .remove_event_listener_with_callback(kind, cl.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }
}

/// Register new event listeners to the element
pub(crate) fn set_listeners(el: &Element, new: &mut Listeners) {
    if let Listeners::Pending(ref mut pending) = new {
        if pending.is_empty() {
            return;
        }

        *new = Listeners::Registered(Registry::with(|reg| {
            let id = reg.set_listener_id(el);
            reg.register(id, std::mem::take(pending));
            id
        }));
    }
}

/// Register new event listeners to the element, replacing old ones
pub(crate) fn patch_listeners(mut new: &mut Listeners, old: &Listeners) {
    if let (Listeners::Pending(pending), Listeners::Registered(id)) = (&mut new, old) {
        Registry::with(|reg| {
            // The listeners should never hav pointer equality so always replacing them is a slight
            // optimisation
            reg.unregister(id);
            if pending.is_empty() {
                return;
            }
            reg.register(*id, std::mem::take(pending));
        });
        *new = Listeners::Registered(*id);
    }
}

/// Remove any registered event listeners from the element.
/// The listener ID should be reused, if the element is reused.
pub(crate) fn remove_listeners(listeners: &Listeners) {
    if let Listeners::Registered(id) = listeners {
        Registry::with(|reg| reg.unregister(id));
    }
}

/// Compare passed listeners for equality with a registered set via pointer equality checks
pub(crate) fn compare_listeners(registered_id: u64, rhs: &[Rc<dyn Listener>]) -> bool {
    // Empty sets are not stored
    if rhs.is_empty() {
        return false;
    }

    Registry::with(|reg| match reg.by_id.get(&registered_id) {
        Some(reg) => {
            if reg.len() != rhs.len() {
                return false;
            }

            rhs.iter()
                .any(|l| match reg.get(&EventDescriptor::from(&**l)) {
                    Some(reg) =>
                    {
                        #[allow(clippy::vtable_address_comparisons)]
                        !Rc::ptr_eq(reg, l)
                    }
                    None => true,
                })
        }
        None => false,
    })
}

#[cfg(all(test, feature = "wasm_test", feature = "listener_tests"))]
mod tests {
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::{html, utils::document, App, Callback, Component, ComponentLink};
    use wasm_bindgen::JsCast;
    use web_sys::MouseEvent;

    #[derive(Copy, Clone)]
    enum Message {
        Click,
        StopClicking,
    }

    trait MakeCallback {
        fn make_callback<C, M>(l: &ComponentLink<C>, msg: M) -> Callback<MouseEvent>
        where
            C: Component<Message = M>,
            M: Copy + 'static;
    }

    struct Comp<MC>
    where
        MC: MakeCallback + 'static,
    {
        stop_clicking: bool,
        clicked: u32,
        link: ComponentLink<Self>,
    }

    impl<MC> Component for Comp<MC>
    where
        MC: MakeCallback + 'static,
    {
        type Message = Message;
        type Properties = ();

        fn create(_: Self::Properties, link: crate::ComponentLink<Self>) -> Self {
            Comp {
                link,
                stop_clicking: false,
                clicked: 0,
            }
        }

        fn update(&mut self, msg: Self::Message) -> bool {
            match msg {
                Message::Click => {
                    self.clicked += 1;
                }
                Message::StopClicking => {
                    self.stop_clicking = true;
                }
            };
            true
        }

        fn change(&mut self, _: Self::Properties) -> bool {
            false
        }

        fn view(&self) -> crate::Html {
            if self.stop_clicking {
                html! {
                    <a>{self.clicked}</a>
                }
            } else {
                html! {
                    <a onclick=MC::make_callback(&self.link, Message::Click)>
                        {self.clicked}
                    </a>
                }
            }
        }
    }

    #[test]
    fn synchronous() {
        super::Registry::with(|r| *r = Default::default());

        struct MC();

        impl MakeCallback for MC {
            fn make_callback<C, M>(l: &ComponentLink<C>, msg: M) -> Callback<MouseEvent>
            where
                C: Component<Message = M>,
                M: Copy + 'static,
            {
                l.callback(move |_| msg)
            }
        }

        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();
        let link = App::<Comp<MC>>::new().mount(root);

        let el = document()
            .query_selector("a")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        macro_rules! assert_count {
            ($c:expr) => {
                assert_eq!(el.text_content(), Some($c.to_string()));
            };
        }

        assert_count!(0);
        el.click();
        assert_count!(1);
        el.click();
        assert_count!(2);
        link.send_message(Message::StopClicking);
        el.click();
        assert_count!(2);
    }

    // TODO: PASSIVE tests
    // TODO: oninput tests
    // TODO: onchange tests
}
