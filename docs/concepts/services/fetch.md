---
id: fetch
title: The fetch service
---
## Introduction
The fetch module can be used to make HTTP requests to a server. This enables applications to
communicate with external services and persist data.

:::note
You might find it helpful to read the [documentation about the format module](format.md) before 
reading this page.
:::

## Making requests
### Building requests
Yew has a `Request` request (which comes from the `http` crate) that is used to 'build' requests 
before they can be dispatched to a server. The type intended to be used as the request body must
have an implementation of `Into<Text>`.
```rust
use yew::services::fetch::Request;
use yew::format::Nothing;
let get_request = Request::get("https://example.com/api/v1/get/something").body(Nothing).expect("Could not build that request");
```
```rust
use yew::services::fetch::Request;
use yew::format::Json;
use serde_json::json;
let post_request = Request::post("https://example.com/api/v1/post/something").header("Content-Type", "application/json").body(Json(&json!({"key": "value"}))).expect("Could not build that request.");
```

:::note
Note that the `Json` struct takes references to values instead of values (i.e. `&T` not `T`).
:::

### Dispatching requests
The `FetchService` provides a binding to the browser's `fetch` API. Requests can be sent using 
either `FetchService::fetch` or `FetchService::fetch_with_options` (`fetch_with_options` should be 
used where cookies need to be sent in a request).

`FetchService::fetch` accepts two parameters: a `Request` object and a `Callback`. The `Callback` is
called once the request has completed allowing you to handle the data returned from the request.
The callback you pass needs to take a single parameter of type `Response<T>` where `T` is the body
of the request. Yew needs to be able to parse the response body to create an instance of the data
type `T` so `T` needs to implement `From<Text>`.

:::note
Because something could go wrong trying to deserialize data `From<Text>` and `From<Binary>` are only 
implemented for `FormatDataType<Result<T, ::anyhow::Error>>` (not `FormatDataType<T>`) where 
`FormatDataType` is used as a placeholder for any type in the format module (e.g. `Json`).

This means that your callbacks should look like 
```rust
self.link.callback(|response: Json<Result<ResponseType, anyhow::Error>>|)
```
instead of 
```rust
self.link.callback(|response: Json<ResponseType>|)
```
:::

:::danger
It's important that the `FetchTask` returned is kept alive until the request has finished – i.e. it 
should not be dropped until the request has finished and a response has been obtained. If the 
`FetchTask` is dropped before the request has finished then the request will be cancelled.
:::

:::important info
If you keep getting an error saying that "the operation was aborted" or "Error 408" this might be 
because the [CORS headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) of the website 
you are trying to access are not set correctly. Please see the linked to article from Mozilla about
how to resolve these errors.
:::

An illustrated example of how to fetch data from an API giving information about the ISS's 
(International Space Station) location is given below.

```rust
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask};
// serde is a library for serializing and deserializing data
use serde::Deserialize;

#[derive(Debug, Clone)]
struct FetchServiceExample {
    ft: Option<FetchTask>,
    iss: Option<ISS>,
    link: ComponentLink<Self>,
    error: Option<String>,
    fetching: bool
}

/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl FetchServiceExample {
    fn view_iss_location(&self) -> Html {
        match self.iss {
            Some(space_station) => html! {
                <p>{"The ISS is at:"}</p>
                <p>{format!("Latitude: {}", space_station.iss_location.latitude)}</p>
                <p>{format!("Longitude: {}", space_station.iss_location.longitude)}</p>
            }
            None => html! {
                <button onclick=self.link.callback(|_| {Self::Message::GetLocation})>
                    {"Where is the ISS?"}
                </button>
           }
        }
    }
    fn is_fetching(&self) -> Html {
        if self.fetching {
            html! {<p>{"Fetching data..."}</p>}
        } else {
            html! {<p></p>}
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ISSPosition {
    latitude: String,
    longitude: String
}

#[derive(Deserialize, Debug, Clone)]
struct ISS {
    message: String,
    timestamp: i32,
    iss_position: ISSPosition,
}

#[derive(Debug, Clone)]
enum Msg {
    GetLocation,
    Noop,
    ReceiveLocation(ISS),
    FetchError(String)
}

impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ft: None,
            iss: None,
            link,
            error: None,
            fetching: false
        }
    }
    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            GetLocation => {
                // 1. build the request
                let request = Request::get("http://api.open-notify.org/iss-now.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback = self.link.callback(|response: Response<Json<Result<ISS, anyhow::Error>>>| {
                    // split up the response into data about the request's status and the returned
                    // data from the request
                    let (meta, Json(Ok(data))) = response.into_parts(); 
                    if meta.status.is_success() && data.message == "success" {
                        Self::Message::ReceiveLocation(match data {
                            Ok(d) => d,
                            Err(e) => Self::Message::FetchError(e.to_string())
                        })
                    } else {
                        Self::Message::FetchError(data.message)
                    }
                });
                // 3. pass the request and callback to the fetch service 
                FetchService::fetch(request, callback);
                self.fetching = true;
                // we want to redraw so that the page displays a 'fetching...' message to the user so return 'true'
                true
            }
            ReceiveLocation(location) => {
                self.iss = Some(location);
                self.fetching = false;
                // we want to redraw so that the page displays the location of the ISS instead of 'fetching...'
                true
            }
            FetchError(error) => {
                self.error = error;
                // redraw to show the error
                true
            }
            _ => false
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                {self.is_fetching()}
                {self.view_iss_location()}
                {
                    if let Some(error) = self.error {
                        <p>{error.clone()}</p>
                    }
                }
            </>
        }
    }
}
```

## Debugging the `FetchService`

Most browsers' developer tools have a "network" pane which can be used to inspect and view requests 
browsers have made, including data such as request headers and the contents of responses. This can 
be a useful way to gain an insight into what requests are being made, the data being sent to the 
server as well as the return data from the server.

The Rust Wasm Book also contains [useful debugging tips](https://rustwasm.github.io/book/reference/debugging.html)
for Wasm applications.

## Further reading
* [The API documentation](https://docs.rs/yew/0.14.3/yew/services/fetch/index.html)
* The [dashboard](https://github.com/yewstack/yew/tree/master/examples/dashboard) and 
[npm_and_rest](https://github.com/yewstack/yew/tree/master/examples/web_sys/npm_and_rest) examples.
* [The Rust Wasm Book on debugging Wasm applications](https://rustwasm.github.io/book/reference/debugging.html)