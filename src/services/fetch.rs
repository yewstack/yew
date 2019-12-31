//! Service to send HTTP-request to a server.

use super::Task;
use crate::callback::Callback;
use crate::format::{Binary, Format, Text};
use failure::Fail;
use std::fmt;
#[cfg(feature = "std_web")]
#[allow(unused_imports)]
use stdweb::{_js_impl, js};
#[cfg(feature = "web_sys")]
pub use web_sys::{
    RequestCache as Cache, RequestCredentials as Credentials, RequestMode as Mode,
    RequestRedirect as Redirect,
};
#[cfg(feature = "web_sys")]
use ::{
    js_sys::{Array, Uint8Array},
    std::{
        rc::Rc,
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{self, Receiver},
        },
    },
    wasm_bindgen::{closure::Closure, JsValue},
    web_sys::{
        AbortController, Headers, Request as WebRequest, RequestInit, Response as WebResponse,
    },
};
#[cfg(feature = "std_web")]
use ::{
    serde::Serialize,
    std::collections::HashMap,
    stdweb::{
        serde::Serde,
        unstable::{TryFrom, TryInto},
        web::ArrayBuffer,
        JsSerialize, Value,
    },
};

pub use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};

#[cfg(feature = "web_sys")]
struct ArrayBuffer(Uint8Array);

#[cfg(feature = "web_sys")]
impl From<ArrayBuffer> for Vec<u8> {
    fn from(from: ArrayBuffer) -> Self {
        from.0.to_vec()
    }
}

#[cfg(feature = "web_sys")]
impl From<JsValue> for ArrayBuffer {
    fn from(from: JsValue) -> Self {
        ArrayBuffer(Uint8Array::from(from))
    }
}

/// Type to set cache for fetch.
#[cfg(feature = "std_web")]
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Cache {
    /// `default` value of cache.
    #[serde(rename = "default")]
    DefaultCache,
    /// `no-store` value of cache.
    NoStore,
    /// `reload` value of cache.
    Reload,
    /// `no-cache` value of cache.
    NoCache,
    /// `force-cache` value of cache
    ForceCache,
    /// `only-if-cached` value of cache
    OnlyIfCached,
}

/// Type to set credentials for fetch.
#[cfg(feature = "std_web")]
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Credentials {
    /// `omit` value of credentials.
    Omit,
    /// `include` value of credentials.
    Include,
    /// `same-origin` value of credentials.
    SameOrigin,
}

/// Type to set mode for fetch.
#[cfg(feature = "std_web")]
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// `same-origin` value of mode.
    SameOrigin,
    /// `no-cors` value of mode.
    NoCors,
    /// `cors` value of mode.
    Cors,
}

/// Type to set redirect behaviour for fetch.
#[cfg(feature = "std_web")]
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Redirect {
    /// `follow` value of redirect.
    Follow,
    /// `error` value of redirect.
    Error,
    /// `manual` value of redirect.
    Manual,
}

/// Init options for `fetch()` function call.
/// https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch
#[cfg_attr(feature = "std_web", derive(Serialize))]
#[derive(Default, Debug)]
pub struct FetchOptions {
    /// Cache of a fetch request.
    #[cfg_attr(feature = "std_web", serde(skip_serializing_if = "Option::is_none"))]
    pub cache: Option<Cache>,
    /// Credentials of a fetch request.
    #[cfg_attr(feature = "std_web", serde(skip_serializing_if = "Option::is_none"))]
    pub credentials: Option<Credentials>,
    /// Redirect behaviour of a fetch request.
    #[cfg_attr(feature = "std_web", serde(skip_serializing_if = "Option::is_none"))]
    pub redirect: Option<Redirect>,
    /// Request mode of a fetch request.
    #[cfg_attr(feature = "std_web", serde(skip_serializing_if = "Option::is_none"))]
    pub mode: Option<Mode>,
}

#[cfg(feature = "web_sys")]
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

