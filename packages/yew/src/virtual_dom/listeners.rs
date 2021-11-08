use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event};

thread_local! {
    /// Global event listener registry
    static REGISTRY: RefCell<Registry> = Default::default();

    /// Key used to store listener id on element
    static LISTENER_ID_PROP: wasm_bindgen::JsValue = "__yew_listener_id".into();

    /// Cached reference to the document body
    static BODY: web_sys::HtmlElement = crate::utils::document().body().unwrap();
}

/// Bubble events during delegation
static mut BUBBLE_EVENTS: bool = true;

/// Set, if events should bubble up the DOM tree, calling any matching callbacks.
///
/// Bubbling is enabled by default. Disabling bubbling can lead to substantial improvements in event
/// handling performance.
///
/// Note that yew uses event delegation and implements internal even bubbling for performance
/// reasons. Calling `Event.stopPropagation()` or `Event.stopImmediatePropagation()` in the event
/// handler has no effect.
///
/// This function should be called before any component is mounted.
pub fn set_event_bubbling(bubble: bool) {
    unsafe {
        BUBBLE_EVENTS = bubble;
    }
}

/// The [Listener] trait is an universal implementation of an event listener
/// which is used to bind Rust-listener to JS-listener (DOM).
pub trait Listener {
    /// Returns the name of the event
    fn kind(&self) -> ListenerKind;

    /// Handles an event firing
    fn handle(&self, event: web_sys::Event);

    /// Makes the event listener passive. See
    /// [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener).
    fn passive(&self) -> bool;
}

impl std::fmt::Debug for dyn Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Listener {{ kind: {}, passive: {:?} }}",
            self.kind().as_ref(),
            self.passive(),
        )
    }
}

macro_rules! gen_listener_kinds {
    ($($kind:ident)*) => {
        /// Supported kinds of DOM event listeners
        // Using instead of strings to optimise registry collection performance by simplifying
        // hashmap hash calculation.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        pub enum ListenerKind {
            $( $kind, )*
        }

        impl AsRef<str> for ListenerKind {
            fn as_ref(&self) -> &str {
                match self {
                    $( Self::$kind => stringify!($kind), )*
                }
            }
        }
    };
}

gen_listener_kinds! {
    onabort
    onauxclick
    onblur
    oncancel
    oncanplay
    oncanplaythrough
    onchange
    onclick
    onclose
    oncontextmenu
    oncuechange
    ondblclick
    ondrag
    ondragend
    ondragenter
    ondragexit
    ondragleave
    ondragover
    ondragstart
    ondrop
    ondurationchange
    onemptied
    onended
    onerror
    onfocus
    onfocusin
    onfocusout
    onformdata
    oninput
    oninvalid
    onkeydown
    onkeypress
    onkeyup
    onload
    onloadeddata
    onloadedmetadata
    onloadstart
    onmousedown
    onmouseenter
    onmouseleave
    onmousemove
    onmouseout
    onmouseover
    onmouseup
    onpause
    onplay
    onplaying
    onprogress
    onratechange
    onreset
    onresize
    onscroll
    onsecuritypolicyviolation
    onseeked
    onseeking
    onselect
    onslotchange
    onstalled
    onsubmit
    onsuspend
    ontimeupdate
    ontoggle
    onvolumechange
    onwaiting
    onwheel
    oncopy
    oncut
    onpaste
    onanimationcancel
    onanimationend
    onanimationiteration
    onanimationstart
    ongotpointercapture
    onloadend
    onlostpointercapture
    onpointercancel
    onpointerdown
    onpointerenter
    onpointerleave
    onpointerlockchange
    onpointerlockerror
    onpointermove
    onpointerout
    onpointerover
    onpointerup
    onselectionchange
    onselectstart
    onshow
    ontouchcancel
    ontouchend
    ontouchmove
    ontouchstart
    ontransitioncancel
    ontransitionend
    ontransitionrun
    ontransitionstart
}

/// A list of event listeners
#[derive(Debug)]
pub enum Listeners {
    /// No listeners registered or pending.
    /// Distinct from `Pending` with an empty slice to avoid an allocation.
    None,

    /// Added to global registry by ID
    Registered(u32),

    /// Not yet added to the element or registry
    Pending(Box<[Option<Rc<dyn Listener>>]>),
}

impl Listeners {
    /// Register listeners and return their handle ID
    fn register(el: &Element, pending: &[Option<Rc<dyn Listener>>]) -> Self {
        Self::Registered(Registry::with(|reg| {
            let id = reg.set_listener_id(el);
            reg.register(id, pending);
            id
        }))
    }

