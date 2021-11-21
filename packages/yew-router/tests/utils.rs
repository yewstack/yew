use wasm_bindgen::JsCast;

pub fn obtain_result_by_id(id: &str) -> String {
    gloo_utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

pub fn click(selector: &str) {
    gloo_utils::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .click();
}

pub fn history_length() -> u32 {
    gloo_utils::window()
        .history()
        .expect("No history found")
        .length()
        .expect("No history length found")
}
