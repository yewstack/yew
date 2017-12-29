use stdweb::Value;
use html::Context;
use services::format::{Storable, Restorable};
use super::Task;

pub struct FetchHandle(Option<Value>);

pub enum Method {
    Get,
    Post,
}

impl Method {
    fn to_argument(&self) -> &'static str {
        match self {
            &Method::Get => "GET",
            &Method::Post => "POST",
        }
    }
}

pub trait FetchService<MSG> {
    fn fetch<F, IN, OUT>(&mut self, method: Method, url: &str, data: IN, converter: F) -> FetchHandle
    where
        IN: Into<Storable>,
        OUT: From<Restorable>,
        F: Fn(OUT) -> MSG + 'static;
}

impl<MSG: 'static> FetchService<MSG> for Context<MSG> {
    fn fetch<F, IN, OUT>(&mut self, method: Method, url: &str, data: IN, converter: F) -> FetchHandle
    where
        IN: Into<Storable>,
        OUT: From<Restorable>,
        F: Fn(OUT) -> MSG + 'static
    {
        let mut tx = self.sender();
        let callback = move |success: bool, s: String| {
            let data = if success { Ok(s) } else { Err(s) };
            let out = OUT::from(data);
            let msg = converter(out);
            tx.send(msg);
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
