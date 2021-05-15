use std::collections::HashMap;
use wasm_bindgen::JsCast;

fn strip_slash(path: String) -> String {
    if path != "/" {
        path.strip_suffix("/")
            .map(|it| it.to_string())
            .unwrap_or(path)
    } else {
        path
    }
}

pub fn base_url() -> Option<String> {
    match yew::utils::document().query_selector("base[href]") {
        Ok(Some(base)) => {
            let base = base.unchecked_into::<web_sys::HtmlBaseElement>().href();

            let url = web_sys::Url::new(&base).unwrap();
            let base = url.pathname();

            let base = strip_slash(base);
            Some(base)
        }
        _ => None,
    }
}

pub fn build_path_with_base(to: &str) -> String {
    let to = format!("{}{}", base_url().as_deref().unwrap_or(""), to);

    strip_slash(to)
}

pub fn get_query_params() -> HashMap<String, String> {
    let url = web_sys::Url::new(&yew::utils::document().url().unwrap()).unwrap();
    let search_params = js_sys::Array::from(url.search_params().as_ref()).to_vec();
    search_params
        .into_iter()
        .map(|value| js_sys::Array::from(&value).to_vec())
        .map(|chunk| {
            (
                chunk[0].as_string().expect("0"),
                chunk[1].as_string().expect("1"),
            )
        })
        .collect()
}
