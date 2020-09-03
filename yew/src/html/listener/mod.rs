use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod listener_stdweb;
        pub use listener_stdweb::*;

        use stdweb::web::FileList;
        use stdweb::web::event::InputEvent;
        use stdweb::web::html_element::SelectElement;
    } else if #[cfg(feature = "web_sys")] {
        mod listener_web_sys;
        pub use listener_web_sys::*;

        mod registry;
        pub(crate) use registry::*;

        use web_sys::{FileList, InputEvent, HtmlSelectElement as SelectElement};
    }
}

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
    /// `selected_values`. You can also iterate through `selected_options` yourself.
    Select(SelectElement),
    /// Files
    Files(FileList),
}
