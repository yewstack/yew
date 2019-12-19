// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(name: "click", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    ondoubleclick(name: "dblclick", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onkeypress(name: "keypress", event: KeyboardEvent) -> web_sys::KeyboardEvent => |_, event| { event }
    onkeydown(name: "keydown", event: KeyboardEvent) -> web_sys::KeyboardEvent => |_, event| { event }
    onkeyup(name: "keyup", event: KeyboardEvent) -> web_sys::KeyboardEvent => |_, event| { event }
    onmousemove(name: "mousemove", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmousedown(name: "mousedown", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseup(name: "mouseup", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseover(name: "mouseover", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseout(name: "mouseout", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseenter(name: "mouseenter", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmouseleave(name: "mouseleave", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    onmousewheel(name: "wheel", event: WheelEvent) -> web_sys::WheelEvent => |_, event| { event }
    ongotpointercapture(name: "gotpointercapture", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onlostpointercapture(name: "lostpointercapture", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointercancel(name: "pointercancel", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerdown(name: "pointerdown", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerenter(name: "pointerenter", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerleave(name: "pointerleave", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointermove(name: "pointermove", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerout(name: "pointerout", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerover(name: "pointerover", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onpointerup(name: "pointerup", event: PointerEvent) -> web_sys::PointerEvent => |_, event| { event }
    onscroll(name: "scroll", event: UiEvent) -> web_sys::UiEvent => |_, event| { event }
    onblur(name: "blur", event: FocusEvent) -> web_sys::FocusEvent => |_, event| { event }
    onfocus(name: "focus", event: FocusEvent) -> web_sys::FocusEvent => |_, event| { event }
    onsubmit(name: "submit", event: Event) -> web_sys::Event => |_, event| { event }
    ondragstart(name: "dragstart", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondrag(name: "drag", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragend(name: "dragend", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragenter(name: "dragenter", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragleave(name: "dragleave", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragover(name: "dragover", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondragexit(name: "dragexit", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    ondrop(name: "drop", event: DragEvent) -> web_sys::DragEvent => |_, event| { event }
    oncontextmenu(name: "contextmenu", event: MouseEvent) -> web_sys::MouseEvent => |_, event| { event }
    oninput(name: "input", event: Event) -> InputData => |this: &Element, _| { oninput_handler(this) }
    onchange(name: "change", event: Event) -> ChangeData => |this: &Element, _| { onchange_handler(this) }
    touchcancel(name: "touchcancel", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    touchend(name: "touchend", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    touchenter(name: "touchenter", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    touchmove(name: "touchmove", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
    touchstart(name: "touchstart", event: TouchEvent) -> web_sys::TouchEvent => |_, event| { event }
}
