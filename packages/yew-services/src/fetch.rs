//! Service to send HTTP-request to a server.

use crate::Task;
use anyhow::{anyhow, Error};
use http::request::Parts;
use js_sys::{Array, Promise, Uint8Array};
use std::cell::RefCell;
use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::rc::Rc;
use thiserror::Error as ThisError;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    AbortController, Headers, ReferrerPolicy, Request as WebRequest, RequestInit,
    Response as WebResponse,
};
use yew::callback::Callback;
use yew::format::{Binary, Format, Text};

#[doc(no_inline)]
pub use web_sys::{
    RequestCache as Cache, RequestCredentials as Credentials, RequestMode as Mode,
    RequestRedirect as Redirect, Window, WorkerGlobalScope,
};

#[doc(no_inline)]
pub use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};

trait JsInterop: Sized {
    fn from_js(js_value: JsValue) -> Result<Self, FetchError>;
    fn into_js(self) -> JsValue;
}

impl JsInterop for Vec<u8> {
    fn from_js(js_value: JsValue) -> Result<Self, FetchError> {
        Ok(Uint8Array::new(&js_value).to_vec())
    }

    fn into_js(self) -> JsValue {
        Uint8Array::from(self.as_slice()).into()
    }
}

impl JsInterop for String {
    fn from_js(js_value: JsValue) -> Result<Self, FetchError> {
        js_value.as_string().ok_or(FetchError::InternalError)
    }

    fn into_js(self) -> JsValue {
        self.into()
    }
}

/// Init options for `fetch()` function call.
/// https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch
#[derive(Default, Debug)]
pub struct FetchOptions {
    /// Cache of a fetch request.
    pub cache: Option<Cache>,
    /// Credentials of a fetch request.
    pub credentials: Option<Credentials>,
    /// Redirect behaviour of a fetch request.
    pub redirect: Option<Redirect>,
    /// Request mode of a fetch request.
    pub mode: Option<Mode>,
    /// Referrer of a fetch request.
    pub referrer: Option<Referrer>,
    /// Referrer policy of a fetch request.
    pub referrer_policy: Option<ReferrerPolicy>,
    /// Integrity of a fetch request.
    pub integrity: Option<String>,
}

impl Into<RequestInit> for FetchOptions {
    fn into(self) -> RequestInit {
        let mut init = RequestInit::new();

        if let Some(cache) = self.cache {
            init.cache(cache);
        }

        if let Some(credentials) = self.credentials {
            init.credentials(credentials);
        }

        if let Some(redirect) = self.redirect {
            init.redirect(redirect);
        }

        if let Some(mode) = self.mode {
            init.mode(mode);
        }

        if let Some(referrer) = self.referrer {
            match referrer {
                Referrer::SameOriginUrl(referrer) => init.referrer(&referrer),
                Referrer::AboutClient => init.referrer("about:client"),
                Referrer::Empty => init.referrer(""),
            };
        }

        if let Some(referrer_policy) = self.referrer_policy {
            init.referrer_policy(referrer_policy);
        }

        if let Some(integrity) = self.integrity {
            init.integrity(&integrity);
        }

        init
    }
}

// convert `headers` to `Iterator<Item = (String, String)>`
fn header_iter(headers: Headers) -> impl Iterator<Item = (String, String)> {
    js_sys::try_iter(&headers)
        .unwrap()
        .unwrap()
        .map(Result::unwrap)
        .map(|entry| {
            let entry = Array::from(&entry);
            let key = entry.get(0);
            let value = entry.get(1);
            (key.as_string().unwrap(), value.as_string().unwrap())
        })
}

/// Represents errors of a fetch service.
#[derive(Debug, ThisError)]
enum FetchError {
    #[error("canceled")]
    Canceled,
    #[error("{0}")]
    FetchFailed(String),
    #[error("invalid response")]
    InvalidResponse,
    #[error("unexpected error, please report")]
    InternalError,
}

#[derive(Debug)]
struct Handle {
    active: Rc<RefCell<bool>>,
    abort_controller: Option<AbortController>,
}

/// A handle to control sent requests.
#[must_use = "the request will be cancelled when the task is dropped"]
pub struct FetchTask(Handle);

impl fmt::Debug for FetchTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FetchTask")
    }
}

