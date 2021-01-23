#[macro_use]
mod macros;
mod events;

use wasm_bindgen::JsCast;
use web_sys::{
    Element, FileList, HtmlInputElement as InputElement, HtmlSelectElement as SelectElement,
    HtmlTextAreaElement as TextAreaElement, InputEvent,
};

pub use events::*;

/// A type representing data from `oninput` event.
#[derive(Debug)]
pub struct InputData {
    /// Inserted characters. Contains value from
    /// [InputEvent](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data).
    pub value: String,
    /// The InputEvent received.
    pub event: InputEvent,
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

fn oninput_handler(this: &Element, event: InputEvent) -> InputData {
    // Normally only InputElement or TextAreaElement can have an oninput event listener. In
    // practice though any element with `contenteditable=true` may generate such events,
    // therefore here we fall back to just returning the text content of the node.
    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
    let (v1, v2) = (
        this.dyn_ref().map(|input: &InputElement| input.value()),
        this.dyn_ref().map(|input: &TextAreaElement| input.value()),
    );
    let v3 = this.text_content();
    let value = v1.or(v2).or(v3)
        .expect("only an InputElement or TextAreaElement or an element with contenteditable=true can have an oninput event listener");
    InputData { value, event }
}

fn onchange_handler(this: &Element) -> ChangeData {
    match this.node_name().as_ref() {
        "INPUT" => {
            let input = this.dyn_ref::<InputElement>().unwrap();
            let is_file = input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false);
            if is_file {
                let files: FileList = input.files().unwrap();
                ChangeData::Files(files)
            } else {
                ChangeData::Value(input.value())
            }
        }
        "TEXTAREA" => {
            let tae = this.dyn_ref::<TextAreaElement>().unwrap();
            ChangeData::Value(tae.value())
        }
        "SELECT" => {
            let se = this.dyn_ref::<SelectElement>().unwrap().clone();
            ChangeData::Select(se)
        }
        _ => {
            panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
        }
    }
}
