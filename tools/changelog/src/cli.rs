use anyhow::{bail, Result};
use clap::Parser;
use semver::Version;

use crate::create_log_lines::create_log_lines;
use crate::get_latest_version::get_latest_version;
use crate::new_version_level::NewVersionLevel;
use crate::stdout_tag_description_changelog::stdout_tag_description_changelog;
use crate::write_changelog_file::write_changelog;
use crate::write_log_lines::write_log_lines;
use crate::write_version_changelog::write_changelog_file;
use crate::yew_package::YewPackage;

#[derive(Parser)]
pub struct Cli {
    /// package to generate changelog for
    pub package: YewPackage,

    /// package to generate changelog for
    pub new_version_level: NewVersionLevel,

    /// From ref. (ex. commit hash or for tags "refs/tags/yew-v0.19.3") overrides version level arg
    pub from: Option<String>,

    /// To commit. (ex. commit hash or for tags "refs/tags/yew-v0.19.3")
    #[clap(short = 'r', long, default_value = "HEAD")]
    pub to: String,

    /// Path to changelog file
    #[clap(short = 'f', long, default_value = "../CHANGELOG.md")]
    pub changelog_path: String,

    /// Skip writing changelog file
    #[clap(short, long)]
    pub skip_file_write: bool,

    /// Skip getting the next version
    #[clap(short = 'b', long)]
    pub skip_get_bump_version: bool,

    /// Github token
    #[clap(short = 't', long)]
    pub token: Option<String>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let Cli {
            package,
            from,
            to,
            changelog_path,
            skip_file_write,
            new_version_level,
            skip_get_bump_version,
            token,
        } = self;
        let package_labels = package.as_labels();

        // set up versions and from ref
        let (from_ref, next_version) = if skip_get_bump_version {
            let from_ref = match from {
                Some(some) => some,
                None => bail!("from required when skip_get_bump_version is true"),
            };
            let version = Version::parse("0.0.0")?;
            (from_ref, version)
        } else {
            let latest_version = get_latest_version(&package)?;

            let next_version = new_version_level.bump(latest_version.clone());

            let from_ref = match from {
                Some(some) => some,
                None => format!("refs/tags/{}-v{}", package, latest_version),
            };
            (from_ref, next_version)
        };

        // walk over each commit find text, user, issue
        let log_lines = create_log_lines(from_ref, to, package_labels, token)?;

        // categorize logs
        let (fixes, features): (Vec<_>, Vec<_>) = log_lines
            .into_iter()
            .partition(|log_line| log_line.message.to_lowercase().contains("fix"));

        // create displayable log lines
        let fixes_logs = write_log_lines(fixes)?;
        let features_logs = write_log_lines(features)?;

        if !skip_file_write {
            // create version changelog
            let version_changelog =
                write_changelog_file(&fixes_logs, &features_logs, package, next_version)?;

            // write changelog
            write_changelog(&changelog_path, &version_changelog)?;
        }

        // stdout changelog meant for tag description
        stdout_tag_description_changelog(&fixes_logs, &features_logs)?;

        Ok(())
    }
}
