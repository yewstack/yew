// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onabort(event: ResourceAbortEvent) -> ResourceAbortEvent => |_, event| { event }
    // onanimationcancel not supported
    // onanimationend not supported
    // onanimationiteration not supported
    // onanimationstart not supported
    onauxclick(event: AuxClickEvent) -> AuxClickEvent => |_, event| { event }
    onblur(event: BlurEvent) -> BlurEvent => |_, event| { event }
    onerror(event: ResourceErrorEvent) -> ResourceErrorEvent => |_, event| { event }
    onfocus(event: FocusEvent) -> FocusEvent => |_, event| { event }
    // oncancel not supported
    // oncanplay not supported
    // oncanplaythrough not supported
    onchange(event: ChangeEvent) -> ChangeData => |this: &Element, _| { onchange_handler(this) }
    onclick(event: ClickEvent) -> ClickEvent => |_, event| { event }
    // onclose not supported
    oncontextmenu(event: ContextMenuEvent) -> ContextMenuEvent => |_, event| { event }
    // oncuechange not supported
    ondoubleclick(event: DoubleClickEvent) -> DoubleClickEvent => |_, event| { event }
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
    // onformdata not supported
    ongotpointercapture(event: GotPointerCaptureEvent) -> GotPointerCaptureEvent => |_, event| { event }
    oninput(event: InputEvent) -> InputData => |this: &Element, _| { oninput_handler(this) }
    // oninvalid not supported
    onkeydown(event: KeyDownEvent) -> KeyDownEvent => |_, event| { event }
    onkeypress(event: KeyPressEvent) -> KeyPressEvent => |_, event| { event }
    onkeyup(event: KeyUpEvent) -> KeyUpEvent => |_, event| { event }
    onload(event: ResourceLoadEvent) -> ResourceLoadEvent => |_, event| { event }
    // onloadeddata not supported
    // onloadedmetadata not supported
    onloadend(event: LoadEndEvent) -> LoadEndEvent => |_, event| { event }
    onloadstart(event: LoadStartEvent) -> LoadStartEvent => |_, event| { event }
    onlostpointercapture(event: LostPointerCaptureEvent) -> LostPointerCaptureEvent => |_, event| { event }
    onmousedown(event: MouseDownEvent) -> MouseDownEvent => |_, event| { event }
    onmouseenter(event: MouseEnterEvent) -> MouseEnterEvent => |_, event| { event }
    onmouseleave(event: MouseLeaveEvent) -> MouseLeaveEvent => |_, event| { event }
    onmousemove(event: MouseMoveEvent) -> MouseMoveEvent => |_, event| { event }
    onmouseout(event: MouseOutEvent) -> MouseOutEvent => |_, event| { event }
    onmouseover(event: MouseOverEvent) -> MouseOverEvent => |_, event| { event }
    onmouseup(event: MouseUpEvent) -> MouseUpEvent => |_, event| { event }
    onwheel(event: MouseWheelEvent) -> MouseWheelEvent => |_, event| { event }
    // onpause not supported
    // onplay not supported
    // onplaying not supported
    onpointerdown(event: PointerDownEvent) -> PointerDownEvent => |_, event| { event }
    onpointermove(event: PointerMoveEvent) -> PointerMoveEvent => |_, event| { event }
    onpointerup(event: PointerUpEvent) -> PointerUpEvent => |_, event| { event }
    onpointercancel(event: PointerCancelEvent) -> PointerCancelEvent => |_, event| { event }
    onpointerover(event: PointerOverEvent) -> PointerOverEvent => |_, event| { event }
    onpointerout(event: PointerOutEvent) -> PointerOutEvent => |_, event| { event }
    onpointerenter(event: PointerEnterEvent) -> PointerEnterEvent => |_, event| { event }
    onpointerleave(event: PointerLeaveEvent) -> PointerLeaveEvent => |_, event| { event }
    onpointerlockchange(event: PointerLockChangeEvent) -> PointerLockChangeEvent => |_, event| { event }
    onpointerlockerror(event: PointerLockErrorEvent) -> PointerLockErrorEvent => |_, event| { event }
    onprogress(event: ProgressEvent) -> ProgressEvent => |_, event| { event }
    // onratechange not supported
    // onreset not supported
    onresize(event: ResizeEvent) -> ResizeEvent => |_, event| { event }
    onscroll(event: ScrollEvent) -> ScrollEvent => |_, event| { event }
    // onseeked not supported
    // onseeking not supported
    // onselect not supported
    // onselectstart not supported
    onselectionchange(event: SelectionChangeEvent) -> SelectionChangeEvent => |_, event| { event }
    // onshow not supported
    // onstalled not supported
    onsubmit(event: SubmitEvent) -> SubmitEvent => |_, event| { event }
    // onsuspend not supported
    // ontimeupdate not supported
    // onvolumechange not supported
    ontouchcancel(event: TouchCancel) -> TouchCancel => |_, event| { event }
    ontouchend(event: TouchEnd) -> TouchEnd => |_, event| { event }
    ontouchmove(event: TouchMove) -> TouchMove => |_, event| { event }
    ontouchstart(event: TouchStart) -> TouchStart => |_, event| { event }
    // ontransitioncancel not supported
    // ontransitionend not supported
    // ontransitionrun not supported
    // ontransitionstart not supported
    // onwaiting not supported

    // oncopy not supported
    // oncut not supported
    // onpaste not supported
}
