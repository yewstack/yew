use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let out_path = Path::new(&out_dir);

    let original_path = src_path.join("getting-started/build-a-sample-app.md");
    let original_str = fs::read_to_string(original_path)?.replace("fn main", "fn _main");
    let sanitized_path = out_path.join("getting-started/build-a-sample-app.md");
    let _ignore = fs::create_dir(&out_path.join("getting-started"));
    fs::write(sanitized_path, original_str)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/getting-started-build-a-simple-app.md");

    Ok(())
}
