use std::cell::RefCell;
use wasm_bindgen::JsCast;

pub(crate) fn strip_slash_suffix(path: &str) -> &str {
    path.strip_suffix('/').unwrap_or(path)
}

static BASE_URL_LOADED: std::sync::Once = std::sync::Once::new();
thread_local! {
    static BASE_URL: RefCell<Option<String>> = RefCell::new(None);
}

// This exists so we can cache the base url. It costs us a `to_string` call instead of a DOM API call.
// Considering base urls are generally short, it *should* be less expensive.
pub fn base_url() -> Option<String> {
    BASE_URL_LOADED.call_once(|| {
        BASE_URL.with(|val| {
            *val.borrow_mut() = fetch_base_url();
        })
    });
    BASE_URL.with(|it| it.borrow().as_ref().map(|it| it.to_string()))
}

pub fn fetch_base_url() -> Option<String> {
    match gloo_utils::document().query_selector("base[href]") {
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

#[cfg(test)]
mod tests {
    use gloo_utils::document;
    use wasm_bindgen_test::wasm_bindgen_test as test;
    use yew_router::prelude::*;
    use yew_router::utils::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, Clone, Copy, PartialEq, Routable)]
    enum Routes {
        #[at("/")]
        Home,
        #[at("/no")]
        No,
        #[at("/404")]
        NotFound,
    }

    #[test]
    fn test_base_url() {
        document().head().unwrap().set_inner_html(r#""#);

        assert_eq!(fetch_base_url(), None);

        document()
            .head()
            .unwrap()
            .set_inner_html(r#"<base href="/base/">"#);
        assert_eq!(fetch_base_url(), Some("/base".to_string()));

        document()
            .head()
            .unwrap()
            .set_inner_html(r#"<base href="/base">"#);
        assert_eq!(fetch_base_url(), Some("/base".to_string()));
    }
}
