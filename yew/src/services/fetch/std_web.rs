//! `stdweb` implementation for the fetch service.

use super::Referrer;
use crate::callback::Callback;
use crate::format::{Binary, Format, Text};
use crate::services::Task;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use stdweb::serde::Serde;
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::web::error::Error;
use stdweb::web::ArrayBuffer;
use stdweb::{JsSerialize, Value};
#[allow(unused_imports)]
use stdweb::{_js_impl, js};
use thiserror::Error;

pub use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};

/// Type to set cache for fetch.
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

impl Serialize for Referrer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Referrer::SameOriginUrl(ref s) => serializer.serialize_str(s),
            Referrer::AboutClient => {
                serializer.serialize_unit_variant("Referrer", 0, "about:client")
            }
            Referrer::Empty => serializer.serialize_unit_variant("Referrer", 1, ""),
        }
    }
}

/// Type to set referrer policy for fetch.
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ReferrerPolicy {
    /// `no-referrer` value of referrerPolicy.
    NoReferrer,
    /// `no-referrer-when-downgrade` value of referrerPolicy.
    NoReferrerWhenDowngrade,
    /// `same-origin` value of referrerPolicy.
    SameOrigin,
    /// `origin` value of referrerPolicy.
    Origin,
    /// `strict-origin` value of referrerPolicy.
    StrictOrigin,
    /// `origin-when-cross-origin` value of referrerPolicy.
    OriginWhenCrossOrigin,
    /// `strict-origin-when-cross-origin` value of referrerPolicy.
    StrictOriginWhenCrossOrigin,
    /// `unsafe-url` value of referrerPolicy.
    UnsafeUrl,
}

/// Init options for `fetch()` function call.
/// https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch
#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetchOptions {
    /// Cache of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
    /// Credentials of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    /// Redirect behaviour of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Redirect>,
    /// Request mode of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Referrer of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<Referrer>,
    /// Referrer policy of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_policy: Option<ReferrerPolicy>,
    /// Integrity of a fetch request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity: Option<String>,
}

/// Represents errors of a fetch service.
#[derive(Debug, Error)]
enum FetchError {
    #[error("failed response")]
    FailedResponse,
}

/// A handle to control sent requests. Can be canceled with a `Task::cancel` call.
#[must_use]
pub struct FetchTask(Option<Value>);

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
    /// The callback function can build a loop message by passing or analyzing the
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
    ///# let post_request: Request<Result<String, anyhow::Error>> = unimplemented!();
    /// let task = fetch_service.fetch(
    ///     post_request,
    ///     link.callback(|response: Response<Result<String, anyhow::Error>>| {
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
    /// let callback = link.callback(|response: Response<Json<Result<Data, anyhow::Error>>>| {
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
    ) -> Result<FetchTask, &str>
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        fetch_impl::<IN, OUT, String, String>(false, request, None, callback)
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
    ///#     fn change(&mut self, _: Self::Properties) -> bool {unimplemented!()}
    ///#     fn view(&self) -> Html {unimplemented!()}
    ///# }
    ///# pub enum Msg { }
    ///# fn dont_execute() {
    ///# let link: ComponentLink<Comp> = unimplemented!();
    ///# let callback = link.callback(|response: Response<Result<String, anyhow::Error>>| -> Msg { unimplemented!() });
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
    ) -> Result<FetchTask, &str>
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        fetch_impl::<IN, OUT, String, String>(false, request, Some(options), callback)
    }

    /// Fetch the data in binary format.
    pub fn fetch_binary<IN, OUT: 'static>(
        &mut self,
        request: Request<IN>,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, &str>
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer>(true, request, None, callback)
    }

    /// Fetch the data in binary format.
    pub fn fetch_binary_with_options<IN, OUT: 'static>(
        &mut self,
        request: Request<IN>,
        options: FetchOptions,
        callback: Callback<Response<OUT>>,
    ) -> Result<FetchTask, &str>
    where
        IN: Into<Binary>,
        OUT: From<Binary>,
    {
        fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer>(true, request, Some(options), callback)
    }
}

fn fetch_impl<IN, OUT: 'static, T, X>(
    binary: bool,
    request: Request<IN>,
    options: Option<FetchOptions>,
    callback: Callback<Response<OUT>>,
) -> Result<FetchTask, &'static str>
where
    IN: Into<Format<T>>,
    OUT: From<Format<T>>,
    T: JsSerialize,
    X: TryFrom<Value> + Into<T>,
{
    // Consume request as parts and body.
    let (parts, body) = request.into_parts();

    // Map headers into a Js `Header` to make sure it's supported.
    let header_list = parts
        .headers
        .iter()
        .map(|(k, v)| {
            Ok((
                k.as_str(),
                v.to_str().map_err(|_| "Unparsable request header")?,
            ))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;
    let header_map = js! {
        try {
            return new Headers(@{header_list});
        } catch(error) {
            return error;
        }
    };
    if Error::try_from(js!( return @{header_map.as_ref()}; )).is_ok() {
        return Err("couldn't build headers");
    }

    // Formats URI.
    let uri = parts.uri.to_string();
    let method = parts.method.as_str();
    let body = body.into().ok();

    // Prepare the response callback.
    // Notice that the callback signature must match the call from the javascript
    // side. There is no static check at this point.
    let callback = move |success: bool, status: u16, headers: HashMap<String, String>, data: X| {
        let mut response_builder = Response::builder();

        if let Ok(status) = StatusCode::from_u16(status) {
            response_builder = response_builder.status(status);
        }

        for (key, values) in headers {
            response_builder = response_builder.header(key.as_str(), values.as_str());
        }

        // Deserialize and wrap response data into a Text object.
        let data = if success {
            Ok(data.into())
        } else {
            Err(FetchError::FailedResponse.into())
        };
        let out = OUT::from(data);
        let response = response_builder.body(out).unwrap();
        callback.emit(response);
    };

    #[allow(clippy::too_many_arguments)]
    let handle = js! {
        var body = @{body};
        if (@{binary} && body != null) {
            body = Uint8Array.from(body);
        }
        var callback = @{callback};
        var abortController = AbortController ? new AbortController() : null;
        var handle = {
            active: true,
            callback,
            abortController,
        };
        var init = {
            method: @{method},
            body: body,
            headers: @{header_map},
        };
        var opts = @{Serde(options)} || {};
        for (var attrname in opts) {
            init[attrname] = opts[attrname];
        }
        if (abortController && !("signal" in init)) {
            init.signal = abortController.signal;
        }
        fetch(@{uri}, init).then(function(response) {
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
    Ok(FetchTask(Some(handle)))
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        if let Some(ref task) = self.0 {
            let result = js! {
                var the_task = @{task};
                return the_task.active &&
                        (!the_task.abortController || !the_task.abortController.signal.aborted);
            };
            result.try_into().unwrap_or(false)
        } else {
            false
        }
    }
}

impl Drop for FetchTask {
    fn drop(&mut self) {
        if self.is_active() {
            // Fetch API doesn't support request cancelling in all browsers
            // and we should use this workaround with a flag.
            // In that case, request not canceled, but callback won't be called.
            let handle = self
                .0
                .take()
                .expect("tried to cancel request fetching twice");
            js! {  @(no_return)
                var handle = @{handle};
                handle.active = false;
                handle.callback.drop();
                if (handle.abortController) {
                    handle.abortController.abort();
                }
            }
        }
    }
}
