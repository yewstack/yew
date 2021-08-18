static_event_impl! {
    abort => web_sys::Event,
    auxclick => web_sys::MouseEvent,
    blur => web_sys::FocusEvent,
    cancel => web_sys::Event,
    canplay => web_sys::Event,
    canplaythrough => web_sys::Event,
    change => web_sys::Event,
    click => web_sys::MouseEvent,
    close => web_sys::Event,
    contextmenu => web_sys::MouseEvent,
    cuechange => web_sys::Event,
    dblclick => web_sys::MouseEvent,
    drag => web_sys::DragEvent,
    dragend => web_sys::DragEvent,
    dragenter => web_sys::DragEvent,
    dragexit => web_sys::DragEvent,
    dragleave => web_sys::DragEvent,
    dragover => web_sys::DragEvent,
    dragstart => web_sys::DragEvent,
    drop => web_sys::DragEvent,
    durationchange => web_sys::Event,
    emptied => web_sys::Event,
    ended => web_sys::Event,
    error => web_sys::Event,
    focus => web_sys::FocusEvent,
    focusin => web_sys::FocusEvent,
    focusout => web_sys::FocusEvent,
    /// [`web_sys`] doesn't have a supporting event type for `FormDataEvent` but does have a type
    /// for `FormData`.
    ///
    /// Getting `FormData` from [`Event`](web_sys::Event) can be done using [`Reflect::get`](js_sys::Reflect::get)
    /// from the [`js_sys`] crate.
    formdata => web_sys::Event,
    input => web_sys::InputEvent,
    invalid => web_sys::Event,
    keydown => web_sys::KeyboardEvent,
    keypress => web_sys::KeyboardEvent,
    keyup => web_sys::KeyboardEvent,
    load => web_sys::Event,
    loadeddata => web_sys::Event,
    loadedmetadata => web_sys::Event,
    loadstart => web_sys::ProgressEvent,
    mousedown => web_sys::MouseEvent,
    mouseenter => web_sys::MouseEvent,
    mouseleave => web_sys::MouseEvent,
    mousemove => web_sys::MouseEvent,
    mouseout => web_sys::MouseEvent,
    mouseover => web_sys::MouseEvent,
    mouseup => web_sys::MouseEvent,
    pause => web_sys::Event,
    play => web_sys::Event,
    playing => web_sys::Event,
    progress => web_sys::ProgressEvent,
    ratechange => web_sys::Event,
    reset => web_sys::Event,
    resize => web_sys::Event,
    scroll => web_sys::Event,
    securitypolicyviolation => web_sys::Event,
    seeked => web_sys::Event,
    seeking => web_sys::Event,
    select => web_sys::Event,
    slotchange => web_sys::Event,
    stalled => web_sys::Event,
    /// [`web_sys`] doesn't have a supporting event type for `SubmitEvent`.
    ///
    /// Getting `submitter` field from [`Event`](web_sys::Event) can be done using [`Reflect::get`](js_sys::Reflect::get)
    /// from the [`js_sys`] crate.
    submit => web_sys::Event,
    suspend => web_sys::Event,
    timeupdate => web_sys::Event,
    toggle => web_sys::Event,
    volumechange => web_sys::Event,
    waiting => web_sys::Event,
    wheel => web_sys::WheelEvent,
    copy => web_sys::Event,
    cut => web_sys::Event,
    paste => web_sys::Event,
    animationcancel => web_sys::AnimationEvent,
    animationend => web_sys::AnimationEvent,
    animationiteration => web_sys::AnimationEvent,
    animationstart => web_sys::AnimationEvent,
    gotpointercapture => web_sys::PointerEvent,
    loadend => web_sys::ProgressEvent,
    lostpointercapture => web_sys::PointerEvent,
    pointercancel => web_sys::PointerEvent,
    pointerdown => web_sys::PointerEvent,
    pointerenter => web_sys::PointerEvent,
    pointerleave => web_sys::PointerEvent,
    pointerlockchange => web_sys::Event,
    pointerlockerror => web_sys::Event,
    pointermove => web_sys::PointerEvent,
    pointerout => web_sys::PointerEvent,
    pointerover => web_sys::PointerEvent,
    pointerup => web_sys::PointerEvent,
    selectionchange => web_sys::Event,
    selectstart => web_sys::Event,
    show => web_sys::Event,
    touchcancel => web_sys::TouchEvent,
    touchend => web_sys::TouchEvent,
    touchmove => web_sys::TouchEvent,
    touchstart => web_sys::TouchEvent,
    transitioncancel => web_sys::TransitionEvent,
    transitionend => web_sys::TransitionEvent,
    transitionrun => web_sys::TransitionEvent,
    transitionstart => web_sys::TransitionEvent,
}

use wasm_bindgen::JsCast;

/// A trait to define event data statically.
pub trait StaticEvent {
    /// Type of the event
    ///
    /// Event type must implement [`JsCast`] so that Yew can cast it to the correct type.
    type Event: AsRef<web_sys::Event> + JsCast + 'static;

    /// Name of the event
    fn event_name() -> &'static str;
}

use crate::callback::Callback;
use crate::virtual_dom::Listener;
use gloo::events::{EventListener, EventListenerOptions};
use std::rc::Rc;
use web_sys::{Element, EventTarget};

use super::IntoEventCallback;

/// A wrapper for a callback which attaches event listeners to elements.
#[derive(Clone, Debug)]
pub struct Wrapper<T: StaticEvent> {
    callback: Callback<T::Event>,
}

impl<T: StaticEvent + 'static> Wrapper<T> {
    /// Create a wrapper for an event-typed callback
    pub fn new(callback: Callback<T::Event>) -> Self {
        Wrapper { callback }
    }

    #[doc(hidden)]
    #[inline]
    pub fn __macro_new(callback: impl IntoEventCallback<T::Event>) -> Option<Rc<dyn Listener>> {
        let callback = callback.into_event_callback()?;
        Some(Rc::new(Self::new(callback)))
    }
}

impl<T: StaticEvent> Listener for Wrapper<T> {
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
