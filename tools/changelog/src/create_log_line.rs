use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use git2::Error;
use git2::Oid;
use git2::Repository;
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Mutex;

use crate::github_issue_labels_fetcher::GitHubIssueLabelsFetcher;
use crate::github_user_fetcher::GitHubUsersFetcher;
use crate::log_line::LogLine;

lazy_static! {
    static ref REGEX_FOR_ISSUE_ID_CAPTURE: Regex = Regex::new(r"\s*\(#(\d+)\)").unwrap();
    static ref GITHUB_USERS_FETCHER: Mutex<GitHubUsersFetcher> = Default::default();
    static ref GITHUB_ISSUE_LABELS_FETCHER: Mutex<GitHubIssueLabelsFetcher> = Default::default();
    static ref PACKAGE_LABELS: Vec<String> = vec![];
}

pub fn create_log_line(
    repo: &Repository,
    package_labels: &'static [&'static str],
    oid: Result<Oid, Error>,
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

    let match_to_be_stripped = captures.get(0).ok_or(anyhow!(
        "Failed to capture first group - issue part of the message like \" (#2263)\""
    ))?;
    let mut message = commit_first_line.clone();
    message.replace_range(match_to_be_stripped.range(), "");

    let issue_id = captures
        .get(1)
        .ok_or(anyhow!(
            "Failed to capture second group - issue id like \"2263\""
        ))?
        .as_str()
        .to_string();

    let user = GITHUB_USERS_FETCHER
        .lock()
        .map_err(|err| anyhow!("Failed to lock GITHUB_USERS_FETCHER: {}", err))?
        .fetch_user_by_commit_author(email, oid.to_string())
        .with_context(|| format!("Could not find GitHub user for commit: {}", oid))?
        .to_string();

    let issue_labels = GITHUB_ISSUE_LABELS_FETCHER
        .lock()
        .map_err(|err| anyhow!("Failed to lock GITHUB_ISSUE_LABELS_FETCHER: {}", err))?
        .fetch_issue_labels(issue_id.clone())
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