    /// Remove any registered event listeners from the global registry
    pub(super) fn unregister(&self) {
        if let Self::Registered(id) = self {
            Registry::with(|r| r.unregister(id));
        }
    }
}

impl super::Apply for Listeners {
    type Element = Element;

    fn apply(&mut self, el: &Self::Element) {
        if let Self::Pending(pending) = self {
            *self = Self::register(el, pending);
        }
    }

    fn apply_diff(&mut self, el: &Self::Element, ancestor: Self) {
        use Listeners::*;

        match (std::mem::take(self), ancestor) {
            (Pending(pending), Registered(id)) => {
                // Reuse the ID
                Registry::with(|reg| reg.patch(&id, &*pending));
                *self = Registered(id);
            }
            (Pending(pending), None) => {
                *self = Self::register(el, &pending);
            }
            (None, Registered(id)) => {
                Registry::with(|reg| reg.unregister(&id));
            }
            _ => (),
        };
    }
}

impl PartialEq for Listeners {
    fn eq(&self, rhs: &Self) -> bool {
        use Listeners::*;

        match (self, rhs) {
            (None, None) => true,
            (Registered(lhs), Registered(rhs)) => lhs == rhs,
            (Registered(registered_id), Pending(pending))
            | (Pending(pending), Registered(registered_id)) => {
                use std::option::Option::None;

                Registry::with(|reg| match reg.by_id.get(registered_id) {
                    Some(reg) => {
                        if reg.len() != pending.len() {
                            return false;
                        }

                        pending.iter().filter_map(|l| l.as_ref()).all(|l| {
                            match reg.get(&EventDescriptor::from(l.deref())) {
                                Some(reg) => reg.iter().any(|reg| {
                                    #[allow(clippy::vtable_address_comparisons)]
                                    Rc::ptr_eq(reg, l)
                                }),
                                None => false,
                            }
                        })
                    }
                    None => false,
                })
            }
            (Pending(lhs), Pending(rhs)) => {
                if lhs.len() != rhs.len() {
                    false
                } else {
                    use std::option::Option::None;

                    lhs.iter()
                        .zip(rhs.iter())
                        .all(|(lhs, rhs)| match (lhs, rhs) {
                            (Some(lhs), Some(rhs)) =>
                            {
                                #[allow(clippy::vtable_address_comparisons)]
                                Rc::ptr_eq(lhs, rhs)
                            }
                            (None, None) => true,
                            _ => false,
                        })
                }
            }
            _ => false,
        }
    }
}

impl Clone for Listeners {
    fn clone(&self) -> Self {
        match self {
            Self::None | Self::Registered(_) => Self::None,
            Self::Pending(v) => Self::Pending(v.clone()),
        }
    }
}

impl Default for Listeners {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct EventDescriptor {
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

/// Ensures global event handler registration.
//
// Separate struct to DRY, while avoiding partial struct mutability.
#[derive(Default, Debug)]
struct GlobalHandlers {
    /// Events with registered handlers that are possibly passive
    handling: HashSet<EventDescriptor>,

    /// Keep track of all listeners to drop them on registry drop.
    /// The registry is never dropped in production.
    #[cfg(test)]
    #[allow(clippy::type_complexity)]
    registered: Vec<(ListenerKind, Closure<dyn Fn(web_sys::Event)>)>,
}

impl GlobalHandlers {
    /// Ensure a descriptor has a global event handler assigned
    fn ensure_handled(&mut self, desc: EventDescriptor) {
        if !self.handling.contains(&desc) {
            let cl = BODY.with(|body| {
                let cl = Closure::wrap(
                    Box::new(move |e: Event| Registry::handle(desc, e)) as Box<dyn Fn(Event)>
                );
                AsRef::<web_sys::EventTarget>::as_ref(body)
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        &desc.kind.as_ref()[2..],
                        cl.as_ref().unchecked_ref(),
                        &{
                            let mut opts = web_sys::AddEventListenerOptions::new();
                            opts.capture(true);
                            // We need to explicitly set passive to override any browser defaults
                            opts.passive(desc.passive);
                            opts
                        },
                    )
                    .map_err(|e| format!("could not register global listener: {:?}", e))
                    .unwrap();
                cl
            });

            // Never drop the closure as this event handler is static
            #[cfg(not(test))]
            cl.forget();
            #[cfg(test)]
            self.registered.push((desc.kind, cl));

            self.handling.insert(desc);
        }
    }
}

// Enable resetting between tests
#[cfg(test)]
impl Drop for GlobalHandlers {
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

/// Global multiplexing event handler registry
#[derive(Default, Debug)]
struct Registry {
    /// Counter for assigning new IDs
    id_counter: u32,

