pub use anyhow::{anyhow, Error};
pub use http::{Request, Response};
pub use serde::{Deserialize, Serialize};
pub use std::rc::Rc;
pub use web_sys::RequestCredentials;
pub use yew::callback::Callback;
pub use yew::format::{Json, Nothing};
pub use yew::services::fetch::{FetchOptions, FetchService, FetchTask};

use super::store::TaskBundle;

pub struct Fetcher {
    // You can add things like a window service here to get your api keys from JS, or similar "global" request needs
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IP {
    pub ip: String,
}

#[derive(Debug)]
pub enum ResponseWrapper {
    IpResponse(Result<IP, Error>),
}

impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {}
    }
    // Init this way just in case we want to do anything with global configs
    pub fn init_fetcher() -> FetchService {
        FetchService::new()
    }

    pub fn build_url(url: &str, endpoint: &str) -> String {
        format!("{}{}", url, endpoint)
    }

    pub fn get_ip(&self, fetch_id: Rc<String>, callback: Callback<TaskBundle>) -> FetchTask {
        let uri = Self::build_url("https://api.ipify.org", "?format=json");
        let req = Request::get(uri).body(Nothing).unwrap();

        let handler = move |res: Response<Json<Result<IP, Error>>>| {
            let (meta, Json(data)) = res.into_parts();
            if meta.status.is_success() {
                let ew = ResponseWrapper::IpResponse(data);
                callback.emit((fetch_id.clone(), ew))
            } else {
                let err = Err(anyhow!("{} error getting user ip", meta.status));
                let ew = ResponseWrapper::IpResponse(err);
                callback.emit((fetch_id.clone(), ew))
            }
        };

        let mut fetcher = Fetcher::init_fetcher();
        fetcher.fetch(req, handler.into()).unwrap()
    }
}
