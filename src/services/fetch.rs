use serde::{Serialize, Deserialize};
use serde_json;
use stdweb::unstable::{TryInto, TryFrom};
use html::Context;
use super::Task;

pub struct FetchHandle {
}

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
        IN: Into<Option<String>>,
        OUT: From<String>,
        F: Fn(OUT) -> MSG + 'static;
}

impl<MSG: 'static> FetchService<MSG> for Context<MSG> {
    fn fetch<F, IN, OUT>(&mut self, method: Method, url: &str, data: IN, converter: F) -> FetchHandle
    where
        IN: Into<Option<String>>,
        OUT: From<String>,
        F: Fn(OUT) -> MSG + 'static
    {
        let mut tx = self.sender();
        let callback = move |s: String| {
            let out = OUT::from(s);
            let msg = converter(out);
            tx.send(msg);
        };
        let method = method.to_argument();
        let body = data.into();
        js! {
            var data = {
                method: @{method},
                body: @{body},
            };
            var request = new Request(@{url}, data);
            var callback = @{callback};
            fetch(request).then(function(response) {
                // TODO Do we need to use blob here?
                return response.text();
            }).then(function(data) {
                callback(data);
                callback.drop();
            });
        }
        FetchHandle {
        }
    }
}

impl Task for FetchHandle {
    fn cancel(&mut self) {
        // Fetch API doesn't support request cancelling
        // and we should use this workaround with a flag.
        // In fact, request not canceled, but callback won't be called.
        unimplemented!();
    }
}
