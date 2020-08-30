use crate::virtual_dom::{Listener, Listeners};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use cfg_if::cfg_if;
use cfg_match::cfg_match;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Reference as Event;
        use stdweb::web::Element;
    } else if #[cfg(feature = "web_sys")] {
        use wasm_bindgen::JsCast;
        use web_sys::{Event, Element};
    }
}

thread_local! {
    static REGISTRY: Rc<RefCell<Registry>> = Default::default();

    /// Key used to store listener id on element
    #[cfg(feature = "web_sys")]
    static LISTENER_ID_PROP: wasm_bindgen::JsValue = "__yew_listener_id".into();
}

#[cfg(feature = "std_web")]
const LISTENER_ID_PROP: &str = "data-yew-listener-id";

#[derive(Clone, Copy, std::hash::Hash, Eq, PartialEq)]
struct EventDescriptor {
    kind: &'static str,
    passive: bool,
}

impl From<&dyn Listener> for EventDescriptor {
    fn from(l: &dyn Listener) -> Self {
        Self {
            kind: l.kind(),
            passive: cfg_match! {
                feature = "std_web" => false,
                feature = "web_sys" => l.passive(),
            },
        }
    }
}

/// Global multiplexing event handler registry
#[derive(Default)]
struct Registry {
    /// Counter for assigning new IDs
    id_counter: u64,

    /// Events with registered handlers that are possibly passive
    handling: HashSet<EventDescriptor>,

    /// Events that have been registered as bubbling at least once
    bubbling: HashSet<EventDescriptor>,

    /// Contains all registered event listeners by listener ID
    by_id: HashMap<u64, HashMap<EventDescriptor, Rc<dyn Listener>>>,
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
                let cl = Box::new(move |e: Event| {
                    Registry::with(move |reg| reg.handle(&key, e.clone()))
                }) as Box<dyn Fn(Event)>;

                #[cfg(feature = "web_sys")]
                {
                    thread_local! {
                        static BODY: web_sys::HtmlElement = web_sys::window()
                            .expect("no window global")
                            .document()
                            .expect("no document on window")
                            .body()
                            .expect("no body on document");
                    };

                    BODY.with(|b| l.attach(b, cl));
                }

                #[cfg(feature = "std_web")]
                l.attach(&stdweb::web::document().body().unwrap().into(), cl);

                self.handling.insert(key);
            }

            if l.handle_bubbled() {
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

        #[cfg(feature = "web_sys")]
        LISTENER_ID_PROP.with(|prop| {
            if !js_sys::Reflect::set(el, &prop, &js_sys::JsString::from(id.to_string())).unwrap() {
                panic!("failed to set listener ID property");
            }
        });

        // std_web does not support reflection so we have little choice but to pollute the
        // attributes
        #[cfg(feature = "std_web")]
        el.set_attribute(LISTENER_ID_PROP, &id.to_string()).unwrap();

        id
    }

    /// Handle a global event firing
    fn handle(&self, desc: &EventDescriptor, event: Event) {
        if let Some(l) = event
            .target()
            .map(|el| {
                cfg_match! {
                    feature = "std_web" => el.try_into::<Element>().ok(),
                    feature = "web_sys" => el.dyn_into::<web_sys::Element>().ok(),
                }
            })
            .flatten()
            .map(|el| {
                cfg_match! {
                    feature = "std_web" => el.get_attribute(LISTENER_ID_PROP),
                    feature = "web_sys" => LISTENER_ID_PROP
                        .with(|prop| js_sys::Reflect::get(&el, &prop).ok()),
                }
            })
            .flatten()
            .map(|v| {
                cfg_match! {
                    feature = "std_web" => v.parse().ok(),
                    feature = "web_sys" => v.dyn_into().ok()
                        .map(|v: js_sys::JsString| String::from(v).parse().ok()),
                }
            })
            .flatten()
            .flatten()
            .map(|id: u64| self.by_id.get(&id).map(|s| s.get(desc)).flatten())
            .flatten()
        {
            l.handle(event);

            if self.bubbling.contains(desc) {
                // TODO: if not passive, check if default was prevented after each call
                // (including the first call above)
                // TODO: travel up the parents for event bubbling
            }
        }
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