/// A service to fetch resources.
#[derive(Default, Debug)]
pub struct FetchService {}

impl FetchService {
    /// Sends a request to a remote server given a Request object and a callback
    /// function to convert a Response object into a loop's message.
    ///
    /// You may use a Request builder to build your request declaratively as on the
    /// following examples:
    ///
    /// ```
    ///# use serde_json::json;
    ///# use yew::format::{Nothing, Json};
    ///# use yew_services::fetch::Request;
    /// let post_request = Request::post("https://my.api/v1/resource")
    ///     .header("Content-Type", "application/json")
    ///     .body(Json(&json!({"foo": "bar"})))
    ///     .expect("Failed to build request.");
    ///
    /// let get_request = Request::get("https://my.api/v1/resource")
    ///     .body(Nothing)
    ///     .expect("Failed to build request.");
    /// ```
    ///
    /// The callback function can build a loop message by passing or analyzing the
    /// response body and metadata.
    ///
    /// ```
    ///# use anyhow::Error;
    ///# use yew::{Component, ComponentLink, Html};
    ///# use yew_services::FetchService;
    ///# use yew_services::fetch::{Response, Request};
    ///# struct Comp;
    ///# impl Component for Comp {
    ///#     type Message = Msg;type Properties = ();
    ///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
    ///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
    ///#     fn change(&mut self, _: Self::Properties) -> bool {unimplemented!()}
    ///#     fn view(&self) -> Html {unimplemented!()}
    ///# }
    ///# enum Msg {
    ///#     Noop,
    ///#     Error
    ///# }
    ///# fn dont_execute() {
    ///# let link: ComponentLink<Comp> = unimplemented!();
    ///# let post_request: Request<Result<String, Error>> = unimplemented!();
    /// let task = FetchService::fetch(
    ///     post_request,
    ///     link.callback(|response: Response<Result<String, Error>>| {
    ///         if response.status().is_success() {
    ///             Msg::Noop
    ///         } else {
    ///             Msg::Error
    ///         }
    ///     }),
    /// );
    ///# }
    /// ```
    ///
    /// For a full example, you can specify that the response must be in the JSON format,
    /// and be a specific serialized data type. If the message isn't JSON, or isn't the specified
    /// data type, then you will get an error message.
    ///
    /// ```
    ///# use anyhow::Error;
    ///# use http::Request;
    ///# use serde::Deserialize;
    ///# use yew::{Component, ComponentLink, Html};
    ///# use yew::format::{Json, Nothing, Format};
    ///# use yew_services::fetch::Response;
    ///# use yew_services::FetchService;
    ///# struct Comp;
    ///# impl Component for Comp {
    ///#     type Message = Msg;type Properties = ();
    ///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
    ///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
    ///#     fn change(&mut self, _: Self::Properties) -> bool {unimplemented!()}
    ///#     fn view(&self) -> Html {unimplemented!()}
    ///# }
    ///# enum Msg {
    ///#     FetchResourceComplete(Data),
    ///#     FetchResourceFailed
    ///# }
    /// #[derive(Deserialize)]
    /// struct Data {
    ///    value: String
    /// }
    ///
    ///# fn dont_execute() {
    ///# let link: ComponentLink<Comp> = unimplemented!();
    /// let get_request = Request::get("/thing").body(Nothing).unwrap();
    /// let callback = link.callback(|response: Response<Json<Result<Data, Error>>>| {
    ///     if let (meta, Json(Ok(body))) = response.into_parts() {
    ///         if meta.status.is_success() {
    ///             return Msg::FetchResourceComplete(body);
    ///         }
    ///     }
    ///     Msg::FetchResourceFailed
    /// });
    ///
    /// let task = FetchService::fetch(get_request, callback);
    ///# }
    /// ```
    ///
    pub fn fetch<IN, OUT: 'static>(
        request: Request<IN>,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, Error>
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        fetch_impl::<IN, OUT, String>(false, request, None, callback)
    }

    /// `fetch` with provided `FetchOptions` object.
    /// Use it if you need to send cookies with a request:
    /// ```
    ///# use anyhow::Error;
    ///# use http::Response;
    ///# use yew::format::Nothing;
    ///# use yew::{Html, Component, ComponentLink};
    ///# use yew_services::fetch::{self, FetchOptions, FetchService, Credentials};
    ///# struct Comp;
    ///# impl Component for Comp {
    ///#     type Message = Msg;
    ///#     type Properties = ();
    ///#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {unimplemented!()}
    ///#     fn update(&mut self, msg: Self::Message) -> bool {unimplemented!()}
    ///#     fn change(&mut self, _: Self::Properties) -> bool {unimplemented!()}
    ///#     fn view(&self) -> Html {unimplemented!()}
    ///# }
    ///# pub enum Msg {}
    ///# fn dont_execute() {
    ///# let link: ComponentLink<Comp> = unimplemented!();
    ///# let callback = link.callback(|response: Response<Result<String, Error>>|  -> Msg { unimplemented!() });
    /// let request = fetch::Request::get("/path/")
    ///     .body(Nothing)
    ///     .unwrap();
    /// let options = FetchOptions {
    ///     credentials: Some(Credentials::SameOrigin),
    ///     ..FetchOptions::default()
    /// };
    /// let task = FetchService::fetch_with_options(request, options, callback);
    ///# }
    /// ```
    pub fn fetch_with_options<IN, OUT: 'static>(
        request: Request<IN>,
        options: FetchOptions,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, Error>
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        fetch_impl::<IN, OUT, String>(false, request, Some(options), callback)
    }

    /// Fetch the data in binary format.
    pub fn fetch_binary<IN, OUT: 'static>(
        request: Request<IN>,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, Error>
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>>(true, request, None, callback)
    }

    /// Fetch the data in binary format with the provided request options.
    pub fn fetch_binary_with_options<IN, OUT: 'static>(
        request: Request<IN>,
        options: FetchOptions,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, Error>
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>>(true, request, Some(options), callback)
    }
}

