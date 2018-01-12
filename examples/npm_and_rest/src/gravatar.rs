use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchHandle, Request, Response};
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
        let url = format!("https://gravatar.com/{}", hash);
        let handler = move |response: Response<Json<Result<Profile, ()>>>| {
            let (_, Json(data)) = response.into_parts();
            callback.emit(data)
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
}
