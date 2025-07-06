use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use build_examples::{get_latest_wasm_opt_version, is_wasm_opt_outdated, NO_TRUNK_EXAMPLES};
use regex::Regex;

fn main() -> ExitCode {
    // Must be run from root of the repo
    let examples_dir = Path::new("examples");
    if !examples_dir.exists() {
        eprintln!(
            "examples directory not found. Make sure you're running from the root of the repo."
        );
        return ExitCode::from(1);
    }

    let latest_wasm_opt = get_latest_wasm_opt_version();
    let mut outdated_example_paths = Vec::new();
    let mut outdated_examples = Vec::new();

    // Get all entries in the examples directory
    let entries = fs::read_dir(examples_dir).expect("Failed to read examples directory");

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        // Skip if not a directory
        if !path.is_dir() {
            continue;
        }

        let example = path
            .file_name()
            .expect("Failed to get directory name")
            .to_string_lossy()
            .to_string();

        // Skip ssr examples as they don't need trunk
        if NO_TRUNK_EXAMPLES.contains(&example.as_str()) {
            continue;
        }

        // Check Trunk.toml for wasm_opt version and collect outdated examples
        if is_wasm_opt_outdated(&path, &latest_wasm_opt) {
            outdated_examples.push(example);
            outdated_example_paths.push(path);
        }
    }

    if outdated_examples.is_empty() {
        println!("All examples are up-to-date with the latest wasm_opt version: {latest_wasm_opt}");
        return ExitCode::from(0);
    }

    println!(
        "Found {} examples with outdated or missing wasm_opt configuration:",
        outdated_examples.len()
    );
    for example in &outdated_examples {
        println!("  - {example}");
    }
    println!("Latest wasm_opt version is: {latest_wasm_opt}");
    println!("Updating all examples...");

    let updated_count = update_all_examples(&outdated_example_paths, &latest_wasm_opt);
    println!("Updated {updated_count} example configurations to use {latest_wasm_opt}");

    ExitCode::from(0)
}

pub fn update_all_examples(outdated_paths: &[PathBuf], latest_version: &str) -> usize {
    let mut updated_count = 0;

    let re = Regex::new(r#"(?m)^\[tools\]\s*\nwasm_opt\s*=\s*"(version_\d+)""#).unwrap();
    for path in outdated_paths {
        let trunk_toml_path = path.join("Trunk.toml");

        let content = fs::read_to_string(&trunk_toml_path).unwrap_or_default();

        let updated_content = if re.is_match(&content) {
            // Replace existing wasm_opt version
            re.replace(&content, |_: &regex::Captures| {
                format!(
                    r#"[tools]
wasm_opt = "{latest_version}""#
                )
            })
            .to_string()
        } else {
            // Add wasm_opt configuration
            if content.is_empty() {
                format!(
                    r#"[tools]
wasm_opt = "{latest_version}""#
                )
            } else {
                format!(
                    "{}\n\n[tools]\nwasm_opt = \"{}\"",
                    content.trim(),
                    latest_version
                )
            }
        };

        if let Err(e) = fs::write(&trunk_toml_path, updated_content) {
            println!("Failed to update {}: {}", trunk_toml_path.display(), e);
        } else {
            updated_count += 1;
        }
    }

    updated_count
}
