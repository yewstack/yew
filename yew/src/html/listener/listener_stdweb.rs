// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
// Yew maintains a list of event listeners not supported by stdweb in the yew-macro crate (in the tag_attribute.rs file).
// Be sure to update that list when you change anything here.

// Reduces repetition for common cases
macro_rules! impl_short {
    ($($action:ident($type:ident))*) => {
        impl_action! {
            $(
                $action($type) -> $type
                    => |e| crate::html::listener::convert_reference::<$type>(e).0
            )*
        }
    };
}

// No converter
impl_short! {
    onabort(ResourceAbortEvent)
    onauxclick(AuxClickEvent)
    onblur(BlurEvent)
    onclick(ClickEvent)
    oncontextmenu(ContextMenuEvent)
    ondblclick(DoubleClickEvent)
    ondrag(DragEvent)
    ondragend(DragEndEvent)
    ondragenter(DragEnterEvent)
    ondragexit(DragExitEvent)
    ondragleave(DragLeaveEvent)
    ondragover(DragOverEvent)
    ondragstart(DragStartEvent)
    ondrop(DragDropEvent)
    onerror(ResourceErrorEvent)
    onfocus(FocusEvent)
    onkeydown(KeyDownEvent)
    onkeypress(KeyPressEvent)
    onkeyup(KeyUpEvent)
    onload(ResourceLoadEvent)
    onloadstart(LoadStartEvent)
    onmousedown(MouseDownEvent)
    onmouseenter(MouseEnterEvent)
    onmouseleave(MouseLeaveEvent)
    onmousemove(MouseMoveEvent)
    onmouseout(MouseOutEvent)
    onmouseover(MouseOverEvent)
    onmouseup(MouseUpEvent)
    onprogress(ProgressEvent)
    onresize(ResizeEvent)
    onscroll(ScrollEvent)
    onslotchange(SlotChangeEvent)
    onsubmit(SubmitEvent)
    onwheel(MouseWheelEvent)
    ongotpointercapture(GotPointerCaptureEvent)
    onloadend(LoadEndEvent)
    onlostpointercapture(LostPointerCaptureEvent)
    onpointercancel(PointerCancelEvent)
    onpointerdown(PointerDownEvent)
    onpointerenter(PointerEnterEvent)
    onpointerleave(PointerLeaveEvent)
    onpointerlockchange(PointerLockChangeEvent)
    onpointerlockerror(PointerLockErrorEvent)
    onpointermove(PointerMoveEvent)
    onpointerout(PointerOutEvent)
    onpointerover(PointerOverEvent)
    onpointerup(PointerUpEvent)
    onselectionchange(SelectionChangeEvent)
    ontouchcancel(TouchCancel)
    ontouchend(TouchEnd)
    ontouchmove(TouchMove)
    ontouchstart(TouchStart)
}

// With converter
impl_action! {
    onchange(ChangeEvent) -> ChangeData => onchange_handler
    oninput(InputEvent) -> InputData => oninput_handler
}
