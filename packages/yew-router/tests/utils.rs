use wasm_bindgen::JsCast;

#[allow(dead_code)]
pub fn obtain_result_by_id(id: &str) -> String {
    gloo::utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

#[allow(dead_code)]
pub fn click(selector: &str) {
    gloo::utils::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .click();
}

#[allow(dead_code)]
pub fn history_length() -> u32 {
    gloo::utils::window()
        .history()
        .expect("No history found")
        .length()
        .expect("No history length found")
}

#[allow(dead_code)]
pub fn link_href(selector: &str) -> String {
    gloo::utils::document()
        .query_selector(selector)
        .expect("Failed to run query selector")
        .unwrap_or_else(|| panic!("No such link: {}", selector))
        .get_attribute("href")
        .expect("No href attribute")
}