fn fetch_impl<IN, OUT: 'static, DATA: 'static>(
    binary: bool,
    request: Request<IN>,
    options: Option<FetchOptions>,
    callback: Callback<Response<OUT>>,
) -> Result<FetchTask, Error>
where
    DATA: JsInterop,
    IN: Into<Format<DATA>>,
    OUT: From<Format<DATA>>,
{
    // Transform http::Request into WebRequest.
    let (parts, body) = request.into_parts();
    let body = match body.into() {
        Ok(b) => b.into_js(),
        Err(_) => JsValue::NULL,
    };
    let request = build_request(parts, &body)?;

    // Transform FetchOptions into RequestInit.
    let abort_controller = AbortController::new().ok();
    let mut init = options.map_or_else(RequestInit::new, Into::into);
    if let Some(abort_controller) = &abort_controller {
        init.signal(Some(&abort_controller.signal()));
    }

    // Start fetch
    let promise = GLOBAL.with(|global| global.fetch_with_request_and_init(&request, &init));

    // Spawn future to resolve fetch
    let active = Rc::new(RefCell::new(true));
    let data_fetcher = DataFetcher::new(binary, callback, active.clone());
    spawn_local(DataFetcher::fetch_data(data_fetcher, promise));

    Ok(FetchTask(Handle {
        active,
        abort_controller,
    }))
}

struct DataFetcher<OUT: 'static, DATA>
where
    DATA: JsInterop,
    OUT: From<Format<DATA>>,
{
    binary: bool,
    active: Rc<RefCell<bool>>,
    callback: Callback<Response<OUT>>,
    _marker: PhantomData<DATA>,
}

