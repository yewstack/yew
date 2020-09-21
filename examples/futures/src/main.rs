use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::future::LinkFuture;

mod markdown;

const MARKDOWN_URL: &str = "https://raw.githubusercontent.com/yewstack/yew/master/README.md";
const INCORRECT_URL: &str = "https://raw.githubusercontent.com/yewstack/yew/master/README.md.404";

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

/// Fetches markdown from Yew's README.md.
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
async fn fetch_markdown(url: &'static str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

enum Msg {
    SetMarkdownFetchState(FetchState<String>),
    GetMarkdown,
    GetError,
}
struct Model {
    markdown: FetchState<String>,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            markdown: FetchState::NotFetching,
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetMarkdownFetchState(fetch_state) => {
                self.markdown = fetch_state;
                true
            }
            Msg::GetMarkdown => {
                self.link.send_future(async {
                    match fetch_markdown(MARKDOWN_URL).await {
                        Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
                    }
                });
                self.link
                    .send_message(Msg::SetMarkdownFetchState(FetchState::Fetching));
                false
            }
            Msg::GetError => {
                self.link.send_future(async {
                    match fetch_markdown(INCORRECT_URL).await {
                        Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
                    }
                });
                self.link
                    .send_message(Msg::SetMarkdownFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self) -> Html {
        match &self.markdown {
            FetchState::NotFetching => html! {
                <>
                    <button onclick=self.link.callback(|_| Msg::GetMarkdown)>
                        { "Get Markdown" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::GetError)>
                        { "Get using incorrect URL" }
                    </button>
                </>
            },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(data) => html! { markdown::render_markdown(&data) },
            FetchState::Failed(err) => html! { err },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
