use failure::Error;
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::callback::Callback;

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

#[derive(Default)]
pub struct GravatarService {
    web: FetchService,
}

impl GravatarService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn profile(&mut self, hash: &str, callback: Callback<Result<Profile, Error>>) -> FetchTask {
        let url = format!("https://gravatar.com/{}", hash);
        let handler = move |response: Response<Json<Result<Profile, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!("{}: error getting profile https://gravatar.com/", meta.status)))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
}