impl<OUT: 'static, DATA> DataFetcher<OUT, DATA>
where
    DATA: JsInterop,
    OUT: From<Format<DATA>>,
{
    fn new(binary: bool, callback: Callback<Response<OUT>>, active: Rc<RefCell<bool>>) -> Self {
        Self {
            binary,
            callback,
            active,
            _marker: PhantomData::default(),
        }
    }

    async fn fetch_data(self, promise: Promise) {
        let result = self.fetch_data_impl(promise).await;
        let (data, status, headers) = match result {
            Ok((data, response)) => (Ok(data), response.status(), Some(response.headers())),
            Err(err) => (Err(err), 408, None),
        };
        self.callback(data, status, headers);
    }

    async fn fetch_data_impl(&self, promise: Promise) -> Result<(DATA, WebResponse), Error> {
        let response = self.get_response(promise).await?;
        let data = self.get_data(&response).await?;
        Ok((data, response))
    }

    // Prepare the response callback.
    // Notice that the callback signature must match the call from the javascript
    // side. There is no static check at this point.
    fn callback(&self, data: Result<DATA, Error>, status: u16, headers: Option<Headers>) {
        let mut response_builder = Response::builder();
        if let Ok(status) = StatusCode::from_u16(status) {
            response_builder = response_builder.status(status);
        }

        if let Some(headers) = headers {
            for (key, value) in header_iter(headers) {
                response_builder = response_builder.header(key.as_str(), value.as_str());
            }
        }

        // Deserialize and wrap response data into a Text or Binary object.
        let response = response_builder
            .body(OUT::from(data))
            .expect("failed to build response, please report");
        *self.active.borrow_mut() = false;
        self.callback.emit(response);
    }

    async fn get_response(&self, fetch_promise: Promise) -> Result<WebResponse, FetchError> {
        let response = JsFuture::from(fetch_promise)
            .await
            .map_err(|err| err.unchecked_into::<js_sys::Error>())
            .map_err(|err| FetchError::FetchFailed(err.to_string().as_string().unwrap()))?;
        if *self.active.borrow() {
            Ok(WebResponse::from(response))
        } else {
            Err(FetchError::Canceled)
        }
    }

    async fn get_data(&self, response: &WebResponse) -> Result<DATA, FetchError> {
        let data_promise = if self.binary {
            response.array_buffer()
        } else {
            response.text()
        }
        .map_err(|_| FetchError::InvalidResponse)?;

        let data_result = JsFuture::from(data_promise).await;
        if *self.active.borrow() {
            data_result
                .map_err(|_| FetchError::InvalidResponse)
                .and_then(DATA::from_js)
        } else {
            Err(FetchError::Canceled)
        }
    }
}

fn build_request(parts: Parts, body: &JsValue) -> Result<WebRequest, Error> {
    // Map headers into a Js `Header` type.
    let header_list = parts
        .headers
        .iter()
        .map(|(k, v)| {
            Ok(Array::from_iter(&[
                JsValue::from_str(k.as_str()),
                JsValue::from_str(
                    v.to_str()
                        .map_err(|_| anyhow!("Unparsable request header"))?,
                ),
            ]))
        })
        .collect::<Result<Array, Error>>()?;

    let header_map = Headers::new_with_str_sequence_sequence(&header_list)
        .map_err(|_| anyhow!("couldn't build headers"))?;

    // Formats URI.
    let uri = parts.uri.to_string();
    let method = parts.method.as_str();
    let mut init = RequestInit::new();
    init.method(method).body(Some(body)).headers(&header_map);
    WebRequest::new_with_str_and_init(&uri, &init).map_err(|_| anyhow!("failed to build request"))
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        *self.0.active.borrow()
    }
}

impl Drop for FetchTask {
    fn drop(&mut self) {
        if self.is_active() {
            // Fetch API doesn't support request cancelling in all browsers
            // and we should use this workaround with a flag.
            // In that case, request not canceled, but callback won't be called.
            *self.0.active.borrow_mut() = false;
            if let Some(abort_controller) = &self.0.abort_controller {
                abort_controller.abort();
            }
        }
    }
}

thread_local! {
    static GLOBAL: WindowOrWorker = WindowOrWorker::new();
}

enum WindowOrWorker {
    Window(Window),
    Worker(WorkerGlobalScope),
}

impl WindowOrWorker {
    fn new() -> Self {
        #[wasm_bindgen]
        extern "C" {
            type Global;

            #[wasm_bindgen(method, getter, js_name = Window)]
            fn window(this: &Global) -> JsValue;

            #[wasm_bindgen(method, getter, js_name = WorkerGlobalScope)]
            fn worker(this: &Global) -> JsValue;
        }

        let global: Global = js_sys::global().unchecked_into();

        if !global.window().is_undefined() {
            Self::Window(global.unchecked_into())
        } else if !global.worker().is_undefined() {
            Self::Worker(global.unchecked_into())
        } else {
            panic!(
                "Yew's `FetchService` only works when a `window` or `worker` object is available."
            );
        }
    }
}

