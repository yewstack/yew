use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[cfg(not(target_arch = "wasm32"))]
use tokio::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

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

#[hook]
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
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
