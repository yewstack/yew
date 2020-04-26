//! `web-sys` implementation for the fetch service.

use super::Referrer;
use crate::callback::Callback;
use crate::format::{Binary, Format, Text};
use crate::services::Task;
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

pub use web_sys::{
    RequestCache as Cache, RequestCredentials as Credentials, RequestMode as Mode,
    RequestRedirect as Redirect, Window, WorkerGlobalScope,
};

pub use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};

trait JsInterop: Sized {
    fn from_js(js_value: JsValue) -> Result<Self, FetchError>;
    fn to_js(self) -> JsValue;
}

impl JsInterop for Vec<u8> {
    fn from_js(js_value: JsValue) -> Result<Self, FetchError> {
        Ok(Uint8Array::new(&js_value).to_vec())
    }

    fn to_js(self) -> JsValue {
        Uint8Array::from(self.as_slice()).into()
    }
}

impl JsInterop for String {
    fn from_js(js_value: JsValue) -> Result<Self, FetchError> {
        js_value.as_string().ok_or(FetchError::InternalError)
    }

    fn to_js(self) -> JsValue {
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
#[must_use]
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
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Sends a request to a remote server given a Request object and a callback
    /// fuction to convert a Response object into a loop's message.
    ///
    /// You may use a Request builder to build your request declaratively as on the
    /// following examples:
    ///
    /// ```
    ///# use yew::format::{Nothing, Json};
    ///# use yew::services::fetch::Request;
    ///# use serde_json::json;
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
    /// The callback function can build a loop message by passing or analizing the
    /// response body and metadata.
    ///
    /// ```
    ///# use yew::{Component, ComponentLink, Html, Renderable};
    ///# use yew::services::FetchService;
    ///# use yew::services::fetch::{Response, Request};
    ///# use anyhow::Error;
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
    ///# let mut fetch_service: FetchService = FetchService::new();
    ///# let post_request: Request<Result<String, Error>> = unimplemented!();
    /// let task = fetch_service.fetch(
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
    /// and be a specific serialized data type. If the mesage isn't Json, or isn't the specified
    /// data type, then you will get a message indicating failure.
    ///
    /// ```
    ///# use yew::format::{Json, Nothing, Format};
    ///# use yew::services::FetchService;
    ///# use http::Request;
    ///# use yew::services::fetch::Response;
    ///# use yew::{Component, ComponentLink, Renderable, Html};
    ///# use serde_derive::Deserialize;
    ///# use anyhow::Error;
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
    /// let task = FetchService::new().fetch(get_request, callback);
    ///# }
    /// ```
    ///
    pub fn fetch<IN, OUT: 'static>(
        &mut self,
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
    ///# use yew::format::Nothing;
    ///# use yew::services::fetch::{self, FetchOptions, Credentials};
    ///# use yew::{Renderable, Html, Component, ComponentLink};
    ///# use yew::services::FetchService;
    ///# use http::Response;
    ///# use anyhow::Error;
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
    /// let task = FetchService::new().fetch_with_options(request, options, callback);
    ///# }
    /// ```
    pub fn fetch_with_options<IN, OUT: 'static>(
        &mut self,
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
        &mut self,
        request: Request<IN>,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, Error>
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>>(true, request, None, callback)
    }

    /// Fetch the data in binary format.
    pub fn fetch_binary_with_options<IN, OUT: 'static>(
        &mut self,
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
        Ok(b) => b.to_js(),
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
            panic!("Only supported in a browser or web worker");
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