    /// Registered global event handlers
    global: GlobalHandlers,

    /// Contains all registered event listeners by listener ID
    by_id: HashMap<u32, HashMap<EventDescriptor, Vec<Rc<dyn Listener>>>>,
}

impl Registry {
    /// Run f with access to global Registry
    #[inline]
    fn with<R>(f: impl FnOnce(&mut Registry) -> R) -> R {
        REGISTRY.with(|r| f(&mut *r.borrow_mut()))
    }

    /// Register all passed listeners under ID
    fn register(&mut self, id: u32, listeners: &[Option<Rc<dyn Listener>>]) {
        let mut by_desc =
            HashMap::<EventDescriptor, Vec<Rc<dyn Listener>>>::with_capacity(listeners.len());
        for l in listeners.iter().filter_map(|l| l.as_ref()).cloned() {
            let desc = EventDescriptor::from(l.deref());
            self.global.ensure_handled(desc);
            by_desc.entry(desc).or_default().push(l);
        }
        self.by_id.insert(id, by_desc);
    }

    /// Patch an already registered set of handlers
    fn patch(&mut self, id: &u32, listeners: &[Option<Rc<dyn Listener>>]) {
        if let Some(by_desc) = self.by_id.get_mut(id) {
            // Keeping empty vectors is fine. Those don't do much and should happen rarely.
            for v in by_desc.values_mut() {
                v.clear()
            }

            for l in listeners.iter().filter_map(|l| l.as_ref()).cloned() {
                let desc = EventDescriptor::from(l.deref());
                self.global.ensure_handled(desc);
                by_desc.entry(desc).or_default().push(l);
            }
        }
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
            if !js_sys::Reflect::set(el, prop, &js_sys::Number::from(id)).unwrap() {
                panic!("failed to set listener ID property");
            }
        });

        id
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

        Self::run_handlers(desc, event, target);
    }

    fn run_handlers(desc: EventDescriptor, event: Event, target: web_sys::Element) {
        let run_handler = |el: &web_sys::Element| {
            if let Some(l) = LISTENER_ID_PROP
                .with(|prop| js_sys::Reflect::get(el, prop).ok())
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
                for l in l {
                    l.handle(event.clone());
                }
            }
        };

        run_handler(&target);

        if unsafe { BUBBLE_EVENTS } {
            let mut el = target;
            loop {
                el = match el.parent_element() {
                    Some(el) => el,
                    None => break,
                };
                // XXX: we have no way to detect, if the callback called `Event.stopPropagation()`
                // or `Event.stopImmediatePropagation()` without breaking the callback API.
                // It's arguably not worth the cost.
                run_handler(&el);
            }
        }
    }
}

#[cfg(all(test, feature = "wasm_test"))]
mod tests {
    use std::marker::PhantomData;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::{Event, EventInit};
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::{html, html::TargetCast, utils::document, AppHandle, Component, Context, Html};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

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
        fn passive() -> Option<bool> {
            None
        }

        fn view<C>(ctx: &Context<C>, state: &State) -> Html
        where
            C: Component<Message = Message>,
        {
            if state.stop_listening {
                html! {
                    <a>{state.action}</a>
                }
            } else {
                html! {
                    <a onclick={ctx.link().callback_with_passive(
                        Self::passive(),
                        |_| Message::Action,
                    )}>
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
        // Remove any existing listeners and elements
        super::Registry::with(|r| *r = Default::default());
        if let Some(el) = document().query_selector(tag).unwrap() {
            el.parent_element().unwrap().remove();
        }

        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();
        let app = crate::start_app_in_element::<Comp<M>>(root);

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
        struct Passive;

        impl Mixin for Passive {
            fn passive() -> Option<bool> {
                Some(true)
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
    async fn non_bubbling_event() {
        struct NonBubbling;

        impl Mixin for NonBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message>,
            {
                let onblur = ctx.link().callback(|_| Message::Action);
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
                    let cb = ctx.link().callback(|_| Message::Action);
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
        el.click();
        assert_count(&el, 4);
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
                    html! {
                        <div>
                            <input
                                type="text"
                                onchange={ctx.link().callback(|e: web_sys::Event| {
                                    let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Message::SetText(el.value())
                                })}
                                oninput={ctx.link().callback(|e: web_sys::InputEvent| {
                                    let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Message::SetText(el.value())
                                })}
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
