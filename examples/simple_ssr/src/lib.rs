use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

#[cfg(feature = "ssr")]
async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())?.unwrap();

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
