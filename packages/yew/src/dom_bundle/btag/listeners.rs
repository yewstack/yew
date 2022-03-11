use super::Apply;
use crate::dom_bundle::{test_log, BSubtree};
use crate::virtual_dom::{Listener, ListenerKind, Listeners};
use ::wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use gloo::events::{EventListener, EventListenerOptions, EventListenerPhase};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use web_sys::{Element, Event, EventTarget as HtmlEventTarget};

#[wasm_bindgen]
extern "C" {
    // Duck-typing, not a real class on js-side. On rust-side, use impls of EventTarget below
    type EventTargetable;
    #[wasm_bindgen(method, getter = __yew_listener_id, structural)]
    fn listener_id(this: &EventTargetable) -> Option<u32>;
    #[wasm_bindgen(method, setter = __yew_listener_id, structural)]
    fn set_listener_id(this: &EventTargetable, id: u32);
}

/// DOM-Types that can have listeners registered on them.
/// Uses the duck-typed interface from above in impls.
pub trait EventTarget {
    fn listener_id(&self) -> Option<u32>;
    fn set_listener_id(&self, id: u32);
}

impl EventTarget for Element {
    fn listener_id(&self) -> Option<u32> {
        self.unchecked_ref::<EventTargetable>().listener_id()
    }

    fn set_listener_id(&self, id: u32) {
        self.unchecked_ref::<EventTargetable>().set_listener_id(id);
    }
}

/// An active set of listeners on an element
#[derive(Debug)]
pub(super) enum ListenerRegistration {
    /// No listeners registered.
    NoReg,
    /// Added to global registry by ID
    Registered(u32),
}

impl Apply for Listeners {
    type Element = Element;
    type Bundle = ListenerRegistration;

    fn apply(self, root: &BSubtree, el: &Self::Element) -> ListenerRegistration {
        match self {
            Self::Pending(pending) => ListenerRegistration::register(root, el, &pending),
            Self::None => ListenerRegistration::NoReg,
        }
    }

    fn apply_diff(self, root: &BSubtree, el: &Self::Element, bundle: &mut ListenerRegistration) {
        use ListenerRegistration::*;
        use Listeners::*;

        match (self, bundle) {
            (Pending(pending), Registered(ref id)) => {
                // Reuse the ID
                test_log!("reusing listeners for {}", id);
                root.with_listener_registry(|reg| reg.patch(root, id, &*pending));
            }
            (Pending(pending), bundle @ NoReg) => {
                *bundle = ListenerRegistration::register(root, el, &pending);
                test_log!(
                    "registering listeners for {}",
                    match bundle {
                        ListenerRegistration::Registered(id) => id,
                        _ => unreachable!(),
                    }
                );
            }
            (None, bundle @ Registered(_)) => {
                let id = match bundle {
                    ListenerRegistration::Registered(ref id) => id,
                    _ => unreachable!(),
                };
                test_log!("unregistering listeners for {}", id);
                root.with_listener_registry(|reg| reg.unregister(id));
                *bundle = NoReg;
            }
            (None, NoReg) => {
                test_log!("{}", &"unchanged empty listeners");
            }
        };
    }
}

impl ListenerRegistration {
    /// Register listeners and return their handle ID
    fn register(root: &BSubtree, el: &Element, pending: &[Option<Rc<dyn Listener>>]) -> Self {
        Self::Registered(root.with_listener_registry(|reg| {
            let id = reg.set_listener_id(root, el);
            reg.register(root, id, pending);
            id
        }))
    }

