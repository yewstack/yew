use std::io::{stdout, Write};

use anyhow::Result;

pub fn stdout_tag_description_changelog(fixes_logs: &[u8], features_logs: &[u8]) -> Result<()> {
    let mut tag_changelog = Vec::new();

    writeln!(tag_changelog, "# Changelog")?;
    writeln!(tag_changelog)?;

    if fixes_logs.is_empty() && features_logs.is_empty() {
        writeln!(tag_changelog, "No changes")?;
        writeln!(tag_changelog)?;
    }

    if !fixes_logs.is_empty() {
        writeln!(tag_changelog, "## 🛠 Fixes")?;
        writeln!(tag_changelog)?;
        tag_changelog.extend(fixes_logs);
        writeln!(tag_changelog)?;
    }

    if !features_logs.is_empty() {
        writeln!(tag_changelog, "## ⚡️ Features")?;
        writeln!(tag_changelog)?;
        tag_changelog.extend(features_logs);
    }

    stdout().write_all(&tag_changelog)?;

    Ok(())
}
