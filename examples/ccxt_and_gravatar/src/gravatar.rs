use anyhow::anyhow;
use serde::Deserialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Deserialize, Debug)]
pub struct Profile {
    entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    id: String,
    hash: String,
    request_hash: String,
    profile_url: String,
    preferred_username: String,
}

pub fn fetch_profile(hash: &str, callback: Callback<anyhow::Result<Profile>>) -> FetchTask {
    let url = format!("https://en.gravatar.com/{}.json", hash);
    let handler = move |response: Response<Json<anyhow::Result<Profile>>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data)
        } else {
            callback.emit(Err(anyhow!(
                "{}: error getting profile https://gravatar.com/",
                meta.status
            )))
        }
    };

    let request = Request::get(&url).body(Nothing).unwrap();
    FetchService::fetch(request, handler.into()).unwrap()
}
