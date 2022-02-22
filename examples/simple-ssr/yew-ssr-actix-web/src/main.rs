use std::cell::RefCell;
use std::rc::Rc;

use actix_web::{get, App as ActixApp, Error, HttpResponse, HttpServer};
use tokio::task::LocalSet;
use tokio::task::{spawn_blocking, spawn_local};

use serde::{Deserialize, Serialize};

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
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::Client::new()
        .get("https://api.github.com/users/zzy")
        .header("User-Agent", "request")
        .send()
        .await
        .unwrap();
    println!("Status: {}", resp.status());
    
    let user_resp = resp.json::<UserResponse>().await.unwrap();

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
            // we use tokio spawn local here.
            spawn_local(async move {
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

#[get("/")]
async fn render() -> Result<HttpResponse, Error> {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            let renderer = yew::ServerRenderer::<App>::new();

            renderer.render().await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            r#"<!DOCTYPE HTML>
                <html>
                    <head>
                        <title>yew-ssr with actix-web example</title>
                    </head>
                    <body>
                        <h1>yew-ssr with actix-web example</h1>
                        {}
                    </body>
                </html>
            "#,
            content
        )))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| ActixApp::new().service(render));
    println!("You can view the website at: http://localhost:8080/");
    server.bind(("127.0.0.1", 8080))?.run().await
}
