#[macro_use]
mod events;

use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use crate::Callback;
pub use events::*;

/// A type representing data from `oninput` event.
#[derive(Debug)]
pub struct InputData {
    /// Inserted characters. Contains value from
    /// [InputEvent](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data).
    pub value: String,
    /// The InputEvent received.
    pub event: web_sys::InputEvent,
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
    /// to collect your required data such as `value` and `selected_index`.
    /// You can also iterate throught `selected_options` yourself, this does require adding the
    /// [web-sys](https://crates.io/crates/web-sys) crate with the `HtmlCollection` feature.
    Select(HtmlSelectElement),
    /// Files
    Files(web_sys::FileList),
}

/// Extract target as Element from event
fn extract_target(event: &Event) -> Element {
    event
        .target()
        .expect("no target on event")
        .dyn_into()
        .unwrap()
}

pub(crate) fn cast_event<T>(e: Event) -> T
where
    T: wasm_bindgen::JsCast,
{
    e.unchecked_into()
}

pub(crate) fn oninput_handler(event: Event) -> InputData {
    let this = extract_target(&event);
    let event: web_sys::InputEvent = cast_event(event);

    // Normally only InputElement or TextAreaElement can have an oninput event listener. In
    // practice though any element with `contenteditable=true` may generate such events,
    // therefore here we fall back to just returning the text content of the node.
    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event.
    let value = this
        .dyn_ref()
        .map(|input: &HtmlInputElement| input.value())
        .or_else(|| {
            this.dyn_ref()
                .map(|input: &HtmlTextAreaElement| input.value())
        })
        .or_else(|| this.text_content())
        .expect(concat!(
            "only an InputElement or TextAreaElement or an element with contenteditable=true ",
            "can have an oninput event listener"
        ));
    InputData { value, event }
}

pub(crate) fn onchange_handler(event: Event) -> ChangeData {
    let this = extract_target(&event);

    match this.node_name().as_ref() {
        "INPUT" => {
            let input = this.dyn_into::<HtmlInputElement>().unwrap();
            if input
                .get_attribute("type")
                .map(|value| value.eq_ignore_ascii_case("file"))
                .unwrap_or(false)
            {
                ChangeData::Files(input.files().unwrap())
            } else {
                ChangeData::Value(input.value())
            }
        }
        "TEXTAREA" => ChangeData::Value(this.dyn_into::<HtmlTextAreaElement>().unwrap().value()),
        "SELECT" => ChangeData::Select(this.dyn_into::<HtmlSelectElement>().unwrap()),
        _ => {
            panic!(concat!(
                "only an InputElement, TextAreaElement or SelectElement ",
                "can have an onchange event listener"
            ));
        }
    }
}

/// A trait similar to `Into<T>` which allows conversion of a value into a [`Callback`].
/// This is used for event listeners.
pub trait IntoEventCallback<EVENT> {
    /// Convert `self` to `Option<Callback<EVENT>>`
    fn into_event_callback(self) -> Option<Callback<EVENT>>;
}

impl<EVENT> IntoEventCallback<EVENT> for Callback<EVENT> {
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(self)
    }
}

impl<EVENT> IntoEventCallback<EVENT> for &Callback<EVENT> {
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(self.clone())
    }
}

impl<EVENT> IntoEventCallback<EVENT> for Option<Callback<EVENT>> {
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        self
    }
}

impl<T, EVENT> IntoEventCallback<EVENT> for T
where
    T: Fn(EVENT) + 'static,
{
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(Callback::from(self))
    }
}

impl<T, EVENT> IntoEventCallback<EVENT> for Option<T>
where
    T: Fn(EVENT) + 'static,
{
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(Callback::from(self?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_into_event_callback_types() {
        let f = |_: usize| ();
        let cb = Callback::from(f);

        // Callbacks
        let _: Option<Callback<usize>> = cb.clone().into_event_callback();
        let _: Option<Callback<usize>> = (&cb).into_event_callback();
        let _: Option<Callback<usize>> = Some(cb).into_event_callback();

        // Fns
        let _: Option<Callback<usize>> = f.into_event_callback();
        let _: Option<Callback<usize>> = Some(f).into_event_callback();
    }
}
