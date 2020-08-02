use std::env;
use std::path::{Path, PathBuf};

use super::WASM32_TARGET_NAME;

pub(crate) struct BuildEnv {
    pub generated_wasm: PathBuf,
}

impl BuildEnv {
    pub fn new(project_root: &Path, is_release: bool) -> Self {
        let cargo_toml = project_root.join("Cargo.toml");
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(cargo_toml)
            .exec()
            .expect("Failed to get crate metadata");

        let workspace_root = metadata.workspace_root;

        let base_target = (match env::var_os("CARGO_TARGET_DIR") {
            Some(provided_target) => Path::new(&provided_target).join(WASM32_TARGET_NAME),
            None => workspace_root
                .clone()
                .join("target")
                .join(WASM32_TARGET_NAME),
        })
        .join(if is_release { "release" } else { "debug" });

        let package_id: cargo_metadata::PackageId = metadata
            .resolve
            .and_then(|resolve| resolve.root)
            .expect("No root package found");
        let package = metadata
            .packages
            .iter()
            .find(|pkg| pkg.id == package_id)
            .expect("Could not access root package");
        let crate_name = &package.name.replace("-", "_"); // TODO test this on Windows; may not have underscores

        return BuildEnv {
            generated_wasm: base_target.join(format!("{}.wasm", crate_name)),
        };
    }
}
