use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use anyhow::{Context, Result};

pub fn write_changelog(changelog_path: &str, version_changelog: &[u8]) -> Result<()> {
    let old_changelog = File::open(changelog_path)
        .context(format!("could not open {changelog_path} for reading"))?;
    let old_changelog_reader = BufReader::new(old_changelog);

    let changelog_path_new = &format!("{changelog_path}.new");

    let mut new_changelog = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(changelog_path_new)
        .context(format!("could not open {changelog_path_new} for writing"))?;

    new_changelog.write_all(version_changelog)?;

    for old_line in old_changelog_reader.lines().skip(2) {
        writeln!(new_changelog, "{}", old_line?)?;
    }

    drop(new_changelog);

    fs::remove_file(changelog_path).context(format!("Could not delete {changelog_path}"))?;
    fs::rename(changelog_path_new, changelog_path).context(format!(
        "Could not replace {changelog_path} with {changelog_path_new}"
    ))?;

    Ok(())
}
