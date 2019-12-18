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
use web_sys::{FileList, HtmlSelectElement as SelectElement};

macro_rules! impl_action {
    (|onaction: $action:ident, stdweb_event: $stdweb_type:ident, web_sys_event: $web_sys_type:ident| -> $ret:ty => $convert:expr) => {
        paste::expr! {
            impl_action!(|action: [<on $action>], name: [<on $action>], stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $ret => $convert);
        }
    };
    (|onaction: $action:ident, stdweb_event: $stdweb_type:ident, web_sys_event: $web_sys_type:ident| => $convert:expr) => {
        paste::expr! {
            #[cfg(feature = "stdweb")]
            impl_action!(|action: [<on $action>], name: [<on $action>], stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $stdweb_type => $convert);
            #[cfg(feature = "web_sys")]
            impl_action!(|action: [<on $action>], name: [<on $action>], stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $web_sys_type => $convert);
        }
    };
    (|onaction: $action:ident, name: $name:ident, stdweb_event: $stdweb_type:ident, web_sys_event: $web_sys_type:ident| => $convert:expr) => {
        paste::expr! {
            #[cfg(feature = "stdweb")]
            impl_action!(|action: [<on $action>], name: $name, stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $stdweb_type => $convert);
            #[cfg(feature = "web_sys")]
            impl_action!(|action: [<on $action>], name: $name, stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $web_sys_type => $convert);
        }
    };
    (|action: $action:ident, stdweb_event: $stdweb_type:ident, web_sys_event: $web_sys_type:ident| => $convert:expr) => {
        #[cfg(feature = "stdweb")]
        impl_action!(|action: $action, name: $action, stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $stdweb_type => $convert);
        #[cfg(feature = "web_sys")]
        impl_action!(|action: $action, name: $action, stdweb_event: $stdweb_type, web_sys_event: $web_sys_type| -> $web_sys_type => $convert);
    };
    (|action: $action:ident, name: $name:ident, stdweb_event: $stdweb_type:ident, web_sys_event: $web_sys_type:ident| -> $ret:ty => $convert:expr) => {
        /// An abstract implementation of a listener.
        pub mod $action {
            #[cfg(feature = "stdweb")]
            use stdweb::web::{IEventTarget, Element, event::{IEvent, $stdweb_type}};
            #[cfg(feature = "web_sys")]
            use ::{
                std::mem::ManuallyDrop,
                wasm_bindgen::{closure::Closure, JsCast},
                web_sys::{Element, EventTarget, $web_sys_type},
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
                fn attach(&self, element: Element) -> EventListenerHandle {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    let listener = move |event: web_sys::Event| {
                        event.stop_propagation();
                        let event = event.dyn_into::<$web_sys_type>().expect("wrong event type");
                        callback.emit($convert(&this, event));
                    };

                    let target = EventTarget::from(element.clone());
                    let listener = Closure::wrap(Box::new(listener) as Box<dyn Fn(web_sys::Event)>);
                    target
                        .add_event_listener_with_callback(
                            stringify!($name),
                            listener.as_ref().unchecked_ref(),
                        )
                        .expect("failed to add event listener");

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
                    let listener = move |event: $stdweb_type| {
                        event.stop_propagation();
                        callback.emit($convert(&this, event));
                    };
                    element.add_event_listener(listener)
                }
            }
        }
    };
}

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action!(|onaction: click, stdweb_event: ClickEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: doubleclick, name: dblclick, stdweb_event: DoubleClickEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: keypress, stdweb_event: KeyPressEvent, web_sys_event: KeyboardEvent| => |_, event| { event });
impl_action!(|onaction: keydown, stdweb_event: KeyDownEvent, web_sys_event: KeyboardEvent| => |_, event| { event });
impl_action!(|onaction: keyup, stdweb_event: KeyUpEvent, web_sys_event: KeyboardEvent| => |_, event| { event });
impl_action!(|onaction: mousemove, stdweb_event: MouseMoveEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: mousedown, stdweb_event: MouseDownEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: mouseup, stdweb_event: MouseUpEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: mouseover, stdweb_event: MouseOverEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: mouseout, stdweb_event: MouseOutEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: mouseenter, stdweb_event: MouseEnterEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: mouseleave, stdweb_event: MouseLeaveEvent, web_sys_event: MouseEvent| => |_, event| { event });
#[cfg(feature = "stdweb")]
impl_action!(|onaction: mousewheel, stdweb_event: MouseWheelEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: gotpointercapture, stdweb_event: GotPointerCaptureEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: lostpointercapture, stdweb_event: LostPointerCaptureEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointercancel, stdweb_event: PointerCancelEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointerdown, stdweb_event: PointerDownEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointerenter, stdweb_event: PointerEnterEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointerleave, stdweb_event: PointerLeaveEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointermove, stdweb_event: PointerMoveEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointerout, stdweb_event: PointerOutEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointerover, stdweb_event: PointerOverEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: pointerup, stdweb_event: PointerUpEvent, web_sys_event: PointerEvent| => |_, event| { event });
impl_action!(|onaction: scroll, stdweb_event: ScrollEvent, web_sys_event: UiEvent| => |_, event| { event });
impl_action!(|onaction: blur, stdweb_event: BlurEvent, web_sys_event: FocusEvent| => |_, event| { event });
impl_action!(|onaction: focus, stdweb_event: FocusEvent, web_sys_event: FocusEvent| => |_, event| { event });
impl_action!(|onaction: submit, stdweb_event: SubmitEvent, web_sys_event: Event| => |_, event| { event });
impl_action!(|onaction: dragstart, stdweb_event: DragStartEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: drag, stdweb_event: DragEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: dragend, stdweb_event: DragEndEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: dragenter, stdweb_event: DragEnterEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: dragleave, stdweb_event: DragLeaveEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: dragover, stdweb_event: DragOverEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: dragexit, name: dragend, stdweb_event: DragExitEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: drop, stdweb_event: DragDropEvent, web_sys_event: DragEvent| => |_, event| { event });
impl_action!(|onaction: contextmenu, stdweb_event: ContextMenuEvent, web_sys_event: MouseEvent| => |_, event| { event });
impl_action!(|onaction: input, stdweb_event: InputEvent, web_sys_event: Event| -> InputData => |this: &Element, _| {
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
impl_action!(|onaction: change, stdweb_event: ChangeEvent, web_sys_event: Event| -> ChangeData => |this: &Element, _| {
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
impl_action!(|action: touchcancel, stdweb_event: TouchCancel, web_sys_event: TouchEvent| => |_, event| { event });
impl_action!(|action: touchend, stdweb_event: TouchEnd, web_sys_event: TouchEvent| => |_, event| { event });
impl_action!(|action: touchenter, stdweb_event: TouchEnter, web_sys_event: TouchEvent| => |_, event| { event });
impl_action!(|action: touchmove, stdweb_event: TouchMove, web_sys_event: TouchEvent| => |_, event| { event });
impl_action!(|action: touchstart, stdweb_event: TouchStart, web_sys_event: TouchEvent| => |_, event| { event });

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
