use std::{env, fs};

use anyhow::{bail, Context, Result};

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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!(
            "Usage: extract-changelog-section <tag>\nExample: extract-changelog-section \
             yew-v0.23.0"
        );
    }

    let tag = &args[1];
    let (package, version) = parse_tag(tag)?;

    let changelog = fs::read_to_string("CHANGELOG.md").context("failed to read CHANGELOG.md")?;

    let section = extract_section(&changelog, package, version)?;
    print!("{section}");

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
}
