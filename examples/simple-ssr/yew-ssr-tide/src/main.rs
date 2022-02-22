use std::cell::RefCell;
use std::rc::Rc;

use async_std::task;
use serde::{Deserialize, Serialize};
use tide::{http::mime, Request, Response, StatusCode};

use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Serialize, Deserialize, Clone)]
struct UserResponse {
    login: String,
    name: String,
    blog: String,
    location: String,
}

async fn fetch_user() -> UserResponse {
    // surf works for both non-wasm and wasm targets.
    let mut resp = surf::get("https://api.github.com/users/zzy")
        .header("User-Agent", "request")
        .await
        .unwrap();
    println!("Status: {:#?}", resp.status());

    let user_resp: UserResponse = resp.body_json().await.unwrap();

    user_resp
}

pub struct UserState {
    susp: Suspension,
    value: Rc<RefCell<Option<UserResponse>>>,
}

impl UserState {
    fn new() -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<UserResponse>>> = Rc::default();

        {
            let value = value.clone();
            // we use async-std spawn local here.
            task::spawn_local(async move {
                let user = fetch_user().await;
                {
                    let mut value = value.borrow_mut();
                    *value = Some(user);
                }

                handle.resume();
            });
        }

        Self { susp, value }
    }
}

#[hook]
fn use_user() -> SuspensionResult<UserResponse> {
    let user_state = use_state(UserState::new);

    let result = match *user_state.value.borrow() {
        Some(ref user) => Ok(user.clone()),
        None => Err(user_state.susp.clone()),
    };

    result
}

#[function_component]
fn Content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {
        <div>
            <div>{"Login name: "}{ user.login }</div>
            <div>{"User name: "}{ user.name }</div>
            <div>{"Blog: "}{ user.blog }</div>
            <div>{"Location: "}{ user.location }</div>
        </div>
    })
}

#[function_component]
fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}

async fn render(_: Request<()>) -> tide::Result {
    let content = task::spawn_blocking(move || {
        task::block_on(async {
            let renderer = yew::ServerRenderer::<App>::new();

            renderer.render().await
        })
    })
    .await;

    let resp_content = format!(
        r#"<!DOCTYPE HTML>
            <html>
                <head>
                    <title>yew-ssr with tide example</title>
                </head>
                <body>
                    <h1>yew-ssr with tide example</h1>
                    {}
                </body>
            </html>
            "#,
        content
    );

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(resp_content);
    resp.set_content_type(mime::HTML);

    Ok(resp.into())
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut server = tide::new();
    server.at("/").get(render);
    println!("You can view the website at: http://localhost:8080");
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
