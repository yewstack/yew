//! Service to send HTTP-request to a server.

use std::collections::HashMap;

use stdweb::{Value, JsSerialize};
use stdweb::web::ArrayBuffer;
use stdweb::unstable::{TryInto, TryFrom};

use super::Task;
use format::{Format, Text, Binary};
use callback::Callback;

pub use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};

/// Represents errors of a fetch service.
#[derive(Debug, Fail)]
enum FetchError {
    #[fail(display = "failed response")]
    FailedResponse,
}

/// A handle to control sent requests. Can be canceled with a `Task::cancel` call.
#[derive(Serialize, Deserialize)]
pub struct FetchTask(Option<Value>);

/// A service to fetch resources.
#[derive(Default)]
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
    /// ```rust
    ///    let post_request = Request::post("https://my.api/v1/resource")
    ///            .header("Content-Type", "application/json")
    ///            .body(Json(&json!({"foo": "bar"})))
    ///            .expect("Failed to build request.");
    ///
    ///    let get_request = Request::get("https://my.api/v1/resource")
    ///            .body(Nothing)
    ///            .expect("Failed to build request.");
    /// ```
    ///
    /// The callback function can build a loop message by passing or analizing the
    /// response body and metadata.
    ///
    /// ```rust
    ///     context.web.fetch(
    ///         post_request,
    ///         |response| {
    ///             if response.status().is_success() {
    ///                 Msg::Noop
    ///             } else {
    ///                 Msg::Error
    ///             }
    ///         }
    /// ```
    ///
    /// One can also simply consume and pass the response or body object into
    /// the message.
    ///
    /// ```rust
    ///     context.web.fetch(
    ///         get_request,
    ///         |response| {
    ///             let (meta, Json(body)) = response.into_parts();
    ///             if meta.status.is_success() {
    ///                 Msg::FetchResourceComplete(body)
    ///             } else {
    ///                 Msg::FetchResourceFailed
    ///             }
    ///         }
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
        fetch_impl::<IN, OUT, String, String>(false, request, callback)
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
        fetch_impl::<IN, OUT, Vec<u8>, ArrayBuffer>(true, request, callback)
    }
}

fn fetch_impl<IN, OUT: 'static, T, X>(
    binary: bool,
    request: Request<IN>,
    callback: Callback<Response<OUT>>,
) -> FetchTask
where
    IN: Into<Format<T>>,
    OUT: From<Format<T>>,
    T: JsSerialize,
    X: TryFrom<Value> + Into<T>,
{
    // Consume request as parts and body.
    let (parts, body) = request.into_parts();

    // Map headers into a Js serializable HashMap.
    let header_map: HashMap<&str, &str> = parts
        .headers
        .iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                v.to_str().expect(
                    format!("Unparsable request header {}: {:?}", k.as_str(), v).as_str(),
                ),
            )
        })
        .collect();

    // Formats URI.
    let uri = format!("{}", parts.uri);
    let method = parts.method.as_str();
    let body = body.into().ok();

    // Prepare the response callback.
    // Notice that the callback signature must match the call from the javascript
    // side. There is no static check at this point.
    let callback = move |success: bool, status: u16, headers: HashMap<String, String>, data: X| {
        let mut response_builder = Response::builder();
        response_builder.status(status);
        for (key, values) in &headers {
            response_builder.header(key.as_str(), values.as_str());
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
        var handle = {
            active: true,
            callback,
        };
        fetch(request).then(function(response) {
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
        });
        return handle;
    };
    FetchTask(Some(handle))
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        if let Some(ref task) = self.0 {
            let result = js! {
                var the_task = @{task};
                return the_task.active;
            };
            result.try_into().unwrap_or(false)
        } else {
            false
        }
    }
    fn cancel(&mut self) {
        // Fetch API doesn't support request cancelling
        // and we should use this workaround with a flag.
        // In fact, request not canceled, but callback won't be called.
        let handle = self.0
            .take()
            .expect("tried to cancel request fetching twice");
        js! {  @(no_return)
            var handle = @{handle};
            handle.active = false;
            handle.callback.drop();
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
