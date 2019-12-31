#[macro_use]
mod macros;
#[cfg(feature = "std_web")]
mod listener_stdweb;
#[cfg(feature = "web_sys")]
mod listener_web_sys;

#[cfg(feature = "std_web")]
pub use listener_stdweb::*;
#[cfg(feature = "web_sys")]
pub use listener_web_sys::*;
#[cfg(feature = "std_web")]
use stdweb::{
    js,
    unstable::TryInto,
    web::{
        html_element::{InputElement, SelectElement, TextAreaElement},
        Element, FileList, IElement, INode,
    },
};
#[cfg(feature = "web_sys")]
use ::{
    std::mem::ManuallyDrop,
    wasm_bindgen::{closure::Closure, JsCast},
    web_sys::{
        Element, Event, EventTarget, FileList, HtmlInputElement as InputElement,
        HtmlSelectElement as SelectElement, HtmlTextAreaElement as TextAreaElement,
    },
};

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

fn oninput_handler(this: &Element) -> InputData {
    // Normally only InputElement or TextAreaElement can have an oninput event listener. In
    // practice though any element with `contenteditable=true` may generate such events,
    // therefore here we fall back to just returning the text content of the node.
    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
    #[cfg(feature = "std_web")]
    let (v1, v2) = (
        this.clone()
            .try_into()
            .map(|input: InputElement| input.raw_value())
            .ok(),
        this.clone()
            .try_into()
            .map(|input: TextAreaElement| input.value())
            .ok(),
    );
    #[cfg(feature = "web_sys")]
    let (v1, v2) = (
        this.dyn_ref().map(|input: &InputElement| input.value()),
        this.dyn_ref().map(|input: &TextAreaElement| input.value()),
    );
    let v3 = this.text_content();
    let value = v1.or(v2).or(v3)
        .expect("only an InputElement or TextAreaElement or an element with contenteditable=true can have an oninput event listener");
    InputData { value }
}

fn onchange_handler(this: &Element) -> ChangeData {
    match this.node_name().as_ref() {
        "INPUT" => {
            #[cfg(feature = "std_web")]
            let input: InputElement = this.clone().try_into().unwrap();
            #[cfg(feature = "web_sys")]
            let input: &InputElement = this.dyn_ref().unwrap();
            let is_file = input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false);
            if is_file {
                #[cfg(feature = "std_web")]
                let files: FileList = js!( return @{input}.files; ).try_into().unwrap();
                #[cfg(feature = "web_sys")]
                let files: FileList = input.files().unwrap();
                ChangeData::Files(files)
            } else {
                #[cfg(feature = "std_web")]
                return ChangeData::Value(input.raw_value());
                #[cfg(feature = "web_sys")]
                ChangeData::Value(input.value())
            }
        }
        "TEXTAREA" => {
            #[cfg(feature = "std_web")]
            let tae: TextAreaElement = this.clone().try_into().unwrap();
            #[cfg(feature = "web_sys")]
            let tae: &TextAreaElement = this.dyn_ref().unwrap();
            ChangeData::Value(tae.value())
        }
        "SELECT" => {
            #[cfg(feature = "std_web")]
            let se: SelectElement = this.clone().try_into().unwrap();
            #[cfg(feature = "web_sys")]
            let se = this.dyn_ref::<SelectElement>().unwrap().clone();
            ChangeData::Select(se)
        }
        _ => {
            panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
        }
    }
}

/// Handler to an event listener, only use is to cancel the event.
// We can't use `gloo`s implementation because it cancels the event upon dropping the handler, but
// we want the event to only be cancelled when the user desires. The main issue here is that
// `wasm-bindgen` doesn't support moving a closure to WASM, so the closure has to be "forgotten"
// and not be dropped, therefore the use of `ManuallyDrop` here.
#[cfg(feature = "web_sys")]
#[derive(Debug)]
pub struct EventListenerHandle {
    pub(crate) target: EventTarget,
    pub(crate) r#type: &'static str,
    pub(crate) callback: ManuallyDrop<Closure<dyn Fn(Event)>>,
}

#[cfg(feature = "web_sys")]
impl EventListenerHandle {
    /// Build new event listener handle.
    pub fn new(
        target: EventTarget,
        r#type: &'static str,
        callback: Closure<dyn Fn(Event)>,
    ) -> Self {
        EventListenerHandle {
            target,
            r#type,
            callback: ManuallyDrop::new(callback),
        }
    }

    /// Cancel event.
    pub fn remove(self) {
        self.target
            .remove_event_listener_with_callback(
                &self.r#type,
                self.callback.as_ref().unchecked_ref(),
            )
            .expect("failed to remove event listener");
        let _ = ManuallyDrop::into_inner(self.callback);
    }
}
