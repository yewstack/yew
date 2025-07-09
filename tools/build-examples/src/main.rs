use std::path::Path;
use std::process::{Command, ExitCode};
use std::{env, fs};

use build_examples::{get_latest_wasm_opt_version, is_wasm_opt_outdated, NO_TRUNK_EXAMPLES};

fn main() -> ExitCode {
    // Must be run from root of the repo:
    // yew $ cargo r -p build-examples -b build-examples
    let output_dir = env::current_dir().expect("Failed to get current directory");
    let output_dir = output_dir.join("dist");

    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    let examples_dir = Path::new("examples");
    if !examples_dir.exists() {
        eprintln!(
            "examples directory not found. Make sure you're running from the root of the repo."
        );
        return ExitCode::from(1);
    }

    let mut failure = false;
    let latest_wasm_opt = get_latest_wasm_opt_version();
    let mut outdated_examples = Vec::new();
    let mut outdated_example_paths = Vec::new();

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
            outdated_examples.push(example.clone());
            outdated_example_paths.push(path.clone());
        }

        println!("::group::Building {example}");

        if !build_example(&path, &output_dir, &example) {
            eprintln!("::error ::{example}  failed to build");
            failure = true;
        }

        println!("::endgroup::");
    }

    // Emit warning if any examples have outdated wasm_opt
    if !outdated_examples.is_empty() {
        println!(
            "::warning ::{} example crates do not have up-to-date wasm_opt: {}",
            outdated_examples.len(),
            outdated_examples.join(", ")
        );
    }

    if failure {
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}

fn build_example(path: &Path, output_dir: &Path, example: &str) -> bool {
    let public_url_prefix = env::var("PUBLIC_URL_PREFIX").unwrap_or_default();

    let dist_dir = output_dir.join(example);

    let uses_rand = has_rand_dependency(path);

    let rustflags = format!(
        "--cfg nightly_yew{}",
        if uses_rand {
            " --cfg getrandom_backend=\"wasm_js\""
        } else {
            ""
        }
    );

    // Run trunk build command
    let status = Command::new("trunk")
        .current_dir(path)
        .arg("build")
        .env("RUSTFLAGS", rustflags)
        .arg("--release")
        .arg("--dist")
        .arg(&dist_dir)
        .arg("--public-url")
        .arg(format!("{public_url_prefix}/{example}"))
        .arg("--no-sri")
        .status();

    match status {
        Ok(status) if status.success() => {
            // Check for undefined symbols (imports from 'env')
            let js_files = match fs::read_dir(&dist_dir) {
                Ok(entries) => entries
                    .filter_map(Result::ok)
                    .filter(|e| e.path().extension().is_some_and(|ext| ext == "js"))
                    .collect::<Vec<_>>(),
                Err(_) => return false,
            };

            for js_file in js_files {
                let content = match fs::read_to_string(js_file.path()) {
                    Ok(content) => content,
                    Err(_) => return false,
                };

                if content.contains("from 'env'") {
                    return false;
                }
            }

            true
        }
        _ => false,
    }
}

// Function to check if the crate has a rand dependency
fn has_rand_dependency(path: &Path) -> bool {
    let cargo_toml_path = path.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        return false;
    }

    match fs::read_to_string(&cargo_toml_path) {
        Ok(content) => content.contains("rand = ") || content.contains("rand =\n"),
        Err(_) => false,
    }
}
