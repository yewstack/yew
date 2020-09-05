// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
// Yew maintains a list of event listeners not supported by stdweb in the yew-macro crate (in the tag_attribute.rs file).
// Be sure to update that list when you change anything here.

use super::{ChangeData, InputData};
use stdweb::js;
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::web::event::InputEvent;
use stdweb::web::html_element::{InputElement, SelectElement, TextAreaElement};
use stdweb::web::{Element, EventListenerHandle, IElement, INode};

pub(crate) fn oninput_handler(this: &Element, event: InputEvent) -> InputData {
    // Normally only InputElement or TextAreaElement can have an oninput event listener. In
    // practice though any element with `contenteditable=true` may generate such events,
    // therefore here we fall back to just returning the text content of the node.
    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
    let value = this
        .clone()
        .try_into()
        .map(|input: InputElement| input.raw_value())
        .ok()
        .or_else(|| {
            this.clone()
                .try_into()
                .map(|input: TextAreaElement| input.value())
                .ok()
        })
        .or_else(|| this.text_content())
        .expect(concat!(
            "only an InputElement, TextAreaElement, or an element with contenteditable=true ",
            "can have an oninput event listener"
        ));
    InputData { value, event }
}

pub(crate) fn onchange_handler(this: &Element) -> ChangeData {
    match this.node_name().as_ref() {
        "INPUT" => {
            let input = InputElement::try_from(this.clone()).unwrap();
            if input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false)
            {
                ChangeData::Files(js!( return @{input}.files; ).try_into().unwrap())
            } else {
                ChangeData::Value(input.raw_value())
            }
        }
        "TEXTAREA" => ChangeData::Value(TextAreaElement::try_from(this.clone()).unwrap().value()),
        "SELECT" => ChangeData::Select(SelectElement::try_from(this.clone()).unwrap()),
        _ => {
            panic!(concat!(
                "only an InputElement, TextAreaElement or SelectElement ",
                "can have an onchange event listener"
            ));
        }
    }
}

/// Handler to an event listener, only use is to cancel the event.
#[derive(Debug)]
pub struct EventListener(pub(crate) Option<EventListenerHandle>);

impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(event) = self.0.take() {
            event.remove()
        }
    }
}

#[macro_use]
macro_rules! impl_action {
    ($($action:ident($type:ident))*) => {
        impl_action! {
            $(
                $action($type) -> $type => |_, event| { event }
            )*
        }
    };
    ($($action:ident($type:ident) -> $ret:ty => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        #[doc(hidden)]
        pub mod $action {
            use crate::callback::Callback;
            use crate::html::listener::EventListener;
            use crate::virtual_dom::Listener;
            use stdweb::web::event::$type;
            use stdweb::web::{Element, IEventTarget};

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

                fn attach(&self, element: &Element) -> EventListener {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    EventListener(Some(element.add_event_listener(
                        move |event: $type | callback.emit($convert(&this, event)))
                    ))
                }
            }
        }
    )*};
}

// No conversion
impl_action! {
    onabort(ResourceAbortEvent)
    onauxclick(AuxClickEvent)
    onblur(BlurEvent)
    // oncancel not supported
    // oncanplay not supported
    // oncanplaythrough not supported
    onclick(ClickEvent)
    // onclose not supported
    oncontextmenu(ContextMenuEvent)
    // oncuechange not supported
    ondblclick(DoubleClickEvent)
    ondrag(DragEvent)
    ondragend(DragEndEvent)
    ondragenter(DragEnterEvent)
    ondragexit(DragExitEvent)
    ondragleave(DragLeaveEvent)
    ondragover(DragOverEvent)
    ondragstart(DragStartEvent)
    ondrop(DragDropEvent)
    // ondurationchange not supported
    // onemptied not supported
    // onended not supported
    onerror(ResourceErrorEvent)
    onfocus(FocusEvent)
    // onformdata not supported
    // oninvalid not supported
    onkeydown(KeyDownEvent)
    onkeypress(KeyPressEvent)
    onkeyup(KeyUpEvent)
    onload(ResourceLoadEvent)
    // onloadeddata not supported
    // onloadedmetadata not supported
    onloadstart(LoadStartEvent)
    onmousedown(MouseDownEvent)
    onmouseenter(MouseEnterEvent)
    onmouseleave(MouseLeaveEvent)
    onmousemove(MouseMoveEvent)
    onmouseout(MouseOutEvent)
    onmouseover(MouseOverEvent)
    onmouseup(MouseUpEvent)
    // onpause not supported
    // onplay not supported
    // onplaying not supported
    onprogress(ProgressEvent)
    // onratechange not supported
    // onreset not supported
    onresize(ResizeEvent)
    onscroll(ScrollEvent)
    // onsecuritypolicyviolation not supported
    // onseeked not supported
    // onseeking not supported
    // onselect not supported
    onslotchange(SlotChangeEvent)
    // onstalled not supported
    onsubmit(SubmitEvent)
    // onsuspend not supported
    // ontimeupdate not supported
    // ontoggle not supported
    // onvolumechange not supported
    // onwaiting not supported
    onwheel(MouseWheelEvent)
    // oncopy not supported
    // oncut not supported
    // onpaste not supported
    // onanimationcancel not supported
    // onanimationend not supported
    // onanimationiteration not supported
    // onanimationstart not supported
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
    // onselectstart not supported
    // onshow not supported
    ontouchcancel(TouchCancel)
    ontouchend(TouchEnd)
    ontouchmove(TouchMove)
    ontouchstart(TouchStart)
    // ontransitioncancel not supported
    // ontransitionend not supported
    // ontransitionrun not supported
    // ontransitionstart not supported
}

// With payload conversion
impl_action! {
    onchange(ChangeEvent) -> crate::html::listener::ChangeData
        => |this: &Element, _| { crate::html::listener::onchange_handler(this) }
    oninput(InputEvent) -> crate::html::listener::InputData
        => |this: &Element, event| { crate::html::listener::oninput_handler(this, event) }
}
