//! Service to send HTTP-request to a server.

use stdweb::Value;
use html::AppSender;
use format::{Storable, Restorable};
use super::Task;

/// A handle to control sent request. Could be canceled by `Task::cancel` call.
pub struct FetchHandle(Option<Value>);

/// A method of HTTP-request of [HTTP protocol](https://tools.ietf.org/html/rfc7231).
pub enum Method {
    /// `GET` method of a request.
    Get,
    /// `POST` method of a request.
    Post,
}

impl Method {
    /// Converts a method to `fetch` input argument.
    fn to_argument(&self) -> &'static str {
        match self {
            &Method::Get => "GET",
            &Method::Post => "POST",
        }
    }
}

/// A service to fetch resources.
pub struct FetchService {
}

impl FetchService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self { }
    }

    /// Sends request to a server. Could contains input data and
    /// needs a fuction to convert returned data to a loop's message.
    pub fn fetch<F, IN, OUT>(&mut self, method: Method, url: &str, data: IN, convert_and_send: F) -> FetchHandle
    where
        IN: Into<Storable>,
        OUT: From<Restorable>,
        F: Fn(OUT) + 'static
    {
        let callback = move |success: bool, s: String| {
            let data = if success { Ok(s) } else { Err(s) };
            let out = OUT::from(data);
            convert_and_send(out);
        };
        let method = method.to_argument();
        let body = data.into();
        let handle = js! {
            var data = {
                method: @{method},
                body: @{body},
            };
            var request = new Request(@{url}, data);
            var callback = @{callback};
            var handle = {
                interrupt: false,
                callback,
            };
            fetch(request).then(function(response) {
                if (response.ok) {
                    // Do we need to use blob here?
                    return response.text();
                } else {
                    throw new Error("Network response was not ok.");
                }
            }).then(function(data) {
                if (handle.interrupted != true) {
                    callback(true, data);
                    callback.drop();
                }
            }).catch(function(err) {
                if (handle.interrupted != true) {
                    callback(false, data);
                    callback.drop();
                }
            });
            return handle;
        };
        FetchHandle(Some(handle))
    }
}

impl Task for FetchHandle {
    fn cancel(&mut self) {
        // Fetch API doesn't support request cancelling
        // and we should use this workaround with a flag.
        // In fact, request not canceled, but callback won't be called.
        let handle = self.0.take().expect("tried to cancel request fetching twice");
        js! {
            var handle = @{handle};
            handle.interrupted = true;
            handle.callback.drop();
        }
    }
}
