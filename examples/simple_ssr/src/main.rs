use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tokio::task::LocalSet;
use tokio::task::{spawn_blocking, spawn_local};
use uuid::Uuid;
use warp::Filter;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

pub struct UuidState {
    s: Suspension,
    value: Rc<RefCell<Option<Uuid>>>,
}

impl UuidState {
    fn new() -> Self {
        let (s, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Uuid>>> = Rc::default();

        {
            let value = value.clone();
            // we use tokio spawn local here.
            spawn_local(async move {
                let uuid = fetch_uuid().await;

                {
                    let mut value = value.borrow_mut();
                    *value = Some(uuid);
                }

                handle.resume();
            });
        }

        Self { s, value }
    }
}

impl PartialEq for UuidState {
    fn eq(&self, rhs: &Self) -> bool {
        self.s == rhs.s
    }
}

fn use_random_uuid() -> SuspensionResult<Uuid> {
    let s = use_state(UuidState::new);

    let result = match *s.value.borrow() {
        Some(ref m) => Ok(*m),
        None => Err(s.s.clone()),
    };

    result
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_random_uuid()?;

    Ok(html! {
        <div>{"Random UUID: "}{uuid}</div>
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

async fn render() -> String {
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

    format!(
        r#"<!DOCTYPE HTML>
<html>
    <head>
        <title>Yew SSR Example</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content
    )
}

#[tokio::main]
async fn main() {
    let routes = warp::any().then(|| async move { warp::reply::html(render().await) });

    println!("You can view the website at: http://localhost:8080/");

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
