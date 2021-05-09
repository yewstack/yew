use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use structopt::StructOpt;

fn main() -> Result<()> {
    Cli::from_args().run()
}

#[derive(StructOpt)]
pub struct Cli {
    /// From commit.
    from: String,

    /// To commit.
    #[structopt(default_value = "HEAD")]
    to: String,

    #[structopt(skip = Self::open_repository())]
    repo: git2::Repository,

    #[structopt(skip)]
    github_users: GitHubUsers,

    #[structopt(skip = regex::Regex::new(r"\(#(\d+)\)").unwrap())]
    re_issue: regex::Regex,
}

impl Cli {
    fn open_repository() -> git2::Repository {
        match git2::Repository::open(".") {
            Err(err) => {
                eprintln!("Error: could not open repository: {}", err);
                std::process::exit(1);
            }
            Ok(repo) => repo,
        }
    }

    fn run(&mut self) -> Result<()> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

        let from_object = self
            .repo
            .revparse_single(&self.from)
            .context("Could not find `from` revision")?;
        let to_object = self
            .repo
            .revparse_single(&self.to)
            .context("Could not find `to` revision")?;
        revwalk.hide(from_object.id())?;
        revwalk.push(to_object.id())?;

        let mut logs = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            let first_line = commit
                .message()
                .context("Invalid UTF-8 in commit message")?
                .lines()
                .next()
                .context("Missing commit message")?;
            let author = commit.author();
            let email = author.email().context("Missing author's email")?;

            if email.contains("dependabot") {
                continue;
            }

            let issue =
                if let Some(issue) = self.re_issue.captures(first_line).map(|x| x[1].to_string()) {
                    issue
                } else {
                    eprintln!("Missing issue for commit: {}", oid);
                    continue;
                };

            let user = self
                .github_users
                .find_user_by_commit_author(email, oid.to_string())
                .with_context(|| format!("Could not find GitHub user for commit: {}", oid))?;

            logs.push((first_line.to_owned(), user.to_owned(), issue.to_owned()));
        }

        let (features, fixes): (Vec<_>, Vec<_>) = logs
            .into_iter()
            .partition(|(msg, _, _)| msg.contains("fix"));

        println!("## ✨ **0.18.0** *(2021-05-02)*");
        println!();

        println!("- #### ⚡️ Features");
        println!();
        for (msg, user, issue) in features {
            println!(
                "  - {msg}. [[@{user}] [#{issue}]](https://github.com/yewstack/yew/pull/{issue})",
                msg = msg,
                user = user,
                issue = issue
            );
        }

        println!("- #### 🛠 Fixes");
        println!();
        for (msg, user, issue) in fixes {
            println!(
                "  - {msg}. [[@{user}] [#{issue}]](https://github.com/yewstack/yew/pull/{issue})",
                msg = msg,
                user = user,
                issue = issue
            );
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct GitHubUsers {
    cache: HashMap<String, Option<String>>,
}

impl GitHubUsers {
    pub fn find_user_by_commit_author(
        &mut self,
        key: impl Into<String>,
        commit: impl AsRef<str>,
    ) -> Option<&str> {
        self.cache
            .entry(key.into())
            .or_insert_with(|| match Self::query_commit(commit) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    None
                }
            })
            .as_deref()
    }

    fn query_commit(q: impl AsRef<str>) -> Result<Option<String>> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(format!(
                "https://api.github.com/repos/yewstack/yew/commits/{}",
                q.as_ref(),
            ))
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
        let body = resp.json::<GitHubCommitApi>()?;

        Ok(Some(body.author.login))
    }
}

#[derive(Deserialize, Debug)]
pub struct GitHubCommitApi {
    author: GitHubCommitAuthorApi,
}

#[derive(Deserialize, Debug)]
pub struct GitHubCommitAuthorApi {
    login: String,
}
