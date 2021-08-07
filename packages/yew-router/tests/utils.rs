use wasm_bindgen::JsCast;

pub fn obtain_result_by_id(id: &str) -> String {
    yew::utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

pub fn click(selector: &str) {
    yew::utils::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .click();
}
