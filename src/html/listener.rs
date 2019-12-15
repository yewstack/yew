use crate::callback::Callback;
#[cfg(feature = "web_sys")]
use crate::compat::EventListenerHandle;
use crate::virtual_dom::Listener;
#[cfg(feature = "stdweb")]
use stdweb::html_element::SelectElement;
#[cfg(feature = "stdweb")]
#[allow(unused_imports)]
use stdweb::{
    _js_impl, js,
    web::{EventListenerHandle, FileList, INode},
};
#[cfg(feature = "web_sys")]
use web_sys::{HtmlSelectElement as SelectElement, FileList};

macro_rules! impl_action {
    (|onaction: $action:ident, $event:ident : $type:ident| -> $ret:ty => $convert:expr) => {
        paste::expr! {
            impl_action!(|action: [<on $action>], name: [<on $action>], $event: $type| -> $ret => $convert);
        }
    };
    (|onaction: $action:ident, name: $name:ident, $event:ident : $type:ident| -> $ret:ty => $convert:expr) => {
        paste::expr! {
            impl_action!(|action: [<on $action>], name: $name, $event: $type| -> $ret => $convert);
        }
    };
    (|action: $action:ident, $event:ident : $type:ident| -> $ret:ty => $convert:expr) => {
        impl_action!(|action: $action, name: $action, $event: $type| -> $ret => $convert);
    };
    (|action: $action:ident, name: $name:ident, $event:ident : $type:ident| -> $ret:ty => $convert:expr) => {
        /// An abstract implementation of a listener.
        pub mod $action {
            #[cfg(feature = "stdweb")]
            use stdweb::web::{IEventTarget, Element, event::{IEvent, $type}};
            #[cfg(feature = "web_sys")]
            use ::{
                std::mem::ManuallyDrop,
                wasm_bindgen::{closure::Closure, JsCast},
                web_sys::{Element, EventTarget, $type},
            };
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

                #[cfg(feature = "web_sys")]
                fn attach(&self, element: &Element) -> EventListenerHandle<$type> {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    let listener = move |event: $type| {
                        event.stop_propagation();
                        callback.emit($convert(&this, event));
                    };

                    let target = EventTarget::from(element.clone());
                    let listener = Closure::wrap(Box::new(listener) as Box<dyn Fn($type)>);
                    target.add_event_listener_with_callback(stringify!($name), listener.as_ref().unchecked_ref());

                    return EventListenerHandle {
                        target,
                        r#type: stringify!($name),
                        callback: ManuallyDrop::new(listener),
                    }
                }

                #[cfg(feature = "stdweb")]
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
    };
}

#[cfg(feature = "stdweb")]
mod internal {
    use super::*;

    impl_action!(|action: touchcancel, event: TouchCancel| -> TouchCancel => |_, event| { event });
    impl_action!(|action: touchend, event: TouchEnd| -> TouchEnd => |_, event| { event });
    impl_action!(|action: touchenter, event: TouchEnter| -> TouchEnter => |_, event| { event });
    impl_action!(|action: touchmove, event: TouchMove| -> TouchMove => |_, event| { event });
    impl_action!(|action: touchstart, event: TouchStart| -> TouchStart => |_, event| { event });
}

#[cfg(feature = "web_sys")]
mod internal {
    use super::*;

    impl_action!(|action: touchcancel, event: TouchEvent| -> TouchEvent => |_, event| { event });
    impl_action!(|action: touchend, event: TouchEvent| -> TouchEvent => |_, event| { event });
    impl_action!(|action: touchenter, event: TouchEvent| -> TouchEvent => |_, event| { event });
    impl_action!(|action: touchmove, event: TouchEvent| -> TouchEvent => |_, event| { event });
    impl_action!(|action: touchstart, event: TouchEvent| -> TouchEvent => |_, event| { event });
}

pub use internal::*;

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action!(|onaction: click, event: ClickEvent| -> ClickEvent => |_, event| { event });
impl_action!(|onaction: doubleclick, name: dblclick, event: DoubleClickEvent| -> DoubleClickEvent => |_, event| { event });
impl_action!(|onaction: keypress, event: KeyPressEvent| -> KeyPressEvent => |_, event| { event });
impl_action!(|onaction: keydown, event: KeyDownEvent| -> KeyDownEvent => |_, event| { event });
impl_action!(|onaction: keyup, event: KeyUpEvent| -> KeyUpEvent => |_, event| { event });
impl_action!(|onaction: mousemove, event: MouseMoveEvent| -> MouseMoveEvent => |_, event| { event });
impl_action!(|onaction: mousedown, event: MouseDownEvent| -> MouseDownEvent => |_, event| { event });
impl_action!(|onaction: mouseup, event: MouseUpEvent| -> MouseUpEvent => |_, event| { event });
impl_action!(|onaction: mouseover, event: MouseOverEvent| -> MouseOverEvent => |_, event| { event });
impl_action!(|onaction: mouseout, event: MouseOutEvent| -> MouseOutEvent => |_, event| { event });
impl_action!(|onaction: mouseenter, event: MouseEnterEvent| -> MouseEnterEvent => |_, event| { event });
impl_action!(|onaction: mouseleave, event: MouseLeaveEvent| -> MouseLeaveEvent => |_, event| { event });
impl_action!(|onaction: mousewheel, event: MouseWheelEvent| -> MouseWheelEvent => |_, event| { event });
impl_action!(|onaction: gotpointercapture, event: GotPointerCaptureEvent| -> GotPointerCaptureEvent => |_, event| { event });
impl_action!(|onaction: lostpointercapture, event: LostPointerCaptureEvent| -> LostPointerCaptureEvent => |_, event| { event });
impl_action!(|onaction: pointercancel, event: PointerCancelEvent| -> PointerCancelEvent => |_, event| { event });
impl_action!(|onaction: pointerdown, event: PointerDownEvent| -> PointerDownEvent => |_, event| { event });
impl_action!(|onaction: pointerenter, event: PointerEnterEvent| -> PointerEnterEvent => |_, event| { event });
impl_action!(|onaction: pointerleave, event: PointerLeaveEvent| -> PointerLeaveEvent => |_, event| { event });
impl_action!(|onaction: pointermove, event: PointerMoveEvent| -> PointerMoveEvent => |_, event| { event });
impl_action!(|onaction: pointerout, event: PointerOutEvent| -> PointerOutEvent => |_, event| { event });
impl_action!(|onaction: pointerover, event: PointerOverEvent| -> PointerOverEvent => |_, event| { event });
impl_action!(|onaction: pointerup, event: PointerUpEvent| -> PointerUpEvent => |_, event| { event });
impl_action!(|onaction: scroll, event: ScrollEvent| -> ScrollEvent => |_, event| { event });
impl_action!(|onaction: blur, event: BlurEvent| -> BlurEvent => |_, event| { event });
impl_action!(|onaction: focus, event: FocusEvent| -> FocusEvent => |_, event| { event });
impl_action!(|onaction: submit, event: SubmitEvent| -> SubmitEvent => |_, event| { event });
impl_action!(|onaction: dragstart, event: DragStartEvent| -> DragStartEvent => |_, event| { event });
impl_action!(|onaction: drag, event: DragEvent| -> DragEvent => |_, event| { event });
impl_action!(|onaction: dragend, event: DragEndEvent| -> DragEndEvent => |_, event| { event });
impl_action!(|onaction: dragenter, event: DragEnterEvent| -> DragEnterEvent => |_, event| { event });
impl_action!(|onaction: dragleave, event: DragLeaveEvent| -> DragLeaveEvent => |_, event| { event });
impl_action!(|onaction: dragover, event: DragOverEvent| -> DragOverEvent => |_, event| { event });
impl_action!(|onaction: dragexit, event: DragExitEvent| -> DragExitEvent => |_, event| { event });
impl_action!(|onaction: drop, event: DragDropEvent| -> DragDropEvent => |_, event| { event });
impl_action!(|onaction: contextmenu, event: ContextMenuEvent| -> ContextMenuEvent => |_, event| { event });
impl_action!(|onaction: input, event: InputEvent| -> InputData => |this: &Element, _| {
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
});
impl_action!(|onaction: change, event: ChangeEvent| -> ChangeData => |this: &Element, _| {
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
});

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
