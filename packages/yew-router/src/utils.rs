use std::cell::RefCell;

use wasm_bindgen::JsCast;

type BaseUrl = Option<String>;

pub fn find_base_url() -> BaseUrl {
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

pub fn base_url() -> BaseUrl {
    thread_local! {
        static BASE_URL: RefCell<Option<BaseUrl>> = RefCell::new(None);
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
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test as test;
    use yew::utils::*;
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

    impl Default for Routes {
        fn default() -> Self {
            Self::NotFound
        }
    }

    #[test]
    fn test_base_url() {
        document().head().unwrap().set_inner_html(r#""#);

        assert_eq!(find_base_url(), None);

        document()
            .head()
            .unwrap()
            .set_inner_html(r#"<base href="/base/">"#);
        assert_eq!(find_base_url(), Some("/base".to_string()));

        document()
            .head()
            .unwrap()
            .set_inner_html(r#"<base href="/base">"#);
        assert_eq!(find_base_url(), Some("/base".to_string()));
    }
}
