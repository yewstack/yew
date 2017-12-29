use serde::{Serialize, Deserialize};
use serde_json;
use html::Context;
use super::{Task, Format};

pub struct Request {
    pub method: Method,
    pub in_format: Format,
    pub out_format: Format,
    pub url: String, // TODO Consider to use `url` crate
}

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
    fn fetch<F, IN, OUT>(&mut self, request: Request, data: Option<IN>, converter: F) -> FetchHandle
    where
        IN: Serialize,
        OUT: for <'de> Deserialize<'de>,
        F: Fn(Result<OUT, ()>) -> MSG + 'static;
}

impl<MSG: 'static> FetchService<MSG> for Context<MSG> {
    fn fetch<F, IN, OUT>(&mut self, request: Request, data: Option<IN>, converter: F) -> FetchHandle
    where
        IN: Serialize,
        OUT: for <'de> Deserialize<'de>,
        F: Fn(Result<OUT, ()>) -> MSG + 'static
    {
        let mut tx = self.sender();
        let out_format = request.out_format;
        let callback = move |s: String| {
            let out = {
                match out_format {
                    Format::Json => {
                        serde_json::from_str(&s).map_err(drop)
                    }
                }
            };
            let msg = converter(out);
            tx.send(msg);
        };
        let method = request.method.to_argument();
        let url = request.url;
        js! {
            var data = {
                method: @{method},
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
