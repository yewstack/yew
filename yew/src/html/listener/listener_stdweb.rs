// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
// Yew maintains a list of event listeners not supported by stdweb in the yew-macro crate (in the tag_attribute.rs file).
// Be sure to update that list when you change anything here.
impl_action! {
    onabort(event: ResourceAbortEvent) -> ResourceAbortEvent => |_, event| { event }
    onauxclick(event: AuxClickEvent) -> AuxClickEvent => |_, event| { event }
    onblur(event: BlurEvent) -> BlurEvent => |_, event| { event }
    // oncancel not supported
    // oncanplay not supported
    // oncanplaythrough not supported
    onchange(event: ChangeEvent) -> ChangeData => |this: &Element, _| { onchange_handler(this) }
    onclick(event: ClickEvent) -> ClickEvent => |_, event| { event }
    // onclose not supported
    oncontextmenu(event: ContextMenuEvent) -> ContextMenuEvent => |_, event| { event }
    // oncuechange not supported
    ondblclick(event: DoubleClickEvent) -> DoubleClickEvent => |_, event| { event }
    ondrag(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragend(event: DragEndEvent) -> DragEndEvent => |_, event| { event }
    ondragenter(event: DragEnterEvent) -> DragEnterEvent => |_, event| { event }
    ondragexit(event: DragExitEvent) -> DragExitEvent => |_, event| { event }
    ondragleave(event: DragLeaveEvent) -> DragLeaveEvent => |_, event| { event }
    ondragover(event: DragOverEvent) -> DragOverEvent => |_, event| { event }
    ondragstart(event: DragStartEvent) -> DragStartEvent => |_, event| { event }
    ondrop(event: DragDropEvent) -> DragDropEvent => |_, event| { event }
    // ondurationchange not supported
    // onemptied not supported
    // onended not supported
    onerror(event: ResourceErrorEvent) -> ResourceErrorEvent => |_, event| { event }
    onfocus(event: FocusEvent) -> FocusEvent => |_, event| { event }
    // onformdata not supported
    oninput(event: InputEvent) -> InputData => |this: &Element, event| { oninput_handler(this, event) }
    // oninvalid not supported
    onkeydown(event: KeyDownEvent) -> KeyDownEvent => |_, event| { event }
    onkeypress(event: KeyPressEvent) -> KeyPressEvent => |_, event| { event }
    onkeyup(event: KeyUpEvent) -> KeyUpEvent => |_, event| { event }
    onload(event: ResourceLoadEvent) -> ResourceLoadEvent => |_, event| { event }
    // onloadeddata not supported
    // onloadedmetadata not supported
    onloadstart(event: LoadStartEvent) -> LoadStartEvent => |_, event| { event }
    onmousedown(event: MouseDownEvent) -> MouseDownEvent => |_, event| { event }
    onmouseenter(event: MouseEnterEvent) -> MouseEnterEvent => |_, event| { event }
    onmouseleave(event: MouseLeaveEvent) -> MouseLeaveEvent => |_, event| { event }
    onmousemove(event: MouseMoveEvent) -> MouseMoveEvent => |_, event| { event }
    onmouseout(event: MouseOutEvent) -> MouseOutEvent => |_, event| { event }
    onmouseover(event: MouseOverEvent) -> MouseOverEvent => |_, event| { event }
    onmouseup(event: MouseUpEvent) -> MouseUpEvent => |_, event| { event }
    // onpause not supported
    // onplay not supported
    // onplaying not supported
    onprogress(event: ProgressEvent) -> ProgressEvent => |_, event| { event }
    // onratechange not supported
    // onreset not supported
    onresize(event: ResizeEvent) -> ResizeEvent => |_, event| { event }
    onscroll(event: ScrollEvent) -> ScrollEvent => |_, event| { event }
    // onsecuritypolicyviolation not supported
    // onseeked not supported
    // onseeking not supported
    // onselect not supported
    onslotchange(event: SlotChangeEvent) -> SlotChangeEvent => |_, event| { event }
    // onstalled not supported
    onsubmit(event: SubmitEvent) -> SubmitEvent => |_, event| { event }
    // onsuspend not supported
    // ontimeupdate not supported
    // ontoggle not supported
    // onvolumechange not supported
    // onwaiting not supported
    onwheel(event: MouseWheelEvent) -> MouseWheelEvent => |_, event| { event }

    // oncopy not supported
    // oncut not supported
    // onpaste not supported

    // onanimationcancel not supported
    // onanimationend not supported
    // onanimationiteration not supported
    // onanimationstart not supported
    ongotpointercapture(event: GotPointerCaptureEvent) -> GotPointerCaptureEvent => |_, event| { event }
    onloadend(event: LoadEndEvent) -> LoadEndEvent => |_, event| { event }
    onlostpointercapture(event: LostPointerCaptureEvent) -> LostPointerCaptureEvent => |_, event| { event }
    onpointercancel(event: PointerCancelEvent) -> PointerCancelEvent => |_, event| { event }
    onpointerdown(event: PointerDownEvent) -> PointerDownEvent => |_, event| { event }
    onpointerenter(event: PointerEnterEvent) -> PointerEnterEvent => |_, event| { event }
    onpointerleave(event: PointerLeaveEvent) -> PointerLeaveEvent => |_, event| { event }
    onpointerlockchange(event: PointerLockChangeEvent) -> PointerLockChangeEvent => |_, event| { event }
    onpointerlockerror(event: PointerLockErrorEvent) -> PointerLockErrorEvent => |_, event| { event }
    onpointermove(event: PointerMoveEvent) -> PointerMoveEvent => |_, event| { event }
    onpointerout(event: PointerOutEvent) -> PointerOutEvent => |_, event| { event }
    onpointerover(event: PointerOverEvent) -> PointerOverEvent => |_, event| { event }
    onpointerup(event: PointerUpEvent) -> PointerUpEvent => |_, event| { event }
    onselectionchange(event: SelectionChangeEvent) -> SelectionChangeEvent => |_, event| { event }
    // onselectstart not supported
    // onshow not supported
    ontouchcancel(event: TouchCancel) -> TouchCancel => |_, event| { event }
    ontouchend(event: TouchEnd) -> TouchEnd => |_, event| { event }
    ontouchmove(event: TouchMove) -> TouchMove => |_, event| { event }
    ontouchstart(event: TouchStart) -> TouchStart => |_, event| { event }
    // ontransitioncancel not supported
    // ontransitionend not supported
    // ontransitionrun not supported
    // ontransitionstart not supported
}
