use std::path::Path;
use cargo_lock::Lockfile;
use anyhow::Result;

fn main() -> Result<()> {

    let lock_path = Path::new("Cargo.lock");
    let lockfile = Lockfile::load(lock_path)?;
    let name = "wasm-bindgen".parse()?;

    let ver = lockfile
        .packages
        .into_iter()
        .find(|p| p.name == name)
        .map(|p| p.version.to_string());

    let ver = ver.as_deref().unwrap_or("0.2.80");

    println!("{}", ver);
    Ok(())
}
