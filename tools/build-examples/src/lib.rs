use std::fs;
use std::path::Path;

use regex::Regex;

/// Examples that don't use Trunk for building
pub const NO_TRUNK_EXAMPLES: [&str; 3] = ["simple_ssr", "ssr_router", "wasi_ssr_module"];

pub fn get_latest_wasm_opt_version() -> String {
    let url = "https://github.com/WebAssembly/binaryen/releases";
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send().unwrap();
    let body = res.text().unwrap();
    let re = Regex::new(r#"version_(\d+)"#).unwrap();
    let captures = re.captures_iter(&body);
    let mut versions: Vec<u32> = captures
        .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        .collect();
    versions.sort();
    format!("version_{}", versions.last().unwrap())
}

pub fn is_wasm_opt_outdated(path: &Path, latest_version: &str) -> bool {
    let trunk_toml_path = path.join("Trunk.toml");

    if !trunk_toml_path.exists() {
        return true;
    }

    let content = match fs::read_to_string(&trunk_toml_path) {
        Ok(content) => content,
        Err(_) => return true,
    };

    // Check if wasm_opt is configured and up-to-date
    let re = Regex::new(r#"(?m)^\[tools\]\s*\nwasm_opt\s*=\s*"(version_\d+)""#).unwrap();
    match re.captures(&content) {
        Some(captures) => {
            let current_version = captures.get(1).unwrap().as_str();
            current_version != latest_version
        }
        None => true,
    }
}
