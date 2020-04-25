//! Service to send HTTP-request to a server.

cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
        pub use std_web::*;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
        pub use self::web_sys::*;
    }
}

/// Type to set referrer for fetch.
#[derive(Debug)]
pub enum Referrer {
    /// `<same-origin URL>` value of referrer.
    SameOriginUrl(String),
    /// `about:client` value of referrer.
    AboutClient,
    /// `<empty string>` value of referrer.
    Empty,
}

#[cfg(test)]
#[cfg(feature = "wasm_test")]
mod tests {
    use super::*;
    use crate::callback::{test_util::CallbackFuture, Callback};
    use crate::format::{Json, Nothing};
    use crate::utils;
    #[cfg(feature = "web_sys")]
    use ::web_sys::ReferrerPolicy;
    use serde::Deserialize;
    use ssri::Integrity;
    use std::collections::HashMap;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Deserialize, Debug)]
    struct HttpBin {
        headers: HashMap<String, String>,
        origin: String,
        url: String,
    }

    #[derive(Deserialize, Debug)]
    struct HttpBinHeaders {
        headers: HashMap<String, String>,
    }

    #[test]
    async fn fetch_referrer_default() {
        let request = Request::get("https://httpbin.org/get")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions::default();
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert!(http_bin.headers.get("Referer").is_some());
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_referrer_same_origin_url() {
        let request = Request::get("https://httpbin.org/get")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer: Some(Referrer::SameOriginUrl(String::from("same-origin"))),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            let referrer = http_bin.headers.get("Referer").expect("no referer set");
            assert!(referrer.ends_with("/same-origin"));
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_referrer_about_client() {
        let request = Request::get("https://httpbin.org/get")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer: Some(Referrer::AboutClient),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert!(http_bin.headers.get("Referer").is_some());
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_referrer_empty() {
        let request = Request::get("https://httpbin.org/get")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer: Some(Referrer::Empty),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert!(http_bin.headers.get("Referer").is_none());
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_redirect_default() {
        let request = Request::get("https://httpbin.org/relative-redirect/1")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions::default();
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert_eq!(http_bin.url, String::from("https://httpbin.org/get"));
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_redirect_follow() {
        let request = Request::get("https://httpbin.org/relative-redirect/1")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            redirect: Some(Redirect::Follow),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert_eq!(http_bin.url, String::from("https://httpbin.org/get"));
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_redirect_error() {
        let request = Request::get("https://httpbin.org/relative-redirect/1")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            redirect: Some(Redirect::Error),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
    }

    #[test]
    async fn fetch_redirect_manual() {
        let request = Request::get("https://httpbin.org/relative-redirect/1")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            redirect: Some(Redirect::Manual),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        // body is empty because the response is opaque for manual redirects
        assert_eq!(resp.body().as_ref().unwrap(), &String::from(""));
    }

    #[test]
    async fn fetch_integrity() {
        let resource = "Yew SRI Test";
        let request = Request::get(format!(
            "https://httpbin.org/base64/{}",
            base64::encode_config(resource, base64::URL_SAFE)
        ))
        .body(Nothing)
        .unwrap();
        let options = FetchOptions {
            integrity: Some(Integrity::from(resource).to_string()),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().as_ref().unwrap(), resource);
    }

    #[test]
    async fn fetch_integrity_fail() {
        let resource = "Yew SRI Test";
        let request = Request::get(format!(
            "https://httpbin.org/base64/{}",
            base64::encode_config(resource, base64::URL_SAFE)
        ))
        .body(Nothing)
        .unwrap();
        let options = FetchOptions {
            integrity: Some(Integrity::from("Yew SRI Test fail").to_string()),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert!(resp.body().is_err());
    }

    #[test]
    async fn fetch_fail() {
        let request = Request::get("https://fetch.fail").body(Nothing).unwrap();
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch(request, callback);
        let resp = cb_future.await;
        #[cfg(feature = "std_web")]
        assert!(resp.body().is_err());
        #[cfg(feature = "web_sys")]
        assert_eq!(
            "TypeError: NetworkError when attempting to fetch resource.",
            resp.body().as_ref().unwrap_err().to_string()
        );
    }

    #[test]
    async fn fetch_referrer_policy_no_referrer() {
        let request = Request::get("https://httpbin.org/headers")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer_policy: Some(ReferrerPolicy::NoReferrer),
            ..FetchOptions::default()
        };
        let cb_future =
            CallbackFuture::<Response<Json<Result<HttpBinHeaders, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(httpbin_headers)) = resp.body() {
            assert_eq!(httpbin_headers.headers.get("Referer"), None);
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_referrer_policy_origin() {
        let request = Request::get("https://httpbin.org/headers")
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer_policy: Some(ReferrerPolicy::Origin),
            ..FetchOptions::default()
        };
        let cb_future =
            CallbackFuture::<Response<Json<Result<HttpBinHeaders, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::new().fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(httpbin_headers)) = resp.body() {
            assert!(httpbin_headers
                .headers
                .get("Referer")
                .unwrap()
                .starts_with(&utils::origin().unwrap()));
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }
}
