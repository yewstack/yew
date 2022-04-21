use anyhow::{Context, Result};
use git2::{Repository, Sort};

use crate::create_log_line::create_log_line;
use crate::log_line::LogLine;

pub fn create_log_lines(
    from: String,
    to: String,
    package_labels: &'static [&'static str],
    token: Option<String>,
) -> Result<Vec<LogLine>> {
    let repo = Repository::open_from_env()?;

    let from_oid = repo
        .revparse_single(&from)
        .context("Could not find `from` revision")?
        .id();
    let to_oid = repo
        .revparse_single(&to)
        .context("Could not find `to` revision")?
        .id();

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;

    revwalk.hide(from_oid)?;
    revwalk.push(to_oid)?;

    revwalk
        .into_iter()
        .filter_map(|oid| create_log_line(&repo, package_labels, oid, token.clone()).transpose())
        .collect()
}
