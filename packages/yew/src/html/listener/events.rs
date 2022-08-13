// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events

macro_rules! impl_action {
    ($($action:ident($type:ident) -> $ret:path => $convert:path)*) => {$(
        impl_action!($action($type, false) -> $ret => $convert);
    )*};
    ($($action:ident($type:ident, $passive:literal) -> $ret:path => $convert:path)*) => {$(
        /// An abstract implementation of a listener.
        #[doc(hidden)]
        pub mod $action {
            use crate::callback::Callback;
            use crate::virtual_dom::{Listener, ListenerKind};
            use std::rc::Rc;

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

                #[doc(hidden)]
                #[inline]
                pub fn __macro_new(
                    callback: impl crate::html::IntoEventCallback<Event>,
                ) -> Option<Rc<dyn Listener>> {
                    let callback = callback.into_event_callback()?;
                    Some(Rc::new(Self::new(callback)))
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

                fn passive(&self) -> bool {
                    $passive
                }
            }
        }
    )*};
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

    onchange

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

    onblur(FocusEvent)
    onfocus(FocusEvent)
    onfocusin(FocusEvent)
    onfocusout(FocusEvent)

    onkeydown(KeyboardEvent)
    onkeypress(KeyboardEvent)
    onkeyup(KeyboardEvent)

    onloadstart(ProgressEvent)
    onprogress(ProgressEvent)
    onloadend(ProgressEvent)

    onmousedown(MouseEvent)
    onmouseenter(MouseEvent)
    onmouseleave(MouseEvent)
    onmousemove(MouseEvent)
    onmouseout(MouseEvent)
    onmouseover(MouseEvent)
    onmouseup(MouseEvent)
    onwheel(WheelEvent)

    oninput(InputEvent)

    onsubmit(SubmitEvent)

    onanimationcancel(AnimationEvent)
    onanimationend(AnimationEvent)
    onanimationiteration(AnimationEvent)
    onanimationstart(AnimationEvent)

    ongotpointercapture(PointerEvent)
    onlostpointercapture(PointerEvent)
    onpointercancel(PointerEvent)
    onpointerdown(PointerEvent)
    onpointerenter(PointerEvent)
    onpointerleave(PointerEvent)
    onpointermove(PointerEvent)
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
    ($($action:ident($type:ident))*) => {
        impl_action! {
            $(
                $action($type, true) -> web_sys::$type
                    => crate::html::listener::cast_event
            )*
        }
    };
}

// Best used with passive listeners for responsiveness
impl_passive! {
    onscroll(Event)

    ontouchmove(TouchEvent)
    ontouchstart(TouchEvent)
}
