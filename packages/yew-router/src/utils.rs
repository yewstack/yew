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
