use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

#[cfg(feature = "ssr")]
async fn fetch_uuid() -> Uuid {
    let client = reqwest::Client::builder()
        .user_agent("yew-simple-ssr-example")
        .build()
        .unwrap();
    let resp = client
        .get("https://httpbingo.org/uuid")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_prepared_state!((), async move |_| -> Uuid { fetch_uuid().await })?.unwrap();

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
