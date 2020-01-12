//! `web-sys` implementation for the fetch service.

use crate::callback::Callback;
use crate::format::{Binary, Format, Text};
use crate::services::Task;
use failure::Fail;
use js_sys::{Array, Promise, Uint8Array};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{
    AbortController, Headers, Request as WebRequest, RequestInit, Response as WebResponse,
};
pub use web_sys::{
    RequestCache as Cache, RequestCredentials as Credentials, RequestMode as Mode,
    RequestRedirect as Redirect,
};

pub use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};

struct ArrayBuffer(Uint8Array);

impl From<ArrayBuffer> for Vec<u8> {
    fn from(from: ArrayBuffer) -> Self {
        from.0.to_vec()
    }
}

impl From<JsValue> for ArrayBuffer {
    fn from(from: JsValue) -> Self {
        ArrayBuffer(Uint8Array::new_with_byte_offset(&from, 0))
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

        init
    }
}

/// Represents errors of a fetch service.
#[derive(Debug, Fail)]
enum FetchError {
    #[fail(display = "failed response")]
    FailedResponse,
}

#[derive(Debug)]
struct Handle {
    active: Rc<AtomicBool>,
    callbacks: Receiver<Closure<dyn FnMut(JsValue)>>,
    abort_controller: Option<AbortController>,
    promise: Promise,
}

/// A handle to control sent requests. Can be canceled with a `Task::cancel` call.
#[must_use]
pub struct FetchTask(Option<Handle>);

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
    ///# struct Comp;
    ///# impl Component for Comp {
    ///#     type Message = Msg;type Properties = ();
    ///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
    ///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
    ///#     fn view(&self) -> Html {unimplemented!()}
    ///# }
    ///# enum Msg {
    ///#     Noop,
    ///#     Error
    ///# }
    ///# fn dont_execute() {
    ///# let link: ComponentLink<Comp> = unimplemented!();
    ///# let mut fetch_service: FetchService = FetchService::new();
    ///# let post_request: Request<Result<String, failure::Error>> = unimplemented!();
    /// let task = fetch_service.fetch(
    ///     post_request,
    ///     link.callback(|response: Response<Result<String, failure::Error>>| {
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
    ///# struct Comp;
    ///# impl Component for Comp {
    ///#     type Message = Msg;type Properties = ();
    ///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
    ///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
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
    /// let callback = link.callback(|response: Response<Json<Result<Data, failure::Error>>>| {
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
    ) -> FetchTask
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        fetch_impl::<IN, OUT, String, String, _, _>(
            false,
            request,
            None,
            callback,
            Into::into,
            |v| v.as_string().unwrap(),
        )
    }

    /// `fetch` with provided `FetchOptions` object.
    /// Use it if you need to send cookies with a request:
    /// ```
    ///# use yew::format::Nothing;
    ///# use yew::services::fetch::{self, FetchOptions, Credentials};
    ///# use yew::{Renderable, Html, Component, ComponentLink};
    ///# use yew::services::FetchService;
    ///# use http::Response;
    ///# struct Comp;
    ///# impl Component for Comp {
    ///#     type Message = Msg;
    ///#     type Properties = ();
    ///#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {unimplemented!()}
    ///#     fn update(&mut self, msg: Self::Message) -> bool {unimplemented!()}
    ///#     fn view(&self) -> Html {unimplemented!()}
    ///# }
    ///# pub enum Msg {}
    ///# fn dont_execute() {
    ///# let link: ComponentLink<Comp> = unimplemented!();
    ///# let callback = link.callback(|response: Response<Result<String, failure::Error>>| unimplemented!());
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
    ) -> FetchTask
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        fetch_impl::<IN, OUT, String, String, _, _>(
            false,
            request,
            Some(options),
            callback,
            Into::into,
            |v| v.as_string().unwrap(),
        )
    }

    /// Fetch the data in binary format.
    pub fn fetch_binary<IN, OUT: 'static>(
        &mut self,
        request: Request<IN>,
        callback: Callback<Response<OUT>>,
    ) -> FetchTask
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer, _, _>(
            true,
            request,
            None,
            callback,
            |v| Uint8Array::from(v.as_slice()).into(),
            From::from,
        )
    }

    /// Fetch the data in binary format.
    pub fn fetch_binary_with_options<IN, OUT: 'static>(
        &mut self,
        request: Request<IN>,
        options: FetchOptions,
        callback: Callback<Response<OUT>>,
    ) -> FetchTask
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer, _, _>(
            true,
            request,
            Some(options),
            callback,
            |v| Uint8Array::from(v.as_slice()).into(),
            From::from,
        )
    }
}

fn fetch_impl<
    IN,
    OUT: 'static,
    T,
    X: Into<T>,
    IC: Fn(T) -> JsValue,
    FC: 'static + Fn(JsValue) -> X,
