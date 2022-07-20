#![allow(dead_code)]

pub fn obtain_result() -> String {
    gloo::utils::document()
        .get_element_by_id("result")
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

pub fn obtain_result_by_id(id: &str) -> String {
    gloo::utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

pub fn output_element() -> web_sys::Element {
    gloo::utils::document().get_element_by_id("output").unwrap()
}
