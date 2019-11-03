use crate::Msg::FutureFinished;
use std::fmt::{Error, Formatter};
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    future_data: Option<String>,
}

enum Msg {
    FutureFinished(String),
}

/// An error that can never happen (because an instance of this can not be created).
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

/// Gets the markdown from yew's readme.
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
async fn get_markdown() -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://raw.githubusercontent.com/yewstack/yew/master/README.md",
        &opts,
    )?;

    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let future = async {
            let md = get_markdown().await;
            md.map(Msg::FutureFinished)
        };
        link.send_future(future);
        Model { future_data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FutureFinished(resolved_future) => {
                self.future_data = Some(resolved_future);
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            if let Some(future_data) = &self.future_data {
                html! {
                    &future_data
                }
            } else {
                html! {
                    "no future yet"
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Model>();
}
