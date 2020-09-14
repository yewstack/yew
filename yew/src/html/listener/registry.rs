use crate::{
    callback::{Callback, Flags, DEFER, HANDLE_BUBBLED, PASSIVE},
    services::render::{RenderService, RenderTask},
    virtual_dom::{Listener, ListenerKind, Listeners},
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event};

thread_local! {
    /// Global event listener registry
    static REGISTRY: RefCell<Registry> = Default::default();

    /// Key used to store listener id on element
    static LISTENER_ID_PROP: wasm_bindgen::JsValue = "__yew_listener_id".into();

    static BODY: web_sys::HtmlElement = crate::utils::document().body().unwrap();
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct EventDescriptor {
    kind: ListenerKind,
    flags: Flags,
}

impl From<&dyn Listener> for EventDescriptor {
    fn from(l: &dyn Listener) -> Self {
        Self {
            kind: l.kind(),
            flags: l.flags(),
        }
    }
}

/// Global multiplexing event handler registry
#[derive(Default, Debug)]
struct Registry {
    /// Counter for assigning new IDs
    id_counter: u32,

    /// Events with registered handlers that are possibly passive
    handling: HashSet<EventDescriptor>,

    /// Events that have been registered as bubbling at least once
    bubbling: HashSet<EventDescriptor>,

    /// Contains all registered event listeners by listener ID
    by_id: HashMap<u32, HashMap<EventDescriptor, Rc<dyn Listener>>>,

    /// Event handling deferred until the next animation frame
    deferred: VecDeque<(EventDescriptor, Event, web_sys::Element)>,

    render_task: Option<RenderTask>,

    /// Keep track of all listeners to drop them on registry drop.
    /// The registry is never dropped in production.
    #[cfg(test)]
    #[allow(clippy::type_complexity)]
    registered: Vec<(ListenerKind, Closure<dyn Fn(web_sys::Event)>)>,
}

impl Registry {
    /// Run f with access to global Registry
    fn with<R>(f: impl FnOnce(&mut Registry) -> R) -> R {
        REGISTRY.with(|r| f(&mut *r.borrow_mut()))
    }

    /// Register all passed listeners under ID
    fn register(&mut self, id: u32, listeners: Vec<Rc<dyn Listener>>) {
        let mut by_id = HashMap::with_capacity(listeners.len());
        for l in listeners.into_iter() {
            // Create global listener, if not yet created
            let key = EventDescriptor::from(&*l);

            if !self.handling.contains(&key) {
                BODY.with(|body| {
                    let cl =
                        Closure::wrap(Box::new(move |e: Event| Registry::handle(key, e))
                            as Box<dyn Fn(Event)>);
                    AsRef::<web_sys::EventTarget>::as_ref(body)
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            &key.kind.as_ref()[2..],
                            cl.as_ref().unchecked_ref(),
                            &{
                                let mut opts = web_sys::AddEventListenerOptions::new();
                                if key.flags.has_set(PASSIVE) {
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
                    self.registered.push((key.kind, cl));
                });

                self.handling.insert(key);
            }

            if key.flags.has_set(HANDLE_BUBBLED) {
                self.bubbling.insert(key);
            }

            by_id.insert(key, l);
        }
        self.by_id.insert(id, by_id);
    }

    /// Unregister any existing listeners for ID
    fn unregister(&mut self, id: &u32) {
        self.by_id.remove(id);
    }

    /// Set unique listener ID onto element and return it
    fn set_listener_id(&mut self, el: &Element) -> u32 {
        let id = self.id_counter;
        self.id_counter += 1;

        LISTENER_ID_PROP.with(|prop| {
            if !js_sys::Reflect::set(el, &prop, &js_sys::Number::from(id)).unwrap() {
                panic!("failed to set listener ID property");
            }
        });

        id
    }

    /// Ensure a render task is create
    fn ensure_render_task(&mut self) {
        if self.render_task.is_none() {
            self.render_task = RenderService::request_animation_frame(Callback::from(|_: f64| {
                Registry::handle_deferred()
            }))
            .into();
        }
    }

    /// Handle any deferred or debounced events
    fn handle_deferred() {
        for (desc, event, target) in Registry::with(|r| {
            r.render_task = None;
            std::mem::take(&mut r.deferred)
        }) {
            Self::run_handlers(desc, event, target);
        }
    }

    /// Handle a global event firing
    fn handle(desc: EventDescriptor, event: Event) {
        let target = match event
            .target()
            .map(|el| el.dyn_into::<web_sys::Element>().ok())
            .flatten()
        {
            Some(el) => el,
            None => return,
        };

        if desc.flags.has_set(DEFER) {
            Registry::with(move |r| {
                r.deferred.push_back((desc, event, target));
                r.ensure_render_task();
            });
            return;
        }

        Self::run_handlers(desc, event, target);
    }

    fn run_handlers(desc: EventDescriptor, event: Event, target: web_sys::Element) {
        let run_handler = |el: &web_sys::Element| {
            if let Some(l) = LISTENER_ID_PROP
                .with(|prop| js_sys::Reflect::get(el, &prop).ok())
                .map(|v| v.dyn_into().ok())
                .flatten()
                .map(|num: js_sys::Number| {
                    Registry::with(|r| {
                        r.by_id
                            .get(&(num.value_of() as u32))
                            .map(|s| s.get(&desc))
                            .flatten()
                            .cloned()
                    })
                })
                .flatten()
            {
                l.handle(event.clone());
            }
        };

        run_handler(&target);

        if Registry::with(|r| r.bubbling.contains(&desc)) {
            let mut el = target;
            loop {
                el = match el.parent_element() {
                    Some(el) => el,
                    None => break,
                };
                run_handler(&el);
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
                    .remove_event_listener_with_callback(
                        &kind.as_ref()[2..],
                        cl.as_ref().unchecked_ref(),
                    )
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
pub(crate) fn compare_listeners(registered_id: u32, rhs: &[Rc<dyn Listener>]) -> bool {
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

#[cfg(all(test, feature = "wasm_test", listener_tests))]
mod tests {
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::{
        callback::{Flags, DEFER, HANDLE_BUBBLED, NO_FLAGS, PASSIVE},
        html,
        html::listener::{ChangeData, InputData},
        utils::document,
        App, Component, ComponentLink, Html,
    };
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    #[derive(Clone)]
    enum Message {
        Click,
        StopListening,
        SetText(String),
        NOP,
    }

    #[derive(Default)]
    struct State {
        stop_listening: bool,
        clicked: u32,
        text: String,
    }

    trait Mixin {
        fn flags() -> Flags;

        fn view<C>(link: &ComponentLink<C>, state: &State) -> Html
        where
            C: Component<Message = Message>,
        {
            if state.stop_listening {
                html! {
                    <a>{state.clicked}</a>
                }
            } else {
                html! {
                    <a onclick=link.callback_with_flags(Self::flags(), |_| Message::Click)>
                        {state.clicked}
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
        link: ComponentLink<Self>,
    }

    impl<M> Component for Comp<M>
    where
        M: Mixin + 'static,
    {
        type Message = Message;
        type Properties = ();

        fn create(_: Self::Properties, link: crate::ComponentLink<Self>) -> Self {
            Comp {
                link,
                state: Default::default(),
            }
        }

        fn update(&mut self, msg: Self::Message) -> bool {
            match msg {
                Message::Click => {
                    self.state.clicked += 1;
                }
                Message::StopListening => {
                    self.state.stop_listening = true;
                }
                Message::SetText(s) => {
                    self.state.text = s;
                }
                Message::NOP => {
                    return false;
                }
            };
            true
        }

        fn change(&mut self, _: Self::Properties) -> bool {
            false
        }

        fn view(&self) -> crate::Html {
            M::view(&self.link, &self.state)
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

    fn init<M>(tag: &str) -> (ComponentLink<Comp<M>>, web_sys::HtmlElement)
    where
        M: Mixin,
    {
        // Remove any existing listeners and elements
        super::Registry::with(|r| *r = Default::default());
        if let Some(el) = document().query_selector(tag).unwrap() {
            el.parent_element().unwrap().remove();
        }

        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();
        let link = App::<Comp<M>>::new().mount(root);

        (link, get_el_by_tag(tag))
    }

    #[test]
    fn synchronous() {
        struct Synchronous();

        impl Mixin for Synchronous {
            fn flags() -> Flags {
                NO_FLAGS
            }
        }

        let (link, el) = init::<Synchronous>("a");

        assert_count(&el, 0);

        el.click();
        assert_count(&el, 1);

        el.click();
        assert_count(&el, 2);

        link.send_message(Message::StopListening);
        el.click();
        assert_count(&el, 2);
    }

    async fn await_animation_frame() {
        JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            crate::utils::window()
                .request_animation_frame(&resolve)
                .unwrap();
        }))
        .await
        .unwrap();
    }

    #[test]
    async fn passive() {
        struct Passive();

        impl Mixin for Passive {
            fn flags() -> Flags {
                PASSIVE
            }
        }

        assert_async::<Passive>().await;
    }

    async fn assert_async<M: Mixin + 'static>() {
        let (link, el) = init::<M>("a");

        macro_rules! assert_after_click {
            ($c:expr) => {
                el.click();
                await_animation_frame().await;
                assert_count(&el, $c);
            };
        }

        assert_count(&el, 0);

        assert_after_click!(1);

        assert_after_click!(2);

        link.send_message(Message::StopListening);
        assert_after_click!(2);
    }

    #[test]
    fn bubbling() {
        struct Bubbling();

        impl Mixin for Bubbling {
            fn flags() -> Flags {
                HANDLE_BUBBLED
            }

            fn view<C>(link: &ComponentLink<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                if state.stop_listening {
                    html! {
                        <div>
                            <a>
                                {state.clicked}
                            </a>
                        </div>
                    }
                } else {
                    let cb = link.callback_with_flags(Self::flags(), |_| Message::Click);
                    html! {
                        <div onclick=cb.clone()>
                            <a onclick=cb>
                                {state.clicked}
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
        el.click();
        assert_count(&el, 4);
    }

    #[test]
    async fn deferred() {
        struct Deferred();

        impl Mixin for Deferred {
            fn flags() -> Flags {
                DEFER
            }
        }

        assert_async::<Deferred>().await;
    }

    fn test_input_listener<E>(make_event: impl Fn() -> E)
    where
        E: JsCast + std::fmt::Debug,
    {
        struct Input();

        impl Mixin for Input {
            fn flags() -> Flags {
                NO_FLAGS
            }

            fn view<C>(link: &ComponentLink<C>, state: &State) -> Html
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
                    html! {
                        <div>
                            <input
                                type="text"
                                onchange=link
                                    .callback_with_flags(Self::flags(), |d: ChangeData| match d {
                                        ChangeData::Value(s) => Message::SetText(s),
                                        _ => Message::NOP,
                                    })
                                oninput=link
                                    .callback_with_flags(Self::flags(), |InputData { value, .. }| {
                                        Message::SetText(value)
                                    })
                                />
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
                &web_sys::InputEventInit::new().bubbles(true),
            )
            .unwrap()
        })
    }

    #[test]
    fn onchange() {
        test_input_listener(|| {
            web_sys::Event::new_with_event_init_dict(
                "change",
                &web_sys::EventInit::new().bubbles(true),
            )
            .unwrap()
        })
    }
}
