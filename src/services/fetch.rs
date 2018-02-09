//! Service to send HTTP-request to a server.

use std::collections::HashMap;

use stdweb::Value;
use stdweb::unstable::TryFrom;

use format::{Storable, Restorable};
use html::Callback;
use super::Task;

pub use http::{
    HeaderMap,
    Method,
    Request,
    Response,
    StatusCode,
    Uri
};

/// Represents errors of a fetch service.
#[derive(Debug, Fail)]
enum FetchError {
    #[fail(display = "failed response")]
    FailedResponse,
}

/// A handle to control sent requests. Can be canceled with a `Task::cancel` call.
pub struct FetchTask(Option<Value>);


/// A service to fetch resources.
pub struct FetchService {
}

impl FetchService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self { }
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



    pub fn fetch<IN, OUT: 'static>(&mut self, request: Request<IN>, callback: Callback<Response<OUT>>) -> FetchTask
    where
        IN: Into<Storable>,
        OUT: From<Restorable>,
    {
        // Consume request as parts and body.
        let (parts, body) = request.into_parts();

        // Map headers into a Js serializable HashMap.
        let header_map: HashMap<&str, &str> = parts.headers.iter().map(
            |(k, v)| (k.as_str(), v.to_str().expect(
                format!("Unparsable request header {}: {:?}", k.as_str(), v).as_str()
            ))
        ).collect();

        // Formats URI.
        let uri = format!("{}", parts.uri);

        // Prepare the response callback.
        // Notice that the callback signature must match the call from the javascript
        // side. There is no static check at this point.
        let callback = move |success: bool, response: Value, body: String| {
            let mut response_builder = Response::builder();

            // Deserialize response status.
            let status = u16::try_from(js!{
                return @{&response}.status;
            });

            if let Ok(code) = status {
                response_builder.status(code);
            }

            // Deserialize response headers.
            let headers: HashMap<String, String> = HashMap::try_from(js!{
                var map = {};
                @{&response}.headers.forEach(function(value, key) {
                    map[key] = value;
                });
                return map;
            }).unwrap_or(HashMap::new());

            for (key, values) in &headers {
                response_builder.header(key.as_str(), values.as_str());
            }

            // Deserialize and wrap response body into a Restorable object.
            let data = if success { Ok(body) } else { Err(FetchError::FailedResponse.into()) };
            let out = OUT::from(data);
            let response = response_builder.body(out).unwrap();
            callback.emit(response);
        };

        let handle = js! {
            var data = {
                method: @{parts.method.as_str()},
                body: @{body.into()},
                headers: @{header_map},
            };
            var request = new Request(@{uri}, data);
            var callback = @{callback};
            var handle = {
                interrupt: false,
                callback,
            };
            fetch(request).then(function(response) {
                response.text().then(function(data) {
                    if (handle.interrupted != true) {
                        callback(true, response, data);
                        callback.drop();
                    }
                }).catch(function(err) {
                    if (handle.interrupted != true) {
                        callback(false, response, data);
                        callback.drop();
                    }
                });
            });
            return handle;
        };
        FetchTask(Some(handle))
    }
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
    fn cancel(&mut self) {
        // Fetch API doesn't support request cancelling
        // and we should use this workaround with a flag.
        // In fact, request not canceled, but callback won't be called.
        let handle = self.0.take().expect("tried to cancel request fetching twice");
        js! {  @(no_return)
            var handle = @{handle};
            handle.interrupted = true;
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