>(
    binary: bool,
    request: Request<IN>,
    options: Option<FetchOptions>,
    callback: Callback<Response<OUT>>,
    into_conversion: IC,
    from_conversion: FC,
) -> FetchTask
where
    IN: Into<Format<T>>,
    OUT: From<Format<T>>,
{
    // Consume request as parts and body.
    let (parts, body) = request.into_parts();

    // Map headers into a Js serializable HashMap.
    let header_map = Headers::new().unwrap();
    for (k, v) in parts.headers.iter().map(|(k, v)| {
        (
            k.as_str(),
            v.to_str()
                .unwrap_or_else(|_| panic!("Unparsable request header {}: {:?}", k.as_str(), v)),
        )
    }) {
        header_map.append(k, v).unwrap();
    }
    // Formats URI.
    let uri = format!("{}", parts.uri);
    let method = parts.method.as_str();
    let body = body.into().ok();

    // Prepare the response callback.
    // Notice that the callback signature must match the call from the javascript
    // side. There is no static check at this point.
    let callback = move |data: Option<X>, status: u16, headers: Headers| {
        let mut response_builder = Response::builder().status(status);
        // convert `headers` to `Iterator<Item = (String, String)>`
        let headers = js_sys::try_iter(&headers)
            .unwrap()
            .unwrap()
            .map(Result::unwrap)
            .map(|entry| {
                let entry = Array::from(&entry);
                let key = entry.get(0);
                let value = entry.get(1);
                (key.as_string().unwrap(), value.as_string().unwrap())
            });
        for (key, value) in headers {
            response_builder = response_builder.header(key.as_str(), value.as_str());
        }

        // Deserialize and wrap response data into a Text object.
        let data = if let Some(data) = data {
            Ok(data.into())
        } else {
            Err(FetchError::FailedResponse.into())
        };
        let out = OUT::from(data);
        let response = response_builder.body(out).unwrap();
        callback.emit(response);
    };

    let mut data = RequestInit::new();
    data.method(method);
    data.body(body.map(into_conversion).as_ref());
    data.headers(&header_map);
    let request = WebRequest::new_with_str_and_init(&uri, &data).unwrap();
    let active = Rc::new(AtomicBool::new(true));
    let (sender, receiver) = mpsc::channel();
    let active_outer_clone = Rc::clone(&active);
    let callback_outer_clone = callback.clone();
    let sender_clone = sender.clone();
    let closure_then = move |response: JsValue| {
        let response = WebResponse::from(response);
        let promise = if binary {
            response.array_buffer()
        } else {
            response.text()
        }
        .unwrap();
        let status = response.status();
        let headers = response.headers();
        let active_clone = Rc::clone(&active_outer_clone);
        let callback_clone = callback_outer_clone.clone();
        let headers_clone = headers.clone();
        let closure_then = move |data: JsValue| {
            let data = from_conversion(data);
            if active_clone.compare_and_swap(true, false, Ordering::SeqCst) {
                callback_clone(Some(data), status, headers_clone);
            }
        };
        let closure_then = Closure::once(closure_then);
        let closure_catch = move |_| {
            if active_outer_clone.compare_and_swap(true, false, Ordering::SeqCst) {
                callback_outer_clone(None, status, headers);
            }
        };
        let closure_catch = Closure::once(closure_catch);
        #[allow(unused_must_use)]
        {
            promise.then(&closure_then).catch(&closure_catch);
        }
        sender_clone.send(closure_then).unwrap();
        sender_clone.send(closure_catch).unwrap();
    };
    let closure_then = Closure::once(closure_then);
    let active_clone = Rc::clone(&active);
    let closure_catch = move |_| {
        if active_clone.compare_and_swap(true, false, Ordering::SeqCst) {
            callback(None, 408, Headers::new().unwrap());
        }
    };
    let closure_catch = Closure::wrap(Box::new(closure_catch) as Box<dyn FnMut(JsValue)>);
    let abort_controller = AbortController::new().ok();
    let mut init = options.map_or_else(RequestInit::new, Into::into);
    if let Some(abort_controller) = &abort_controller {
        init.signal(Some(&abort_controller.signal()));
    }
    let promise = crate::global!(global, global.fetch_with_request_and_init(&request, &init))
        .then(&closure_then)
        .catch(&closure_catch);
    sender.send(closure_then).unwrap();
    sender.send(closure_catch).unwrap();

    FetchTask(Some(Handle {
        active,
        callbacks: receiver,
        abort_controller,
        promise,
    }))
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        if let Some(ref task) = self.0 {
            task.active.load(Ordering::SeqCst)
                && task
                    .abort_controller
                    .as_ref()
                    .map(|abort_controller| abort_controller.signal().aborted())
                    .filter(|value| *value)
                    .is_none()
        } else {
            false
        }
    }
    fn cancel(&mut self) {
        // Fetch API doesn't support request cancelling in all browsers
        // and we should use this workaround with a flag.
        // In that case, request not canceled, but callback won't be called.
        let handle = self
            .0
            .take()
            .expect("tried to cancel request fetching twice");

        thread_local! {
            static CATCH: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(|_| ()) as Box<dyn FnMut(JsValue)>);
        }
        handle.active.store(false, Ordering::SeqCst);
        #[allow(unused_must_use)]
        {
            CATCH.with(|c| handle.promise.catch(&c));
        }
        if let Some(abort_controller) = handle.abort_controller {
            abort_controller.abort();
        }
        handle.callbacks.try_iter().for_each(drop);
    }
}

impl Drop for FetchTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
