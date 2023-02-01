use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::de::DeserializeOwned;

pub fn github_fetch<T: DeserializeOwned>(url: &str, token: Option<String>) -> Result<T> {
    thread::sleep(Duration::from_secs(1));
    let mut optional_headers = HeaderMap::new();
    if let Some(token) = token {
        optional_headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    }

    let request_client = Client::new();
    let resp = request_client
        .get(url)
        .header(USER_AGENT, "reqwest")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .headers(optional_headers)
        .send()?;
    let status = resp.status();
    if !status.is_success() {
        if let Some(remaining) = resp.headers().get("x-ratelimit-remaining") {
            if remaining == "0" {
                bail!("GitHub API limit reached.");
            }
        }
        bail!("GitHub API request error: {status}");
    }
    Ok(resp.json()?)
}
