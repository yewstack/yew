use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchHandle, Method};
use yew::html::Callback;

#[derive(Deserialize, Debug)]
pub struct Profile {
    entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Entry {
    id: String,
    hash: String,
    request_hash: String,
    profile_url: String,
    preferred_username: String,
}

pub struct GravatarService {
    web: FetchService,
}

impl GravatarService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn profile(&mut self, hash: &str, callback: Callback<Result<Profile, ()>>) -> FetchHandle {
        let url = format!("https://www.gravatar.com/{}.json", hash);
        let handler = move |Json(data)| { callback(data) };
        self.web.fetch(Method::Get, &url, Nothing, Box::new(handler))
    }
}
