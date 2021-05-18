use wasm_bindgen::JsCast;

pub(crate) fn strip_slash_suffix(path: &str) -> &str {
    path.strip_suffix("/").unwrap_or(&path)
}

pub fn base_url() -> Option<String> {
    match yew::utils::document().query_selector("base[href]") {
        Ok(Some(base)) => {
            let base = base.unchecked_into::<web_sys::HtmlBaseElement>().href();

            let url = web_sys::Url::new(&base).unwrap();
            let base = url.pathname();

            let base = if base != "/" {
                strip_slash_suffix(&base)
            } else {
                return None;
            };

            Some(base.to_string())
        }
        _ => None,
    }
}

pub fn build_path_with_base(to: &str) -> String {
    let path = format!(
        "{}{}",
        base_url().as_deref().map(strip_slash_suffix).unwrap_or(""),
        to
    );
    strip_slash_suffix(&path).to_string()
}
