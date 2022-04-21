use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use super::github_fetch::github_fetch;

#[derive(Deserialize, Debug)]
pub struct BodyListItem {
    name: String,
}

#[derive(Debug, Default)]
pub struct GitHubIssueLabelsFetcher {
    cache: HashMap<String, Option<Vec<String>>>,
}

impl GitHubIssueLabelsFetcher {
    pub fn fetch_issue_labels(
        &mut self,
        issue: String,
        token: Option<String>,
    ) -> Option<Vec<String>> {
        self.cache
            .entry(issue.clone())
            .or_insert_with(|| match Self::inner_fetch(&issue, token) {
                Ok(labels) => labels,
                Err(err) => {
                    eprintln!("fetch_issue_labels Error: {}", err);
                    None
                }
            })
            .clone()
    }

    fn inner_fetch(q: &str, token: Option<String>) -> Result<Option<Vec<String>>> {
        let url = format!(
            "https://api.github.com/repos/yewstack/yew/issues/{}/labels",
            q,
        );
        let body: Vec<BodyListItem> = github_fetch(&url, token)?;
        let label_names: Vec<String> = body.into_iter().map(|label| label.name).collect();
        Ok(Some(label_names))
    }
}
