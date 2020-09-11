// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events

use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

macro_rules! impl_action {
    ($($action:ident($type:ident) -> $ret:path => $convert:path)*) => {$(
        impl_action!($action($type, crate::callback::NO_FLAGS) -> $ret => $convert);
    )*};
    ($($action:ident($type:ident, $flags:path) -> $ret:path => $convert:path)*) => {$(
        /// An abstract implementation of a listener.
        #[doc(hidden)]
        pub mod $action {
            use crate::callback::{Callback, Flags};
            use crate::virtual_dom::Listener;
            use crate::virtual_dom::ListenerKind;

            /// A wrapper for a callback which attaches event listeners to elements.
            #[derive(Clone, Debug)]
            pub struct Wrapper {
                callback: Callback<Event>,
            }

            impl Wrapper {
                /// Create a wrapper for an event-typed callback
                pub fn new(callback: Callback<Event>) -> Self {
                    Wrapper { callback }
                }
            }

            /// And event type which keeps the returned type.
            pub type Event = $ret;

            impl Listener for Wrapper {
                fn kind(&self) -> ListenerKind {
                    ListenerKind::$action
                }

                fn handle(&self, event: web_sys::Event) {
                    self.callback.emit($convert(event));
                }

                fn flags(&self) -> Flags {
                    match &self.callback {
                        Callback::Callback{flags, ..} => (*flags).unwrap_or($flags),
                        _ => $flags,
                    }
                }
            }
        }
    )*};
}

pub(crate) fn cast_event<T>(e: web_sys::Event) -> T
where
    T: wasm_bindgen::JsCast,
{
    e.dyn_into().unwrap()
}

// Reduces repetition for common cases
macro_rules! impl_short {
    ($($action:ident)*) => {
        impl_action! {
            $(
                $action(Event) -> web_sys::Event => std::convert::identity
            )*
        }
    };
    ($($action:ident($type:ident))*) => {
        impl_action! {
            $(
                $action($type) -> web_sys::$type  => crate::html::listener::cast_event
            )*
        }
    };
}

// Unspecialized event type
impl_short! {
    onabort
    oncancel
    oncanplay
    oncanplaythrough
    onclose
    oncuechange
    ondurationchange
    onemptied
    onended
    onerror
    onformdata  // web_sys doesn't have a struct for `FormDataEvent`
    oninvalid
    onload
    onloadeddata
    onloadedmetadata
    onpause
    onplay
    onplaying
    onratechange
    onreset
    onresize
    onsecuritypolicyviolation
    onseeked
    onseeking
    onselect
    onslotchange
    onstalled
    onsuspend
    ontimeupdate
    ontoggle
    onvolumechange
    onwaiting

    oncopy
    oncut
    onpaste

    onpointerlockchange
    onpointerlockerror
    onselectionchange
    onselectstart
    onshow
}

// Specialized event type
impl_short! {
    onauxclick(MouseEvent)
    onblur(FocusEvent)
    onclick(MouseEvent)
    oncontextmenu(MouseEvent)
    ondblclick(MouseEvent)
    ondrag(DragEvent)
    ondragend(DragEvent)
    ondragenter(DragEvent)
    ondragexit(DragEvent)
    ondragleave(DragEvent)
    ondragover(DragEvent)
    ondragstart(DragEvent)
    ondrop(DragEvent)
    onfocus(FocusEvent)
    onkeydown(KeyboardEvent)
    onkeypress(KeyboardEvent)
    onkeyup(KeyboardEvent)
    onloadstart(ProgressEvent)
    onmousedown(MouseEvent)
    onmouseenter(MouseEvent)
    onmouseleave(MouseEvent)
    onmouseout(MouseEvent)
    onmouseover(MouseEvent)
    onmouseup(MouseEvent)
    onprogress(ProgressEvent)
    onsubmit(FocusEvent)
    onwheel(WheelEvent)
    onanimationcancel(AnimationEvent)
    onanimationend(AnimationEvent)
    onanimationiteration(AnimationEvent)
    onanimationstart(AnimationEvent)
    ongotpointercapture(PointerEvent)
    onloadend(ProgressEvent)
    onlostpointercapture(PointerEvent)
    onpointercancel(PointerEvent)
    onpointerdown(PointerEvent)
    onpointerenter(PointerEvent)
    onpointerleave(PointerEvent)
    onpointerout(PointerEvent)
    onpointerover(PointerEvent)
    onpointerup(PointerEvent)
    ontouchcancel(TouchEvent)
    ontouchend(TouchEvent)
    ontransitioncancel(TransitionEvent)
    ontransitionend(TransitionEvent)
    ontransitionrun(TransitionEvent)
    ontransitionstart(TransitionEvent)
}

macro_rules! impl_passive {
    ($($action:ident)*) => {
        impl_action! {
            $(
                $action(Event, crate::callback::PASSIVE) -> web_sys::Event
                    => std::convert::identity
            )*
        }
    };
    ($($action:ident($type:ident))*) => {
        impl_action! {
            $(
                $action($type, crate::callback::PASSIVE) -> web_sys::$type
                    => crate::html::listener::cast_event
            )*
        }
    };
}

// Best used with passive listeners, if you handle each and every event globally
impl_passive! {
    onscroll
}
impl_passive! {
    onmousemove(MouseEvent)
    onpointermove(PointerEvent)
    ontouchmove(TouchEvent)
    ontouchstart(TouchEvent)
}

// More specialized cases
impl_action! {
    onchange(Event) -> crate::html::listener::ChangeData => crate::html::listener::onchange_handler
    oninput(InputEvent) -> crate::html::listener::InputData
        => crate::html::listener::oninput_handler
}

/// Extract target as Element from event
fn extract_target(event: &web_sys::Event) -> Element {
    event
        .target()
        .expect("no target on event")
        .dyn_into()
        .unwrap()
}

pub(crate) fn oninput_handler(event: Event) -> super::InputData {
    let this = extract_target(&event);
    let event: web_sys::InputEvent = cast_event(event);

    // Normally only InputElement or TextAreaElement can have an oninput event listener. In
    // practice though any element with `contenteditable=true` may generate such events,
    // therefore here we fall back to just returning the text content of the node.
    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
    let value = this
        .dyn_ref()
        .map(|input: &HtmlInputElement| input.value())
        .or_else(|| {
            this.dyn_ref()
                .map(|input: &HtmlTextAreaElement| input.value())
        })
        .or_else(|| this.text_content())
        .expect(concat!(
            "only an InputElement or TextAreaElement or an element with contenteditable=true ",
            "can have an oninput event listener"
        ));
    super::InputData { value, event }
}

pub(crate) fn onchange_handler(event: Event) -> super::ChangeData {
    use super::ChangeData;

    let this = extract_target(&event);

    match this.node_name().as_ref() {
        "INPUT" => {
            let input = this.dyn_into::<HtmlInputElement>().unwrap();
            if input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false)
            {
                ChangeData::Files(input.files().unwrap())
            } else {
                ChangeData::Value(input.value())
            }
        }
        "TEXTAREA" => ChangeData::Value(this.dyn_into::<HtmlTextAreaElement>().unwrap().value()),
        "SELECT" => ChangeData::Select(this.dyn_into::<HtmlSelectElement>().unwrap()),
        _ => {
            panic!(concat!(
                "only an InputElement, TextAreaElement or SelectElement ",
                "can have an onchange event listener"
            ));
        }
    }
}
