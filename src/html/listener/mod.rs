#[macro_use]
mod macros;

use cfg_if::cfg_if;
use cfg_match::cfg_match;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod listener_stdweb;

        use stdweb::js;
        use stdweb::unstable::{TryFrom, TryInto};
        use stdweb::web::html_element::{InputElement, SelectElement, TextAreaElement};
        use stdweb::web::{Element, EventListenerHandle, FileList, IElement, INode};

        pub use listener_stdweb::*;
    } else if #[cfg(feature = "web_sys")] {
        mod listener_web_sys;

        use wasm_bindgen::JsCast;
        use web_sys::{
            Element, FileList, HtmlInputElement as InputElement, HtmlSelectElement as SelectElement,
            HtmlTextAreaElement as TextAreaElement,
        };

        pub use listener_web_sys::*;
    }
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

fn oninput_handler(this: &Element) -> InputData {
    // Normally only InputElement or TextAreaElement can have an oninput event listener. In
    // practice though any element with `contenteditable=true` may generate such events,
    // therefore here we fall back to just returning the text content of the node.
    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
    let (v1, v2) = cfg_match! {
        feature = "std_web" => ({
            (
                this.clone()
                    .try_into()
                    .map(|input: InputElement| input.raw_value())
                    .ok(),
                this.clone()
                    .try_into()
                    .map(|input: TextAreaElement| input.value())
                    .ok(),
            )
        }),
        feature = "web_sys" => ({
            (
                this.dyn_ref().map(|input: &InputElement| input.value()),
                this.dyn_ref().map(|input: &TextAreaElement| input.value()),
            )
        }),
    };
    let v3 = this.text_content();
    let value = v1.or(v2).or(v3)
        .expect("only an InputElement or TextAreaElement or an element with contenteditable=true can have an oninput event listener");
    InputData { value }
}

fn onchange_handler(this: &Element) -> ChangeData {
    match this.node_name().as_ref() {
        "INPUT" => {
            let input = cfg_match! {
                feature = "std_web" => InputElement::try_from(this.clone()).unwrap(),
                feature = "web_sys" => this.dyn_ref::<InputElement>().unwrap(),
            };
            let is_file = input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false);
            if is_file {
                let files: FileList = cfg_match! {
                    feature = "std_web" => js!( return @{input}.files; ).try_into().unwrap(),
                    feature = "web_sys" => input.files().unwrap(),
                };
                ChangeData::Files(files)
            } else {
                cfg_match! {
                    feature = "std_web" => ChangeData::Value(input.raw_value()),
                    feature = "web_sys" => ChangeData::Value(input.value()),
                }
            }
        }
        "TEXTAREA" => {
            let tae = cfg_match! {
                feature = "std_web" => TextAreaElement::try_from(this.clone()).unwrap(),
                feature = "web_sys" => this.dyn_ref::<TextAreaElement>().unwrap(),
            };
            ChangeData::Value(tae.value())
        }
        "SELECT" => {
            let se = cfg_match! {
                feature = "std_web" => SelectElement::try_from(this.clone()).unwrap(),
                feature = "web_sys" => this.dyn_ref::<SelectElement>().unwrap().clone(),
            };
            ChangeData::Select(se)
        }
        _ => {
            panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
        }
    }
}

/// Handler to an event listener, only use is to cancel the event.
#[cfg(feature = "std_web")]
#[derive(Debug)]
pub struct EventListener(Option<EventListenerHandle>);

#[cfg(feature = "std_web")]
impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(event) = self.0.take() {
            event.remove()
        }
    }
}