    /// Remove any registered event listeners from the global registry
    pub(super) fn unregister(&self, root: &BSubtree) {
        if let Self::Registered(id) = self {
            root.with_listener_registry(|r| r.unregister(id));
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct EventDescriptor {
    kind: ListenerKind,
    passive: bool,
}

impl From<&dyn Listener> for EventDescriptor {
    fn from(l: &dyn Listener) -> Self {
        Self {
            kind: l.kind(),
            passive: l.passive(),
        }
    }
}

/// Ensures event handler registration.
//
// Separate struct to DRY, while avoiding partial struct mutability.
#[derive(Debug)]
struct HostHandlers {
    /// The host element where events are registered
    host: HtmlEventTarget,

    /// Events with registered handlers that are possibly passive
    handling: HashSet<EventDescriptor>,

    /// Keep track of all listeners to drop them on registry drop.
    /// The registry is never dropped in production.
    #[cfg(test)]
    registered: Vec<(ListenerKind, EventListener)>,
}

impl HostHandlers {
    fn new(host: HtmlEventTarget) -> Self {
        Self {
            host,
            handling: HashSet::default(),
            #[cfg(test)]
            registered: Vec::default(),
        }
    }

    /// Ensure a descriptor has a global event handler assigned
    fn ensure_handled(&mut self, root: &BSubtree, desc: EventDescriptor) {
        if !self.handling.contains(&desc) {
            let cl = {
                let desc = desc.clone();
                let options = EventListenerOptions {
                    phase: EventListenerPhase::Capture,
                    passive: desc.passive,
                };
                EventListener::new_with_options(
                    &self.host,
                    desc.kind.type_name(),
                    options,
                    root.event_listener(desc),
                )
            };

            // Never drop the closure as this event handler is static
            #[cfg(not(test))]
            cl.forget();
            #[cfg(test)]
            self.registered.push((desc.kind.clone(), cl));

            self.handling.insert(desc);
        }
    }
}

/// Global multiplexing event handler registry
#[derive(Debug)]
pub struct Registry {
    /// Counter for assigning new IDs
    id_counter: u32,

    /// Registered global event handlers
    global: HostHandlers,

    /// Contains all registered event listeners by listener ID
    by_id: HashMap<u32, HashMap<EventDescriptor, Vec<Rc<dyn Listener>>>>,
}

impl Registry {
    pub fn new(host: HtmlEventTarget) -> Self {
        Self {
            id_counter: u32::default(),
            global: HostHandlers::new(host),
            by_id: HashMap::default(),
        }
    }

    // Handle a single event, given the listener and event descriptor.
    pub fn get_handler(
        registry: &RefCell<Registry>,
        listener: &dyn EventTarget,
        desc: &EventDescriptor,
    ) -> Option<impl FnOnce(&Event)> {
        // The tricky part is that we want to drop the reference to the registry before
        // calling any actual listeners (since that might end up running lifecycle methods
        // and modify the registry). So we clone the current listeners and return a closure
        let listener_id = listener.listener_id()?;
        let registry_ref = registry.borrow();
        let handlers = registry_ref.by_id.get(&listener_id)?;
        let listeners = handlers.get(desc)?.clone();
        drop(registry_ref); // unborrow the registry, before running any listeners
        Some(move |event: &Event| {
            for l in listeners {
                l.handle(event.clone());
            }
        })
    }

    /// Register all passed listeners under ID
    fn register(&mut self, root: &BSubtree, id: u32, listeners: &[Option<Rc<dyn Listener>>]) {
        let mut by_desc =
            HashMap::<EventDescriptor, Vec<Rc<dyn Listener>>>::with_capacity(listeners.len());
        for l in listeners.iter().filter_map(|l| l.as_ref()).cloned() {
            let desc = EventDescriptor::from(l.deref());
            self.global.ensure_handled(root, desc.clone());
            by_desc.entry(desc).or_default().push(l);
        }
        self.by_id.insert(id, by_desc);
    }

    /// Patch an already registered set of handlers
    fn patch(&mut self, root: &BSubtree, id: &u32, listeners: &[Option<Rc<dyn Listener>>]) {
        if let Some(by_desc) = self.by_id.get_mut(id) {
            // Keeping empty vectors is fine. Those don't do much and should happen rarely.
            for v in by_desc.values_mut() {
                v.clear()
            }

            for l in listeners.iter().filter_map(|l| l.as_ref()).cloned() {
                let desc = EventDescriptor::from(l.deref());
                self.global.ensure_handled(root, desc.clone());
                by_desc.entry(desc).or_default().push(l);
            }
        }
    }

    /// Unregister any existing listeners for ID
    fn unregister(&mut self, id: &u32) {
        self.by_id.remove(id);
    }

    /// Set unique listener ID onto element and return it
    fn set_listener_id(&mut self, root: &BSubtree, el: &Element) -> u32 {
        let id = self.id_counter;
        self.id_counter += 1;

        root.brand_element(el);
        el.set_listener_id(id);

        id
    }
}

#[cfg(all(test, feature = "wasm_test"))]
mod tests {
    use std::marker::PhantomData;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::{Event, EventInit, MouseEvent};
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::{html, html::TargetCast, scheduler, AppHandle, Component, Context, Html};
    use gloo_utils::document;
    use wasm_bindgen::JsCast;
    use yew::Callback;

    #[derive(Clone)]
    enum Message {
        Action,
        StopListening,
        SetText(String),
    }

    #[derive(Default)]
    struct State {
        stop_listening: bool,
        action: u32,
        text: String,
    }

    trait Mixin {
        fn view<C>(ctx: &Context<C>, state: &State) -> Html
        where
            C: Component<Message = Message>,
        {
            let link = ctx.link().clone();
            let onclick = Callback::from(move |_| {
                link.send_message(Message::Action);
                scheduler::start_now();
            });

            if state.stop_listening {
                html! {
                    <a>{state.action}</a>
                }
            } else {
                html! {
                    <a {onclick}>
                        {state.action}
                    </a>
                }
            }
        }
    }

    struct Comp<M>
    where
        M: Mixin + 'static,
    {
        state: State,
        pd: PhantomData<M>,
    }

    impl<M> Component for Comp<M>
    where
        M: Mixin + 'static,
    {
        type Message = Message;
        type Properties = ();

        fn create(_: &Context<Self>) -> Self {
            Comp {
                state: Default::default(),
                pd: PhantomData,
            }
        }

        fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Message::Action => {
                    self.state.action += 1;
                }
                Message::StopListening => {
                    self.state.stop_listening = true;
                }
                Message::SetText(s) => {
                    self.state.text = s;
                }
            };
            true
        }

        fn view(&self, ctx: &Context<Self>) -> crate::Html {
            M::view(ctx, &self.state)
        }
    }

    fn assert_count(el: &web_sys::HtmlElement, count: isize) {
        assert_eq!(el.text_content(), Some(count.to_string()))
    }

    fn get_el_by_tag(tag: &str) -> web_sys::HtmlElement {
        document()
            .query_selector(tag)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
    }

    fn init<M>(tag: &str) -> (AppHandle<Comp<M>>, web_sys::HtmlElement)
    where
        M: Mixin,
    {
        // Remove any existing elements
        if let Some(el) = document().query_selector(tag).unwrap() {
            el.parent_element().unwrap().remove();
        }

        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();
        let app = crate::start_app_in_element::<Comp<M>>(root);
        scheduler::start_now();

        (app, get_el_by_tag(tag))
    }

    #[test]
    fn synchronous() {
        struct Synchronous;

        impl Mixin for Synchronous {}

        let (link, el) = init::<Synchronous>("a");

        assert_count(&el, 0);

        el.click();
        assert_count(&el, 1);

        el.click();
        assert_count(&el, 2);

        link.send_message(Message::StopListening);
        scheduler::start_now();

        el.click();
        assert_count(&el, 2);
    }

    #[test]
    async fn non_bubbling_event() {
        struct NonBubbling;

        impl Mixin for NonBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                let link = ctx.link().clone();
                let onblur = Callback::from(move |_| {
                    link.send_message(Message::Action);
                    scheduler::start_now();
                });
                html! {
                    <div>
                        <a>
                            <input id="input" {onblur} type="text" />
                            {state.action}
                        </a>
                    </div>
                }
            }
        }

