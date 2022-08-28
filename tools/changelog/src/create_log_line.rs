use std::sync::Mutex;

use anyhow::{anyhow, Context, Result};
use git2::{Error, Oid, Repository};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::github_issue_labels_fetcher::GitHubIssueLabelsFetcher;
use crate::github_user_fetcher::GitHubUsersFetcher;
use crate::log_line::LogLine;

static REGEX_FOR_ISSUE_ID_CAPTURE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s*\(#(\d+)\)").unwrap());
static GITHUB_USERS_FETCHER: Lazy<Mutex<GitHubUsersFetcher>> = Lazy::new(Default::default);
static GITHUB_ISSUE_LABELS_FETCHER: Lazy<Mutex<GitHubIssueLabelsFetcher>> =
    Lazy::new(Default::default);

pub fn create_log_line(
    repo: &Repository,
    package_labels: &'static [&'static str],
    oid: Result<Oid, Error>,
    token: Option<String>,
) -> Result<Option<LogLine>> {
    let oid = oid?;
    let commit = repo.find_commit(oid)?;
    let commit_first_line = commit
        .message()
        .context("Invalid UTF-8 in commit message")?
        .lines()
        .next()
        .context("Missing commit message")?
        .to_string();
    let author = commit.author();
    let email = author.email().context("Missing author's email")?;

    if email.contains("dependabot") || email.contains("github-action") {
        return Ok(None);
    }

    let mb_captures = REGEX_FOR_ISSUE_ID_CAPTURE
        .captures_iter(&commit_first_line)
        .last();

    let captures = match mb_captures {
        Some(some) => some,
        None => {
            eprintln!("Missing issue for commit: {}", oid);
            return Ok(None);
        }
    };

    let match_to_be_stripped = captures.get(0).ok_or_else(|| {
        anyhow!("Failed to capture first group - issue part of the message like \" (#2263)\"")
    })?;
    let mut message = commit_first_line.clone();
    message.replace_range(match_to_be_stripped.range(), "");

    let issue_id = captures
        .get(1)
        .ok_or_else(|| anyhow!("Failed to capture second group - issue id like \"2263\""))?
        .as_str()
        .to_string();

    let user = GITHUB_USERS_FETCHER
        .lock()
        .map_err(|err| anyhow!("Failed to lock GITHUB_USERS_FETCHER: {}", err))?
        .fetch_user_by_commit_author(email, oid.to_string(), token.clone())
        .with_context(|| format!("Could not find GitHub user for commit: {}", oid))?
        .to_string();

    let issue_labels = GITHUB_ISSUE_LABELS_FETCHER
        .lock()
        .map_err(|err| anyhow!("Failed to lock GITHUB_ISSUE_LABELS_FETCHER: {}", err))?
        .fetch_issue_labels(issue_id.clone(), token)
        .with_context(|| format!("Could not find GitHub labels for issue: {}", issue_id))?;

    let is_issue_for_this_package = issue_labels
        .into_iter()
        .any(|label| package_labels.contains(&label.as_str()));

    if !is_issue_for_this_package {
        return Ok(None);
    }

    let log_line = LogLine {
        message,
        user,
        issue_id,
    };

    Ok(Some(log_line))
}
