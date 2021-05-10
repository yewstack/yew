use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};

pub fn base_url() -> Option<String> {
    match yew::utils::document().query_selector("base[href]") {
        Ok(Some(base)) => {
            let base = base.unchecked_into::<web_sys::HtmlBaseElement>().href();

            let url = web_sys::Url::new(&base).unwrap();
            let base = url.pathname();

            let base = if base != "/" {
                base.strip_suffix("/")
                    .map(|it| it.to_string())
                    .unwrap_or(base)
            } else {
                base
            };
            Some(base)
        }
        _ => None,
    }
}

pub fn build_path_with_base(to: &str) -> String {
    let to = format!("{}{}", base_url().as_deref().unwrap_or(""), to);

    let path = if to == "/" {
        to
    } else {
        to.strip_suffix("/").map(|it| it.to_string()).unwrap_or(to)
    };

    path
}

pub fn get_query_params() -> HashMap<String, String> {
    let url = web_sys::Url::new(&yew::utils::document().url().unwrap()).unwrap();

    let iter = js_sys::try_iter(&JsValue::from(&url.search_params()))
        .expect("try_iter failed")
        .expect("try_iter failed")
        .into_iter()
        .map(|it| it.unwrap().unchecked_into::<js_sys::Array>().to_vec())
        .map(|it| {
            let mut iter = it.into_iter();
            // unwraps are unreachable
            // there will be at least 2 values here
            // both of them will be strings
            (
                iter.next().unwrap().as_string().unwrap(),
                iter.next().unwrap().as_string().unwrap(),
            )
        });

    iter.collect()
}