        let (_, el) = init::<NonBubbling>("a");

        assert_count(&el, 0);

        let input = document().get_element_by_id("input").unwrap();

        input
            .dispatch_event(
                &Event::new_with_event_init_dict("blur", &{
                    let mut dict = EventInit::new();
                    dict.bubbles(false);
                    dict
                })
                .unwrap(),
            )
            .unwrap();

        assert_count(&el, 1);
    }

    #[test]
    fn bubbling() {
        struct Bubbling;

        impl Mixin for Bubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                if state.stop_listening {
                    html! {
                        <div>
                            <a>
                                {state.action}
                            </a>
                        </div>
                    }
                } else {
                    let link = ctx.link().clone();
                    let cb = Callback::from(move |_| {
                        link.send_message(Message::Action);
                        scheduler::start_now();
                    });
                    html! {
                        <div onclick={cb.clone()}>
                            <a onclick={cb}>
                                {state.action}
                            </a>
                        </div>
                    }
                }
            }
        }

        let (link, el) = init::<Bubbling>("a");

        assert_count(&el, 0);

        el.click();
        assert_count(&el, 2);

        el.click();
        assert_count(&el, 4);

        link.send_message(Message::StopListening);
        scheduler::start_now();
        el.click();
        assert_count(&el, 4);
    }

    #[test]
    fn cancel_bubbling() {
        struct CancelBubbling;

        impl Mixin for CancelBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                let link = ctx.link().clone();
                let onclick = Callback::from(move |_| {
                    link.send_message(Message::Action);
                    scheduler::start_now();
                });

                let link = ctx.link().clone();
                let onclick2 = Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    link.send_message(Message::Action);
                    scheduler::start_now();
                });

                html! {
                    <div onclick={onclick}>
                        <a onclick={onclick2}>
                            {state.action}
                        </a>
                    </div>
                }
            }
        }

        let (_, el) = init::<CancelBubbling>("a");

        assert_count(&el, 0);

        el.click();
        assert_count(&el, 1);

        el.click();
        assert_count(&el, 2);
    }

    #[test]
    fn cancel_bubbling_nested() {
        // Here an event is being delivered to a DOM node which does
        // _not_ have a listener but which is contained within an
        // element that does and which cancels the bubble.
        struct CancelBubbling;

        impl Mixin for CancelBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                let link = ctx.link().clone();
                let onclick = Callback::from(move |_| {
                    link.send_message(Message::Action);
                    scheduler::start_now();
                });

                let link = ctx.link().clone();
                let onclick2 = Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    link.send_message(Message::Action);
                    scheduler::start_now();
                });
                html! {
                    <div onclick={onclick}>
                        <div onclick={onclick2}>
                            <a>
                                {state.action}
                            </a>
                        </div>
                    </div>
                }
            }
        }

        let (_, el) = init::<CancelBubbling>("a");

        assert_count(&el, 0);

        el.click();
        assert_count(&el, 1);

        el.click();
        assert_count(&el, 2);
    }

    fn test_input_listener<E>(make_event: impl Fn() -> E)
    where
        E: JsCast + std::fmt::Debug,
    {
        struct Input;

        impl Mixin for Input {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                if state.stop_listening {
                    html! {
                        <div>
                            <input type="text" />
                            <p>{state.text.clone()}</p>
                        </div>
                    }
                } else {
                    let link = ctx.link().clone();
                    let onchange = Callback::from(move |e: web_sys::Event| {
                        let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                        link.send_message(Message::SetText(el.value()));
                        scheduler::start_now();
                    });

                    let link = ctx.link().clone();
                    let oninput = Callback::from(move |e: web_sys::InputEvent| {
                        let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                        link.send_message(Message::SetText(el.value()));
                        scheduler::start_now();
                    });

                    html! {
                        <div>
                            <input type="text" {onchange} {oninput} />
                            <p>{state.text.clone()}</p>
                        </div>
                    }
                }
            }
        }

        let (link, input_el) = init::<Input>("input");
        let input_el = input_el.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        let p_el = get_el_by_tag("p");

        assert_eq!(&p_el.text_content().unwrap(), "");
        for mut s in ["foo", "bar", "baz"].iter() {
            input_el.set_value(s);
            if s == &"baz" {
                link.send_message(Message::StopListening);
                scheduler::start_now();

                s = &"bar";
            }
            input_el
                .dyn_ref::<web_sys::EventTarget>()
                .unwrap()
                .dispatch_event(&make_event().dyn_into().unwrap())
                .unwrap();
            assert_eq!(&p_el.text_content().unwrap(), s);
        }
    }

    #[test]
    fn oninput() {
        test_input_listener(|| {
            web_sys::InputEvent::new_with_event_init_dict(
                "input",
                web_sys::InputEventInit::new().bubbles(true),
            )
            .unwrap()
        })
    }

    #[test]
    fn onchange() {
        test_input_listener(|| {
            web_sys::Event::new_with_event_init_dict(
                "change",
                web_sys::EventInit::new().bubbles(true),
            )
            .unwrap()
        })
    }
}
