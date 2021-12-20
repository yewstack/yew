use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

use super::github_fetch::github_fetch;

#[derive(Deserialize, Debug)]
struct ResponseBody {
    author: ResponseBodyAuthor,
}

#[derive(Deserialize, Debug)]
struct ResponseBodyAuthor {
    login: String,
}

#[derive(Debug, Default)]
pub struct GitHubUsersFetcher {
    cache: HashMap<String, Option<String>>,
}

impl GitHubUsersFetcher {
    pub fn fetch_user_by_commit_author(
        &mut self,
        key: impl Into<String>,
        commit: impl AsRef<str>,
    ) -> Option<&str> {
        self.cache
            .entry(key.into())
            .or_insert_with(|| match Self::inner_fetch(commit) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("fetch_user_by_commit_author Error: {}", err);
                    None
                }
            })
            .as_deref()
    }

    fn inner_fetch(commit: impl AsRef<str>) -> Result<Option<String>> {
        let url = format!(
            "https://api.github.com/repos/yewstack/yew/commits/{}",
            commit.as_ref(),
        );
        let body: ResponseBody = github_fetch(&url)?;
        Ok(Some(body.author.login))
    }
}
