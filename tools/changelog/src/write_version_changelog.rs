use anyhow::Result;
use semver::Version;
use std::io::Write;

use crate::yew_package::YewPackage;

pub fn write_changelog_file(
    fixes_logs: &[u8],
    features_logs: &[u8],
    package: YewPackage,
    next_version: Version,
) -> Result<Vec<u8>> {
    let mut version_only_changelog = Vec::default();

    writeln!(version_only_changelog, "# Changelog")?;
    writeln!(version_only_changelog)?;

    writeln!(
        version_only_changelog,
        "## ✨ {package} **{next_version}** *({release_date})* Changelog",
        next_version = next_version,
        package = package.to_string(),
        release_date = chrono::Utc::now().format("%Y-%m-%d")
    )?;
    writeln!(version_only_changelog)?;

    if fixes_logs.is_empty() && features_logs.is_empty() {
        writeln!(version_only_changelog, "No changes")?;
        writeln!(version_only_changelog)?;
    }

    if !fixes_logs.is_empty() {
        writeln!(version_only_changelog, "### 🛠 Fixes")?;
        writeln!(version_only_changelog)?;
        version_only_changelog.extend(fixes_logs);
        writeln!(version_only_changelog)?;
    }

    if !features_logs.is_empty() {
        writeln!(version_only_changelog, "### ⚡️ Features")?;
        writeln!(version_only_changelog)?;
        version_only_changelog.extend(features_logs);
        writeln!(version_only_changelog)?;
    }

    Ok(version_only_changelog)
}
