use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub fn write_changelog(changelog_path: &str, version_changelog: &[u8]) -> Result<()> {
    let old_changelog = File::open(changelog_path)
        .context(format!("could not open {} for reading", changelog_path))?;
    let old_changelog_reader = BufReader::new(old_changelog);

    let changelog_path_new = &format!("{}.new", changelog_path);

    let mut new_changelog = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(changelog_path_new)
        .context(format!("could not open {} for writing", changelog_path_new))?;

    new_changelog.write_all(version_changelog)?;

    for old_line in old_changelog_reader.lines().skip(2) {
        writeln!(new_changelog, "{}", old_line?)?;
    }

    drop(new_changelog);

    fs::remove_file(changelog_path).context(format!("Could not delete {}", changelog_path))?;
    fs::rename(changelog_path_new, changelog_path).context(format!(
        "Could not replace {} with {}",
        changelog_path, changelog_path_new
    ))?;

    Ok(())
}
