// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events

// Reduces repetition for common cases
macro_rules! impl_short {
    ($($action:ident)*) => {
        impl_action! {
            $(
                $action(Event, false) -> web_sys::Event => |e| e
            )*
        }
    };
    ($($action:ident($type:ident))*) => {
        impl_action! {
            $(
                $action($type, false) -> web_sys::$type
                    => |e| wasm_bindgen::JsCast::dyn_into::<web_sys::$type>(e).unwrap()
            )*
        }
    };
    ($($action:ident($type:ident, $passive:literal))*) => {
        impl_action! {
            $(
                $action($type, $passive) -> web_sys::$type
                    => |e| wasm_bindgen::JsCast::dyn_into::<web_sys::$type>(e).unwrap()
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

// Best used with passive listeners, if you handle each and every event globally
impl_short! {
    onmousemove(MouseEvent, true)
    onscroll(Event, true)
    onpointermove(PointerEvent, true)
    ontouchmove(TouchEvent, true)
    ontouchstart(TouchEvent, true)
}

// More specialized cases
impl_action! {
    onchange(Event) -> ChangeData => onchange_handler
    oninput(InputEvent) -> InputData => oninput_handler
}
