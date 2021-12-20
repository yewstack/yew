use serde::de::DeserializeOwned;
use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use reqwest::blocking::Client;

pub fn github_fetch<T: DeserializeOwned>(url: &str) -> Result<T> {
    thread::sleep(Duration::from_secs(1));
    let request_client = Client::new();
    let resp = request_client
        .get(url)
        .header("user-agent", "reqwest")
        .header("accept", "application/vnd.github.v3+json")
        .send()?;
    let status = resp.status();
    if !status.is_success() {
        if let Some(remaining) = resp.headers().get("x-ratelimit-remaining") {
            if remaining == "0" {
                bail!("GitHub API limit reached.");
            }
        }
        bail!("GitHub API request error: {}", status);
    }
    Ok(resp.json()?)
}