#[cfg(feature = "web_sys")]
#[derive(Debug)]
struct Handle {
    active: Rc<AtomicBool>,
    callbacks: Receiver<Closure<dyn FnMut(JsValue)>>,
    abort_controller: Option<AbortController>,
}

/// A handle to control sent requests. Can be canceled with a `Task::cancel` call.
#[must_use]
pub struct FetchTask(
    #[cfg(feature = "std_web")] Option<Value>,
    #[cfg(feature = "web_sys")] Option<Handle>,
);

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
        #[cfg(feature = "std_web")]
        {
            fetch_impl::<IN, OUT, String, String>(false, request, None, callback)
        }
        #[cfg(feature = "web_sys")]
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
        #[cfg(feature = "std_web")]
        {
            fetch_impl::<IN, OUT, String, String>(false, request, Some(options), callback)
        }
        #[cfg(feature = "web_sys")]
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
        #[cfg(feature = "std_web")]
        {
            fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer>(true, request, None, callback)
        }
        #[cfg(feature = "web_sys")]
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
        #[cfg(feature = "std_web")]
        return fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer>(true, request, Some(options), callback);
        #[cfg(feature = "web_sys")]
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
    #[cfg(feature = "std_web")] T: JsSerialize,
    #[cfg(feature = "web_sys")] T,
    #[cfg(feature = "std_web")] X: TryFrom<Value> + Into<T>,
    #[cfg(feature = "web_sys")] X: Into<T>,
    #[cfg(feature = "web_sys")] IC: Fn(T) -> JsValue,
    #[cfg(feature = "web_sys")] FC: 'static + Fn(JsValue) -> X,
