use std::cell::RefCell;

use wasm_bindgen::JsCast;

pub fn find_base_url() -> Option<String> {
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

pub fn base_url() -> Option<String> {
    thread_local! {
        static BASE_URL: RefCell<Option<Option<String>>> = RefCell::new(None);
    }

    BASE_URL.with(|maybe_base_url| {
        let mut maybe_base_url = maybe_base_url.borrow_mut();
        if let Some(base_url) = &*maybe_base_url {
            base_url.clone()
        } else {
            let base_url = find_base_url();
            *maybe_base_url = Some(base_url.clone());
            base_url
        }
    })
}
