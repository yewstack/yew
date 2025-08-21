use std::fs;
use std::path::Path;

use serde::Deserialize;
use toml::Table;

/// Examples that don't use Trunk for building
pub const NO_TRUNK_EXAMPLES: [&str; 3] = ["simple_ssr", "ssr_router", "wasi_ssr_module"];

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub fn get_latest_wasm_opt_version() -> String {
    let url = "https://api.github.com/repos/WebAssembly/binaryen/releases/latest";
    let client = reqwest::blocking::Client::new();

    // github api requires a user agent
    // https://docs.github.com/en/rest/using-the-rest-api/troubleshooting-the-rest-api?apiVersion=2022-11-28#user-agent-required
    let req_builder = client.get(url).header("User-Agent", "yew-wasm-opt-checker");

    // Send the request
    let res = req_builder
        .send()
        .expect("Failed to send request to GitHub API");

    if !res.status().is_success() {
        // Get more details about the error
        let status = res.status();
        let error_text = res
            .text()
            .unwrap_or_else(|_| "Could not read error response".to_string());

        panic!("GitHub API request failed with status: {status}. Details: {error_text}");
    }

    let release: GitHubRelease = res.json().expect("Failed to parse GitHub API response");
    release.tag_name
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
    let table: Table = toml::from_str(&content).unwrap();
    let tools = table.get("tools").unwrap().as_table().unwrap();
    let wasm_opt = tools.get("wasm_opt").unwrap().as_str().unwrap();
    wasm_opt != latest_version
}
