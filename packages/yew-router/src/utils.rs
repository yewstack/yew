use wasm_bindgen::JsCast;

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
                return None;
            };

            Some(base)
        }
        _ => None,
    }
}

pub fn build_path_with_base(to: &str) -> String {
    format!(
        "{}{}",
        base_url()
            .as_deref()
            .map(|it| it.strip_suffix("/").unwrap_or(it))
            .unwrap_or(""),
        to
    )
}
