use crate::Msg::SetMarkdownFetchState;
use std::fmt::{Error, Formatter};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

/// Gets the markdown from Yew's readme.
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
async fn fetch_markdown() -> Result<String, FetchError> {
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

struct Model {
    markdown: FetchState<String>,
    link: ComponentLink<Self>,
}

enum Msg {
    SetMarkdownFetchState(FetchState<String>),
    GetMarkdown,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            markdown: FetchState::NotFetching,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetMarkdownFetchState(fetch_state) => {
                self.markdown = fetch_state;
                true
            }
            Msg::GetMarkdown => {
                let future = async {
                    match fetch_markdown().await {
                        Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
                    }
                };
                self.link.send_future(future);
                self.link
                    .send_self(SetMarkdownFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self) -> Html<Self> {
        match &self.markdown {
            FetchState::NotFetching => {
                html! {<button onclick=|_| Msg::GetMarkdown>{"Get Markdown"}</button>}
            }
            FetchState::Fetching => html! {"Fetching"},
            FetchState::Success(data) => html! {&data},
            FetchState::Failed(err) => html! {&err},
        }
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Model>();
}
