use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, EventTarget as HtmlEventTarget};

use super::Apply;
use crate::dom_bundle::{test_log, BSubtree, EventDescriptor};
use crate::virtual_dom::{Listener, Listeners};

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
pub trait EventListening {
    fn listener_id(&self) -> Option<u32>;
    fn set_listener_id(&self, id: u32);
}

impl EventListening for Element {
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
    type Bundle = ListenerRegistration;
    type Element = Element;

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
                root.with_listener_registry(|reg| reg.patch(root, id, &pending));
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
    pub fn unregister(&self, root: &BSubtree) {
        if let Self::Registered(id) = self {
            root.with_listener_registry(|r| r.unregister(id));
        }
    }
}

/// Global multiplexing event handler registry
#[derive(Debug)]
pub struct Registry {
    /// Counter for assigning new IDs
    id_counter: u32,

    /// Contains all registered event listeners by listener ID
    by_id: HashMap<u32, HashMap<EventDescriptor, Vec<Rc<dyn Listener>>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            id_counter: u32::default(),
            by_id: HashMap::default(),
        }
    }

    /// Handle a single event, given the listening element and event descriptor.
    pub fn get_handler(
        registry: &RefCell<Registry>,
        listening: &dyn EventListening,
        desc: &EventDescriptor,
    ) -> Option<impl FnOnce(&Event)> {
        // The tricky part is that we want to drop the reference to the registry before
        // calling any actual listeners (since that might end up running lifecycle methods
        // and modify the registry). So we clone the current listeners and return a closure
        let listener_id = listening.listener_id()?;
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
            root.ensure_handled(&desc);
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
                root.ensure_handled(&desc);
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

        root.brand_element(el as &HtmlEventTarget);
        el.set_listener_id(id);

        id
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::{Event, EventInit, FocusEvent, HtmlElement, MouseEvent};
    wasm_bindgen_test_configure!(run_in_browser);

    use gloo::utils::document;
    use wasm_bindgen::JsCast;
    use yew::Callback;

    use crate::html::TargetCast;
    use crate::virtual_dom::VNode;
    use crate::{
        create_portal, html, scheduler, AppHandle, Component, Context, Html, NodeRef, Properties,
    };

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

    #[derive(Default, PartialEq, Properties)]
    struct MixinProps<M: Properties> {
        state_ref: NodeRef,
        wrapped: M,
    }

    trait Mixin: Properties + Sized {
        fn view<C>(ctx: &Context<C>, state: &State) -> Html
        where
            C: Component<Message = Message, Properties = MixinProps<Self>>;
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
        M: Mixin + Properties + 'static,
    {
        type Message = Message;
        type Properties = MixinProps<M>;

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

    #[track_caller]
    fn assert_count(el: &NodeRef, count: isize) {
        let text = el
            .get()
            .expect("State ref not bound in the test case?")
            .text_content();
        assert_eq!(text, Some(count.to_string()))
    }

    #[track_caller]
    fn click(el: &NodeRef) {
        el.get().unwrap().dyn_into::<HtmlElement>().unwrap().click();
        scheduler::start_now();
    }

    fn get_el_by_selector(selector: &str) -> web_sys::HtmlElement {
        document()
            .query_selector(selector)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
    }

    fn init<M>() -> (AppHandle<Comp<M>>, NodeRef)
    where
        M: Mixin + Properties + Default,
    {
        // Remove any existing elements
        let body = document().query_selector("#output").unwrap().unwrap();
        while let Some(child) = body.query_selector("div#testroot").unwrap() {
            body.remove_child(&child).unwrap();
        }

        let root = document().create_element("div").unwrap();
        root.set_id("testroot");
        body.append_child(&root).unwrap();
        let props = <Comp<M> as Component>::Properties::default();
        let el_ref = props.state_ref.clone();
        let app = crate::Renderer::<Comp<M>>::with_root_and_props(root, props).render();
        scheduler::start_now();

        (app, el_ref)
    }

    #[test]
    fn synchronous() {
        #[derive(Default, PartialEq, Properties)]
        struct Synchronous;

        impl Mixin for Synchronous {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let onclick = ctx.link().callback(|_| Message::Action);

                if state.stop_listening {
                    html! {
                        <a ref={&ctx.props().state_ref}>{state.action}</a>
                    }
                } else {
                    html! {
                        <a {onclick} ref={&ctx.props().state_ref}>
                            {state.action}
                        </a>
                    }
                }
            }
        }

        let (link, el) = init::<Synchronous>();

        assert_count(&el, 0);

        click(&el);
        assert_count(&el, 1);

        click(&el);
        assert_count(&el, 2);

        link.send_message(Message::StopListening);
        scheduler::start_now();

        click(&el);
        assert_count(&el, 2);
    }

    #[test]
    async fn non_bubbling_event() {
        #[derive(Default, PartialEq, Properties)]
        struct NonBubbling;

        impl Mixin for NonBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let link = ctx.link().clone();
                let onblur = Callback::from(move |_| {
                    link.send_message(Message::Action);
                    scheduler::start_now();
                });
                html! {
                    <div>
                        <a ref={&ctx.props().state_ref}>
                            <input id="input" {onblur} type="text" />
                            {state.action}
                        </a>
                    </div>
                }
            }
        }

        let (_, el) = init::<NonBubbling>();

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
        #[derive(Default, PartialEq, Properties)]
        struct Bubbling;

        impl Mixin for Bubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                if state.stop_listening {
                    html! {
                        <div>
                            <a ref={&ctx.props().state_ref}>
                                {state.action}
                            </a>
                        </div>
                    }
                } else {
                    let cb = ctx.link().callback(|_| Message::Action);
                    html! {
                        <div onclick={cb.clone()}>
                            <a onclick={cb} ref={&ctx.props().state_ref}>
                                {state.action}
                            </a>
                        </div>
                    }
                }
            }
        }

        let (link, el) = init::<Bubbling>();

        assert_count(&el, 0);
        click(&el);
        assert_count(&el, 2);
        click(&el);
        assert_count(&el, 4);

        link.send_message(Message::StopListening);
        scheduler::start_now();
        click(&el);
        assert_count(&el, 4);
    }

    #[test]
    fn cancel_bubbling() {
        #[derive(Default, PartialEq, Properties)]
        struct CancelBubbling;

        impl Mixin for CancelBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let onclick = ctx.link().callback(|_| Message::Action);
                let onclick2 = ctx.link().callback(|e: MouseEvent| {
                    e.stop_propagation();
                    Message::Action
                });

                html! {
                    <div onclick={onclick}>
                        <a onclick={onclick2} ref={&ctx.props().state_ref}>
                            {state.action}
                        </a>
                    </div>
                }
            }
        }

        let (_, el) = init::<CancelBubbling>();

        assert_count(&el, 0);
        click(&el);
        assert_count(&el, 1);
        click(&el);
        assert_count(&el, 2);
    }

    #[test]
    fn cancel_bubbling_nested() {
        // Here an event is being delivered to a DOM node which does
        // _not_ have a listener but which is contained within an
        // element that does and which cancels the bubble.
        #[derive(Default, PartialEq, Properties)]
        struct CancelBubbling;

        impl Mixin for CancelBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let onclick = ctx.link().callback(|_| Message::Action);
                let onclick2 = ctx.link().callback(|e: MouseEvent| {
                    e.stop_propagation();
                    Message::Action
                });
                html! {
                    <div onclick={onclick}>
                        <div onclick={onclick2}>
                            <a ref={&ctx.props().state_ref}>
                                {state.action}
                            </a>
                        </div>
                    </div>
                }
            }
        }

        let (_, el) = init::<CancelBubbling>();

        assert_count(&el, 0);
        click(&el);
        assert_count(&el, 1);
        click(&el);
        assert_count(&el, 2);
    }

    #[test]
    fn non_bubbling() {
        #[derive(Default, PartialEq, Properties)]
        struct NonBubbling;

        impl Mixin for NonBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let onfocus = ctx.link().callback(|_| Message::Action);
                let onfocus_inner = ctx.link().callback(|e: FocusEvent| {
                    assert!(!e.bubbles(), "event should be non-bubbling");
                    Message::Action
                });

                html! {
                    <div {onfocus}>
                        <button onfocus={onfocus_inner} ref={&ctx.props().state_ref}>
                            {state.action}
                        </button>
                    </div>
                }
            }
        }
        // Should only trigger the inner listener, not also the outer one
        let (_, el) = init::<NonBubbling>();

        assert_count(&el, 0);
        el.get()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .focus()
            .unwrap();
        scheduler::start_now();
        assert_count(&el, 1);
    }

    /// Here an event is being delivered to a DOM node which is contained
    /// in a portal. It should bubble through the portal and reach the containing
    /// element.
    #[test]
    fn portal_bubbling() {
        #[derive(PartialEq, Properties)]
        struct PortalBubbling {
            host: web_sys::Element,
        }
        impl Default for PortalBubbling {
            fn default() -> Self {
                let host = document().create_element("div").unwrap();
                PortalBubbling { host }
            }
        }

        impl Mixin for PortalBubbling {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let portal_target = ctx.props().wrapped.host.clone();
                let onclick = ctx.link().callback(|_| Message::Action);
                html! {
                    <>
                        <div onclick={onclick}>
                            {create_portal(html! {
                                <a ref={&ctx.props().state_ref}>
                                    {state.action}
                                </a>
                            }, portal_target.clone())}
                        </div>
                        {VNode::VRef(portal_target.into())}
                    </>
                }
            }
        }

        let (_, el) = init::<PortalBubbling>();

        assert_count(&el, 0);
        click(&el);
        assert_count(&el, 1);
    }

    /// Here an event is being from inside a shadow root. It should only be caught exactly once on
    /// each handler
    #[test]
    fn open_shadow_dom_bubbling() {
        use web_sys::{ShadowRootInit, ShadowRootMode};
        #[derive(PartialEq, Properties)]
        struct OpenShadowDom {
            host: web_sys::Element,
            inner_root: web_sys::Element,
        }
        impl Default for OpenShadowDom {
            fn default() -> Self {
                let host = document().create_element("div").unwrap();
                let inner_root = document().create_element("div").unwrap();
                let shadow = host
                    .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                    .unwrap();
                shadow.append_child(&inner_root).unwrap();
                OpenShadowDom { host, inner_root }
            }
        }
        impl Mixin for OpenShadowDom {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let onclick = ctx.link().callback(|_| Message::Action);
                let mixin = &ctx.props().wrapped;
                html! {
                    <div onclick={onclick.clone()}>
                        <div {onclick}>
                            {create_portal(html! {
                                <a ref={&ctx.props().state_ref}>
                                    {state.action}
                                </a>
                            }, mixin.inner_root.clone())}
                        </div>
                        {VNode::VRef(mixin.host.clone().into())}
                    </div>
                }
            }
        }
        let (_, el) = init::<OpenShadowDom>();

        assert_count(&el, 0);
        click(&el);
        assert_count(&el, 2); // Once caught per handler
    }

    fn test_input_listener<E>(make_event: impl Fn() -> E)
    where
        E: Into<Event> + std::fmt::Debug,
    {
        #[derive(Default, PartialEq, Properties)]
        struct Input;

        impl Mixin for Input {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                if state.stop_listening {
                    html! {
                        <div>
                            <input type="text" />
                            <p ref={&ctx.props().state_ref}>{state.text.clone()}</p>
                        </div>
                    }
                } else {
                    let onchange = ctx.link().callback(|e: web_sys::Event| {
                        let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                        Message::SetText(el.value())
                    });
                    let oninput = ctx.link().callback(|e: web_sys::InputEvent| {
                        let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                        Message::SetText(el.value())
                    });

                    html! {
                        <div>
                            <input type="text" {onchange} {oninput} />
                            <p ref={&ctx.props().state_ref}>{state.text.clone()}</p>
                        </div>
                    }
                }
            }
        }

        let (link, state_ref) = init::<Input>();
        let input_el = get_el_by_selector("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();

        assert_eq!(&state_ref.get().unwrap().text_content().unwrap(), "");
        for mut s in ["foo", "bar", "baz"].iter() {
            input_el.set_value(s);
            if s == &"baz" {
                link.send_message(Message::StopListening);
                scheduler::start_now();

                s = &"bar";
            }
            input_el.dispatch_event(&make_event().into()).unwrap();
            scheduler::start_now();
            assert_eq!(&state_ref.get().unwrap().text_content().unwrap(), s);
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

    #[test]
    fn reentrant_listener() {
        #[derive(PartialEq, Properties, Default)]
        struct Reetrant {
            secondary_target_ref: NodeRef,
        }
        impl Mixin for Reetrant {
            fn view<C>(ctx: &Context<C>, state: &State) -> Html
            where
                C: Component<Message = Message, Properties = MixinProps<Self>>,
            {
                let targetref = &ctx.props().wrapped.secondary_target_ref;
                let onclick = {
                    let targetref = targetref.clone();
                    ctx.link().callback(move |_| {
                        // Note: `click` (and dispatchEvent for that matter) swallows errors thrown
                        // from listeners and reports them as uncaught to the console. Hence, we
                        // assert that we got to the second event listener instead, by dispatching a
                        // second Message::Action
                        click(&targetref);
                        Message::Action
                    })
                };
                let onclick2 = ctx.link().callback(move |_| Message::Action);
                html! {
                    <div>
                        <button {onclick} ref={&ctx.props().state_ref}>{state.action}</button>
                        <a onclick={onclick2} ref={targetref}></a>
                    </div>
                }
            }
        }
        let (_, el) = init::<Reetrant>();

        assert_count(&el, 0);
        click(&el);
        assert_count(&el, 2);
    }
}
