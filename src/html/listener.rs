use crate::callback::Callback;
use crate::virtual_dom::Listener;
use stdweb::web::html_element::SelectElement;
#[allow(unused_imports)]
use stdweb::web::{EventListenerHandle, FileList, INode};
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

macro_rules! impl_action {
    ($($action:ident($event:ident : $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        pub mod $action {
            use stdweb::web::{IEventTarget, Element};
            use stdweb::web::event::{IEvent, $type};
            use super::*;

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
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&self, element: &Element) -> EventListenerHandle {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    let listener = move |event: $type| {
                        event.stop_propagation();
                        callback.emit($convert(&this, event));
                    };
                    element.add_event_listener(listener)
                }
            }
        }
    )*};
}

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(event: ClickEvent) -> ClickEvent => |_, event| { event }
    ondoubleclick(event: DoubleClickEvent) -> DoubleClickEvent => |_, event| { event }
    onkeypress(event: KeyPressEvent) -> KeyPressEvent => |_, event| { event }
    onkeydown(event: KeyDownEvent) -> KeyDownEvent => |_, event| { event }
    onkeyup(event: KeyUpEvent) -> KeyUpEvent => |_, event| { event }
    onmousemove(event: MouseMoveEvent) -> MouseMoveEvent => |_, event| { event }
    onmousedown(event: MouseDownEvent) -> MouseDownEvent => |_, event| { event }
    onmouseup(event: MouseUpEvent) -> MouseUpEvent => |_, event| { event }
    onmouseover(event: MouseOverEvent) -> MouseOverEvent => |_, event| { event }
    onmouseout(event: MouseOutEvent) -> MouseOutEvent => |_, event| { event }
    onmouseenter(event: MouseEnterEvent) -> MouseEnterEvent => |_, event| { event }
    onmouseleave(event: MouseLeaveEvent) -> MouseLeaveEvent => |_, event| { event }
    onmousewheel(event: MouseWheelEvent) -> MouseWheelEvent => |_, event| { event }
    ongotpointercapture(event: GotPointerCaptureEvent) -> GotPointerCaptureEvent => |_, event| { event }
    onlostpointercapture(event: LostPointerCaptureEvent) -> LostPointerCaptureEvent => |_, event| { event }
    onpointercancel(event: PointerCancelEvent) -> PointerCancelEvent => |_, event| { event }
    onpointerdown(event: PointerDownEvent) -> PointerDownEvent => |_, event| { event }
    onpointerenter(event: PointerEnterEvent) -> PointerEnterEvent => |_, event| { event }
    onpointerleave(event: PointerLeaveEvent) -> PointerLeaveEvent => |_, event| { event }
    onpointermove(event: PointerMoveEvent) -> PointerMoveEvent => |_, event| { event }
    onpointerout(event: PointerOutEvent) -> PointerOutEvent => |_, event| { event }
    onpointerover(event: PointerOverEvent) -> PointerOverEvent => |_, event| { event }
    onpointerup(event: PointerUpEvent) -> PointerUpEvent => |_, event| { event }
    onscroll(event: ScrollEvent) -> ScrollEvent => |_, event| { event }
    onblur(event: BlurEvent) -> BlurEvent => |_, event| { event }
    onfocus(event: FocusEvent) -> FocusEvent => |_, event| { event }
    onsubmit(event: SubmitEvent) -> SubmitEvent => |_, event| { event }
    ondragstart(event: DragStartEvent) -> DragStartEvent => |_, event| { event }
    ondrag(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragend(event: DragEndEvent) -> DragEndEvent => |_, event| { event }
    ondragenter(event: DragEnterEvent) -> DragEnterEvent => |_, event| { event }
    ondragleave(event: DragLeaveEvent) -> DragLeaveEvent => |_, event| { event }
    ondragover(event: DragOverEvent) -> DragOverEvent => |_, event| { event }
    ondragexit(event: DragExitEvent) -> DragExitEvent => |_, event| { event }
    ondrop(event: DragDropEvent) -> DragDropEvent => |_, event| { event }
    oncontextmenu(event: ContextMenuEvent) -> ContextMenuEvent => |_, event| { event }
    oninput(event: InputEvent) -> InputData => |this: &Element, _| {
        use stdweb::web::html_element::{InputElement, TextAreaElement};
        use stdweb::unstable::TryInto;
        // Normally only InputElement or TextAreaElement can have an oninput event listener. In
        // practice though any element with `contenteditable=true` may generate such events,
        // therefore here we fall back to just returning the text content of the node.
        // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
        let v1 = this.clone().try_into().map(|input: InputElement| input.raw_value()).ok();
        let v2 = this.clone().try_into().map(|input: TextAreaElement| input.value()).ok();
        let v3 = this.text_content();
        let value = v1.or(v2).or(v3)
            .expect("only an InputElement or TextAreaElement or an element with contenteditable=true can have an oninput event listener");
        InputData { value }
    }
    onchange(event: ChangeEvent) -> ChangeData => |this: &Element, _| {
        use stdweb::web::{FileList, IElement};
        use stdweb::web::html_element::{InputElement, TextAreaElement, SelectElement};
        use stdweb::unstable::TryInto;
        match this.node_name().as_ref() {
            "INPUT" => {
                let input: InputElement = this.clone().try_into().unwrap();
                let is_file = input.get_attribute("type").map(|value| {
                        value.eq_ignore_ascii_case("file")
                    })
                    .unwrap_or(false);
                if is_file {
                    let files: FileList = js!( return @{input}.files; )
                        .try_into()
                        .unwrap();
                    ChangeData::Files(files)
                } else {
                    ChangeData::Value(input.raw_value())
                }
            }
            "TEXTAREA" => {
                let tae: TextAreaElement = this.clone().try_into().unwrap();
                ChangeData::Value(tae.value())
            }
            "SELECT" => {
                let se: SelectElement = this.clone().try_into().unwrap();
                ChangeData::Select(se)
            }
            _ => {
                panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
            }
        }
    }
    ontouchcancel(event: TouchCancel) -> TouchCancel => |_, event| { event }
    ontouchend(event: TouchEnd) -> TouchEnd => |_, event| { event }
    ontouchenter(event: TouchEnter) -> TouchEnter => |_, event| { event }
    ontouchmove(event: TouchMove) -> TouchMove => |_, event| { event }
    ontouchstart(event: TouchStart) -> TouchStart => |_, event| { event }
}

/// A type representing data from `oninput` event.
#[derive(Debug)]
pub struct InputData {
    /// Inserted characters. Contains value from
    /// [InputEvent](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data).
    pub value: String,
}

// There is no '.../Web/API/ChangeEvent/data' (for onchange) similar to
// https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data (for oninput).
// ChangeData actually contains the value of the InputElement/TextAreaElement
// after `change` event occured or contains the SelectElement (see more at the
// variant ChangeData::Select)

/// A type representing change of value(s) of an element after committed by user
/// ([onchange event](https://developer.mozilla.org/en-US/docs/Web/Events/change)).
#[derive(Debug)]
pub enum ChangeData {
    /// Value of the element in cases of `<input>`, `<textarea>`
    Value(String),
    /// SelectElement in case of `<select>` element. You can use one of methods of SelectElement
    /// to collect your required data such as: `value`, `selected_index`, `selected_indices` or
    /// `selected_values`. You can also iterate throught `selected_options` yourself.
    Select(SelectElement),
    /// Files
    Files(FileList),
}
