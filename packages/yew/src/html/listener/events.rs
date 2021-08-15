// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onabort(name: "abort", event: Event) -> web_sys::Event => |_, event| { event }
    onauxclick(name: "auxclick", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onblur(name: "blur", event: FocusEvent) -> web_sys::FocusEvent => |_, event| { event }
    oncancel(name: "cancel", event: Event) -> web_sys::Event => |_, event| { event }
    oncanplay(name: "canplay", event: Event) -> web_sys::Event => |_, event| { event }
    oncanplaythrough(name: "canplaythrough", event: Event) -> web_sys::Event => |_, event| { event }
    onchange(name: "change", event: Event) -> web_sys::Event => |_, event| { event }
    onclick(name: "click", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onclose(name: "close", event: Event) -> web_sys::Event => |_, event| { event }
    oncontextmenu(name: "contextmenu", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    oncuechange(name: "cuechange", event: Event) -> web_sys::Event => |_, event| { event }
    ondblclick(name: "dblclick", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    ondrag(name: "drag", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragend(name: "dragend", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragenter(name: "dragenter", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragexit(name: "dragexit", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragleave(name: "dragleave", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragover(name: "dragover", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragstart(name: "dragstart", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondrop(name: "drop", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondurationchange(name: "durationchange", event: Event) -> web_sys::Event => |_, event| { event }
    onemptied(name: "emptied", event: Event) -> web_sys::Event => |_, event| { event }
    onended(name: "ended", event: Event) -> web_sys::Event => |_, event| { event }
    onerror(name: "error", event: Event) -> web_sys::Event => |_, event| { event }
    onfocus(name: "focus", event: FocusEvent) -> web_sys::FocusEvent => |_, event| { event }
    onfocusin(name: "focusin", event: FocusEvent) -> web_sys::FocusEvent => |_, event| { event }
    onfocusout(name: "focusout", event: FocusEvent) -> web_sys::FocusEvent => |_, event| { event }
    // web_sys doesn't have a struct for `FormDataEvent`
    onformdata(name: "formdata", event: Event) -> web_sys::Event => |_, event| { event }
    oninput(name: "input", event: InputEvent) -> web_sys::InputEvent => |_, event| { event }
    oninvalid(name: "invalid", event: Event) -> web_sys::Event => |_, event| { event }
    onkeydown(name: "keydown", event: KeyboardEvent) -> web_sys::KeyboardEvent => |_, event| { event }
    onkeypress(name: "keypress", event: KeyboardEvent) -> web_sys::KeyboardEvent => |_, event| { event }
    onkeyup(name: "keyup", event: KeyboardEvent) -> web_sys::KeyboardEvent => |_, event| { event }
    onload(name: "load", event: Event) -> web_sys::Event => |_, event| { event }
    onloadeddata(name: "loadeddata", event: Event) -> web_sys::Event => |_, event| { event }
    onloadedmetadata(name: "loadedmetadata", event: Event) -> web_sys::Event => |_, event| { event }
    onloadstart(name: "loadstart", event: ProgressEvent) -> web_sys::ProgressEvent => |_, event| { event }
    onmousedown(name: "mousedown", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseenter(name: "mouseenter", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseleave(name: "mouseleave", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmousemove(name: "mousemove", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseout(name: "mouseout", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseover(name: "mouseover", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseup(name: "mouseup", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onpause(name: "pause", event: Event) -> web_sys::Event => |_, event| { event }
    onplay(name: "play", event: Event) -> web_sys::Event => |_, event| { event }
    onplaying(name: "playing", event: Event) -> web_sys::Event => |_, event| { event }
    onprogress(name: "progress", event: ProgressEvent) -> web_sys::ProgressEvent => |_, event| { event }
    onratechange(name: "ratechange", event: Event) -> web_sys::Event => |_, event| { event }
    onreset(name: "reset", event: Event) -> web_sys::Event => |_, event| { event }
    onresize(name: "resize", event: Event) -> web_sys::Event => |_, event| { event }
    onscroll(name: "scroll", event: Event) -> web_sys::Event => |_, event| { event }
    onsecuritypolicyviolation(name: "securitypolicyviolation", event: Event) -> web_sys::Event => |_, event| { event }
    onseeked(name: "seeked", event: Event) -> web_sys::Event => |_, event| { event }
    onseeking(name: "seeking", event: Event) -> web_sys::Event => |_, event| { event }
    onselect(name: "select", event: Event) -> web_sys::Event => |_, event| { event }
    onslotchange(name: "slotchange", event: Event) -> web_sys::Event => |_, event| { event }
    onstalled(name: "stalled", event: Event) -> web_sys::Event => |_, event| { event }
    // web_sys doesn't have a struct for `SubmitEvent`
    onsubmit(name: "submit", event: Event) -> web_sys::Event => |_, event| { event }
    onsuspend(name: "suspend", event: Event) -> web_sys::Event => |_, event| { event }
    ontimeupdate(name: "timeupdate", event: Event) -> web_sys::Event => |_, event| { event }
    ontoggle(name: "toggle", event: Event) -> web_sys::Event => |_, event| { event }
    onvolumechange(name: "volumechange", event: Event) -> web_sys::Event => |_, event| { event }
    onwaiting(name: "waiting", event: Event) -> web_sys::Event => |_, event| { event }
    onwheel(name: "wheel", event: WheelEvent) -> web_sys::WheelEvent => |_, event| { event }

    oncopy(name: "copy", event: Event) -> web_sys::Event => |_, event| { event }
    oncut(name: "cut", event: Event) -> web_sys::Event => |_, event| { event }
    onpaste(name: "paste", event: Event) -> web_sys::Event => |_, event| { event }

    onanimationcancel(name: "animationcancel", event: AnimationEvent) -> web_sys::AnimationEvent => |_, event| { event }
    onanimationend(name: "animationend", event: AnimationEvent) -> web_sys::AnimationEvent => |_, event| { event }
    onanimationiteration(name: "animationiteration", event: AnimationEvent) -> web_sys::AnimationEvent => |_, event| { event }
    onanimationstart(name: "animationstart", event: AnimationEvent) -> web_sys::AnimationEvent => |_, event| { event }
    ongotpointercapture(name: "gotpointercapture", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onloadend(name: "loadend", event: ProgressEvent) -> web_sys::ProgressEvent => |_, event| { event }
    onlostpointercapture(name: "lostpointercapture", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointercancel(name: "pointercancel", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerdown(name: "pointerdown", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerenter(name: "pointerenter", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerleave(name: "pointerleave", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerlockchange(name: "pointerlockchange", event: Event) -> web_sys::Event => |_, event| { event }
    onpointerlockerror(name: "pointerlockerror", event: Event) -> web_sys::Event => |_, event| { event }
    onpointermove(name: "pointermove", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerout(name: "pointerout", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerover(name: "pointerover", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerup(name: "pointerup", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onselectionchange(name: "selectionchange", event: Event) -> web_sys::Event => |_, event| { event }
    onselectstart(name: "selectstart", event: Event) -> web_sys::Event => |_, event| { event }
    onshow(name: "show", event: Event) -> web_sys::Event => |_, event| { event }
    ontouchcancel(name: "touchcancel", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    ontouchend(name: "touchend", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    ontouchmove(name: "touchmove", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    ontouchstart(name: "touchstart", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    ontransitioncancel(name: "transitioncancel", event: TransitionEvent) -> web_sys::TransitionEvent => |_, event| { event }
    ontransitionend(name: "transitionend", event: TransitionEvent) -> web_sys::TransitionEvent => |_, event| { event }
    ontransitionrun(name: "transitionrun", event: TransitionEvent) -> web_sys::TransitionEvent => |_, event| { event }
    ontransitionstart(name: "transitionstart", event: TransitionEvent) -> web_sys::TransitionEvent => |_, event| { event }
}

use wasm_bindgen::JsCast;

/// A trait to define event meta data statically.
pub trait EventMeta {
    /// Type of the event
    ///
    /// Event type must implement [`JsCast`] so that Yew can cast it to the correct type.
    type Event: AsRef<web_sys::Event> + JsCast + 'static;

    /// Name of the event
    fn event_name() -> &'static str;
}

#[doc(hidden)]
pub mod oncustom {
    use super::EventMeta;
    use crate::callback::Callback;
    use crate::html::IntoPropValue;
    use crate::virtual_dom::Listener;
    use gloo::events::{EventListener, EventListenerOptions};
    use std::rc::Rc;
    use wasm_bindgen::JsCast;
    use web_sys::{Element, EventTarget};

    /// A wrapper for a callback which attaches event listeners to elements.
    #[derive(Clone, Debug)]
    pub struct Wrapper<T: EventMeta> {
        callback: Callback<T::Event>,
    }

    impl<T: EventMeta + 'static> Wrapper<T> {
        /// Create a wrapper for an event-typed callback
        pub fn new(callback: Callback<T::Event>) -> Self {
            Wrapper { callback }
        }

        #[doc(hidden)]
        #[inline]
        pub fn __macro_new(
            callback: impl IntoPropValue<Option<Callback<T::Event>>>,
        ) -> Option<Rc<dyn Listener>> {
            let callback = callback.into_prop_value()?;
            Some(Rc::new(Self::new(callback)))
        }
    }

    impl<T: EventMeta> Listener for Wrapper<T> {
        fn kind(&self) -> &'static str {
            T::event_name()
        }

        fn attach(&self, element: &Element) -> EventListener {
            let event_name = T::event_name();
            let callback = self.callback.clone();
            let listener = move |event: &web_sys::Event| {
                callback.emit(event.clone().unchecked_into());
            };

            let options = EventListenerOptions::enable_prevent_default();
            EventListener::new_with_options(
                &EventTarget::from(element.clone()),
                event_name,
                options,
                listener,
            )
        }
    }
}
