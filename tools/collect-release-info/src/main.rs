use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use anyhow::{bail, Context, Result};
use serde_json::json;

fn parse_tag(tag: &str) -> Result<(&str, &str)> {
    let (package, version) = tag
        .rsplit_once("-v")
        .context("tag must match <package>-v<version>")?;

    if package.is_empty() || version.is_empty() {
        bail!("tag must match <package>-v<version>, got: {tag}");
    }

    Ok((package, version))
}

fn extract_section(content: &str, package: &str, version: &str) -> Result<String> {
    let pkg_lower = package.to_lowercase();
    let version_needle = format!("**{version}**");

    let mut lines = content.lines();
    let mut found_target = false;

    for line in lines.by_ref() {
        if !line.starts_with("## ") {
            continue;
        }

        if !line.contains("**") {
            bail!(
                "unexpected ## header that is not a version header: {:?}",
                line
            );
        }

        if line.to_lowercase().contains(&pkg_lower) && line.contains(&version_needle) {
            found_target = true;
            break;
        }
    }

    if !found_target {
        bail!("no changelog section found for {package} {version}");
    }

    let mut section_lines: Vec<&str> = Vec::new();

    for line in lines {
        if line.starts_with("## ") {
            if !line.contains("**") {
                bail!(
                    "unexpected ## header that is not a version header: {:?}",
                    line
                );
            }
            break;
        }
        section_lines.push(line);
    }

    let body = section_lines.join("\n");
    let trimmed = body.trim();

    if trimmed.is_empty() {
        bail!("changelog section for {package} {version} is empty");
    }

    Ok(trimmed.to_string())
}

fn find_latest_tag(package: &str) -> Result<String> {
    let output = Command::new("git")
        .args([
            "describe",
            "--tags",
            "--match",
            &format!("{package}-v*"),
            "--abbrev=0",
        ])
        .output()
        .context("failed to run git describe")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("no tag found for package {package}: {stderr}");
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn count_rust_lines(dir: &Path) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    let mut total = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            total += count_rust_lines(&path);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                total += content.lines().count();
            }
        }
    }
    total
}

struct ReleaseInfo {
    tag: String,
    package: String,
    version: String,
    body: String,
    line_count: usize,
}

fn main() -> Result<()> {
    let packages: Vec<String> = env::args().skip(1).collect();
    if packages.is_empty() {
        bail!(
            "Usage: collect-release-info <package> [<package>...]\nExample: collect-release-info \
             yew yew-router"
        );
    }

    let changelog = fs::read_to_string("CHANGELOG.md").context("failed to read CHANGELOG.md")?;

    let mut releases = Vec::new();

    for package in &packages {
        let tag = match find_latest_tag(package) {
            Ok(tag) => tag,
            Err(e) => {
                eprintln!("skipping {package}: {e}");
                continue;
            }
        };
        let (pkg, version) = parse_tag(&tag)?;
        let pkg = pkg.to_string();
        let version = version.to_string();

        let body = match extract_section(&changelog, &pkg, &version) {
            Ok(section) => section,
            Err(e) => {
                eprintln!("warning: {e}");
                String::new()
            }
        };

        let pkg_dir = Path::new("packages").join(package);
        let line_count = count_rust_lines(&pkg_dir);

        releases.push(ReleaseInfo {
            tag,
            package: pkg,
            version,
            body,
            line_count,
        });
    }

    releases.sort_by_key(|r| r.line_count);

    let single = releases.len() == 1;
    let releases_json: Vec<serde_json::Value> = releases
        .iter()
        .map(|r| {
            let body = if single || r.body.is_empty() {
                r.body.clone()
            } else {
                format!("## {} v{}\n\n{}", r.package, r.version, r.body)
            };
            json!({
                "tag": r.tag,
                "name": format!("{} v{}", r.package, r.version),
                "body": body,
            })
        })
        .collect();

    let version_branch = releases.last().map(|r| r.tag.as_str()).unwrap_or("");
    let releases_str = serde_json::to_string(&releases_json)?;

    if let Ok(path) = env::var("GITHUB_OUTPUT") {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .with_context(|| format!("failed to open GITHUB_OUTPUT at {path}"))?;
        writeln!(file, "releases<<RELEASES_EOF")?;
        writeln!(file, "{releases_str}")?;
        writeln!(file, "RELEASES_EOF")?;
        writeln!(file, "version_branch={version_branch}")?;
    }

    for r in &releases {
        eprintln!("{}: {} lines", r.package, r.line_count);
    }
    print!("{releases_str}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag() {
        let (pkg, ver) = parse_tag("yew-v0.23.0").unwrap();
        assert_eq!(pkg, "yew");
        assert_eq!(ver, "0.23.0");
    }

    #[test]
    fn test_parse_tag_with_hyphenated_package() {
        let (pkg, ver) = parse_tag("yew-router-v0.19.0").unwrap();
        assert_eq!(pkg, "yew-router");
        assert_eq!(ver, "0.19.0");
    }

    #[test]
    fn test_parse_tag_invalid() {
        assert!(parse_tag("yew0.23.0").is_err());
        assert!(parse_tag("-v0.23.0").is_err());
        assert!(parse_tag("yew-v").is_err());
    }

    #[test]
    fn test_extract_section_basic() {
        let content = "\
# Changelog

## ✨ yew **0.23.0** *(2026-04-01)*

### Fixes

- Fixed a bug

### Features

- Added a feature

## ✨ yew **0.22.0** *(2025-12-08)*

### Fixes

- Old fix
";
        let result = extract_section(content, "yew", "0.23.0").unwrap();
        assert!(result.contains("Fixed a bug"));
        assert!(result.contains("Added a feature"));
        assert!(!result.contains("Old fix"));
    }

    #[test]
    fn test_extract_section_not_found() {
        let content = "# Changelog\n\n## ✨ yew **0.22.0**\n\n- stuff\n";
        assert!(extract_section(content, "yew", "0.99.0").is_err());
    }

    #[test]
    fn test_extract_section_empty() {
        let content = "\
## ✨ yew **0.23.0** *(2026-04-01)*

## ✨ yew **0.22.0** *(2025-12-08)*
";
        assert!(extract_section(content, "yew", "0.23.0").is_err());
    }

    #[test]
    fn test_extract_section_last_in_file() {
        let content = "\
## ✨ yew **0.23.0** *(2026-04-01)*

### Fixes

- The only fix
";
        let result = extract_section(content, "yew", "0.23.0").unwrap();
        assert!(result.contains("The only fix"));
    }

    #[test]
    fn test_extract_section_case_insensitive_package() {
        let content = "\
## ✨ Yew **0.23.0** *(2026-04-01)*

- A change
";
        let result = extract_section(content, "yew", "0.23.0").unwrap();
        assert!(result.contains("A change"));
    }

    #[test]
    fn test_rejects_malformed_h2() {
        let content = "\
## ✨ yew **0.23.0** *(2026-04-01)*

## Not a version header

- A change
";
        assert!(extract_section(content, "yew", "0.23.0").is_err());
    }

    #[test]
    fn test_yew_has_more_code_than_yew_agent() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let yew_lines = count_rust_lines(&workspace_root.join("packages/yew"));
        let agent_lines = count_rust_lines(&workspace_root.join("packages/yew-agent"));
        assert!(
            yew_lines > agent_lines,
            "expected yew ({yew_lines}) > yew-agent ({agent_lines})"
        );
    }
}