>(
    binary: bool,
    request: Request<IN>,
    options: Option<FetchOptions>,
    callback: Callback<Response<OUT>>,
    #[cfg(feature = "web_sys")] into_conversion: IC,
    #[cfg(feature = "web_sys")] from_conversion: FC,
) -> FetchTask
where
    IN: Into<Format<T>>,
    OUT: From<Format<T>>,
{
    // Consume request as parts and body.
    let (parts, body) = request.into_parts();

    // Map headers into a Js serializable HashMap.
    let header_map = parts.headers.iter().map(|(k, v)| {
        (
            k.as_str(),
            v.to_str()
                .unwrap_or_else(|_| panic!("Unparsable request header {}: {:?}", k.as_str(), v)),
        )
    });
    #[cfg(feature = "std_web")]
    let header_map: HashMap<&str, &str> = header_map.collect();
    #[cfg(feature = "web_sys")]
    let header_map = {
        let headers = Headers::new().unwrap();
        for (k, v) in header_map {
            headers.append(k, v).unwrap();
        }
        headers
    };
    // Formats URI.
    let uri = format!("{}", parts.uri);
    let method = parts.method.as_str();
    let body = body.into().ok();

    // Prepare the response callback.
    // Notice that the callback signature must match the call from the javascript
    // side. There is no static check at this point.
    let callback = move |#[cfg(feature = "std_web")] success: bool,
                         #[cfg(feature = "web_sys")] data: Option<X>,
                         status: u16,
                         #[cfg(feature = "std_web")] headers: HashMap<String, String>,
                         #[cfg(feature = "web_sys")] headers: Headers,
                         #[cfg(feature = "std_web")] data: X| {
        let mut response_builder = Response::builder().status(status);
        #[cfg(feature = "web_sys")]
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
        #[cfg(feature = "std_web")]
        let data = Some(data).filter(|_| success);
        let data = if let Some(data) = data {
            Ok(data.into())
        } else {
            Err(FetchError::FailedResponse.into())
        };
        let out = OUT::from(data);
        let response = response_builder.body(out).unwrap();
        callback.emit(response);
    };

    #[cfg(feature = "std_web")]
    #[allow(clippy::too_many_arguments)]
    let handle = js! {
        var body = @{body};
        if (@{binary} && body != null) {
            body = Uint8Array.from(body);
        }
        var data = {
            method: @{method},
            body: body,
            headers: @{header_map},
        };
        var request = new Request(@{uri}, data);
        var callback = @{callback};
        var abortController = AbortController ? new AbortController() : null;
        var handle = {
            active: true,
            callback,
            abortController,
        };
        var init = @{Serde(options)} || {};
        if (abortController && !("signal" in init)) {
            init.signal = abortController.signal;
        }
        fetch(request, init).then(function(response) {
            var promise = (@{binary}) ? response.arrayBuffer() : response.text();
            var status = response.status;
            var headers = {};
            response.headers.forEach(function(value, key) {
                headers[key] = value;
            });
            promise.then(function(data) {
                if (handle.active == true) {
                    handle.active = false;
                    callback(true, status, headers, data);
                    callback.drop();
                }
            }).catch(function(err) {
                if (handle.active == true) {
                    handle.active = false;
                    callback(false, status, headers, data);
                    callback.drop();
                }
            });
        }).catch(function(e) {
            if (handle.active == true) {
                var data = (@{binary}) ? new ArrayBuffer() : "";
                handle.active = false;
                callback(false, 408, {}, data);
                callback.drop();
            }
        });
        return handle;
    };
    #[cfg(feature = "web_sys")]
    let handle = {
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
                if active_clone.load(Ordering::SeqCst) {
                    active_clone.store(false, Ordering::SeqCst);
                    callback_clone(Some(data), status, headers_clone);
                }
            };
            let closure_then = Closure::once(closure_then);
            let closure_catch = move |_| {
                if active_outer_clone.load(Ordering::SeqCst) {
                    active_outer_clone.store(false, Ordering::SeqCst);
                    callback_outer_clone(None, status, headers);
                }
            };
            let closure_catch = Closure::once(closure_catch);
            promise.then(&closure_then).catch(&closure_catch);
            sender_clone.send(closure_then).unwrap();
            sender_clone.send(closure_catch).unwrap();
        };
        let closure_then = Closure::once(closure_then);
        let active_clone = Rc::clone(&active);
        let closure_catch = move |_| {
            if active_clone.load(Ordering::SeqCst) {
                active_clone.store(false, Ordering::SeqCst);
                callback(None, 408, Headers::new().unwrap());
            }
        };
        let closure_catch = Closure::wrap(Box::new(closure_catch) as Box<dyn FnMut(JsValue)>);
        let abort_controller = AbortController::new().ok();
        let mut init = options.map_or_else(RequestInit::new, Into::into);
        if let Some(abort_controller) = &abort_controller {
            init.signal(Some(&abort_controller.signal()));
        }
        let handle = Handle {
            active,
            callbacks: receiver,
            abort_controller,
        };
        web_sys::window()
            .unwrap()
            .fetch_with_request_and_init(&request, &init)
            .then(&closure_then)
            .catch(&closure_catch);
        sender.send(closure_then).unwrap();
        sender.send(closure_catch).unwrap();
        handle
    };
    FetchTask(Some(handle))
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        if let Some(ref task) = self.0 {
            #[cfg(feature = "std_web")]
            {
                let result = js! {
                    var the_task = @{task};
                    return the_task.active &&
                            (!the_task.abortController || !the_task.abortController.signal.aborted);
                };
                result.try_into().unwrap_or(false)
            }
            #[cfg(feature = "web_sys")]
            {
                task.active.load(Ordering::SeqCst)
                    && task
                        .abort_controller
                        .as_ref()
                        .map(|abort_controller| abort_controller.signal().aborted())
                        .filter(|value| *value)
                        .is_none()
            }
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
        #[cfg(feature = "std_web")]
        js! {  @(no_return)
            var handle = @{handle};
            handle.active = false;
            handle.callback.drop();
            if (handle.abortController) {
                handle.abortController.abort();
            }
        }
        #[cfg(feature = "web_sys")]
        {
            handle.active.store(false, Ordering::SeqCst);
            if let Some(abort_controller) = handle.abort_controller {
                abort_controller.abort();
            }
            handle.callbacks.try_iter().for_each(drop);
        }
    }
}

impl Drop for FetchTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