impl WindowOrWorker {
    fn fetch_with_request_and_init(&self, input: &WebRequest, init: &RequestInit) -> Promise {
        match self {
            Self::Window(window) => window.fetch_with_request_and_init(input, init),
            Self::Worker(worker) => worker.fetch_with_request_and_init(input, init),
        }
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
#[cfg(all(feature = "wasm_test", feature = "httpbin_test"))]
mod tests {
    use super::*;
    use crate::callback_test_util::CallbackFuture;
    use serde::Deserialize;
    use ssri::Integrity;
    use std::collections::HashMap;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::ReferrerPolicy;
    use yew::callback::Callback;
    use yew::format::{Json, Nothing};
    use yew::utils;

    wasm_bindgen_test_configure!(run_in_browser);

    const fn httpbin_base_url() -> &'static str {
        // we can't do this at runtime because we're running in the browser.
        env!("HTTPBIN_URL")
    }

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
        let request = Request::get(format!("{}/get", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions::default();
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
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
        let request = Request::get(format!("{}/get", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer: Some(Referrer::SameOriginUrl(String::from("same-origin"))),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
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
        let request = Request::get(format!("{}/get", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer: Some(Referrer::AboutClient),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
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
        let request = Request::get(format!("{}/get", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer: Some(Referrer::Empty),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
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
        let request = Request::get(format!("{}/relative-redirect/1", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions::default();
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert_eq!(http_bin.url, format!("{}/get", httpbin_base_url()));
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_redirect_follow() {
        let request = Request::get(format!("{}/relative-redirect/1", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            redirect: Some(Redirect::Follow),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Json<Result<HttpBin, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        if let Json(Ok(http_bin)) = resp.body() {
            assert_eq!(http_bin.url, format!("{}/get", httpbin_base_url()));
        } else {
            assert!(false, "unexpected resp: {:#?}", resp);
        }
    }

    #[test]
    async fn fetch_redirect_error() {
        let request = Request::get(format!("{}/relative-redirect/1", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            redirect: Some(Redirect::Error),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
    }

    #[test]
    async fn fetch_redirect_manual() {
        let request = Request::get(format!("{}/relative-redirect/1", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            redirect: Some(Redirect::Manual),
            ..FetchOptions::default()
        };
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        // body is empty because the response is opaque for manual redirects
        assert_eq!(resp.body().as_ref().unwrap(), &String::from(""));
    }

    #[test]
    async fn fetch_integrity() {
        let resource = "Yew SRI Test";
        let request = Request::get(format!(
            "{}/base64/{}",
            httpbin_base_url(),
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
        let _task = FetchService::fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().as_ref().unwrap(), resource);
    }

    #[test]
    async fn fetch_integrity_fail() {
        let resource = "Yew SRI Test";
        let request = Request::get(format!(
            "{}/base64/{}",
            httpbin_base_url(),
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
        let _task = FetchService::fetch_with_options(request, options, callback);
        let resp = cb_future.await;
        assert!(resp.body().is_err());
    }

    #[test]
    async fn fetch_fail() {
        let request = Request::get("https://fetch.fail").body(Nothing).unwrap();
        let cb_future = CallbackFuture::<Response<Result<String, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch(request, callback);
        let resp = cb_future.await;
        assert!(resp
            .body()
            .as_ref()
            .unwrap_err()
            .to_string()
            .starts_with("TypeError:"));
    }

    #[test]
    async fn fetch_referrer_policy_no_referrer() {
        let request = Request::get(format!("{}/headers", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer_policy: Some(ReferrerPolicy::NoReferrer),
            ..FetchOptions::default()
        };
        let cb_future =
            CallbackFuture::<Response<Json<Result<HttpBinHeaders, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
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
        let request = Request::get(format!("{}/headers", httpbin_base_url()))
            .body(Nothing)
            .unwrap();
        let options = FetchOptions {
            referrer_policy: Some(ReferrerPolicy::Origin),
            ..FetchOptions::default()
        };
        let cb_future =
            CallbackFuture::<Response<Json<Result<HttpBinHeaders, anyhow::Error>>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let _task = FetchService::fetch_with_options(request, options, callback);
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
