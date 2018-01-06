use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchHandle, Method};
use yew::html::AppSender;

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

pub struct GravatarService<MSG> {
    web: FetchService<MSG>,
}

impl<MSG: 'static> GravatarService<MSG> {
    pub fn new(sender: AppSender<MSG>) -> Self {
        Self {
            web: FetchService::new(sender),
        }
    }

    pub fn profile<F>(&mut self, hash: &str, listener: F) -> FetchHandle
    where
        F: Fn(Result<Profile, ()>) -> MSG + 'static
    {
        let url = format!("https://www.gravatar.com/{}.json", hash);
        let handler = move |Json(data)| { listener(data) };
        self.web.fetch(Method::Get, &url, Nothing, handler)
    }
}
