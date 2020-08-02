use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::env::BuildEnv;

use super::WASM32_TARGET_NAME;

pub fn print_args(binary: &str, args: Vec<OsString>) {
    let mut output_str = String::from(binary);
    for arg in args.clone() {
        // TODO use shell escape
        output_str.push_str(&format!(" {}", arg.to_string_lossy()));
    }
    println!("{}", output_str);
}

pub fn execute_wasm_bindgen(
    is_release: bool,
    cargo_flags: &Vec<OsString>,
    wasm_bindgen_flags: &Vec<OsString>,
    project_root: &Path,
) -> Result<(), i32> {
    execute_cargo_build(
        is_release,
        cargo_flags,
        WASM32_TARGET_NAME.to_string(),
        project_root,
    )?;
    // TODO: first run cargo build [--release] --target wasm32-unknown-unknown, then
    // wasm-bindgen --target web --no-typescript --out-dir ./static/ --out-name wasm "$TARGET_DIR/$EXAMPLE.wasm"

    let wasm_env = BuildEnv::new(project_root, is_release);
    let wasm_path = wasm_env.generated_wasm.as_path();

    let mut args: Vec<OsString> = Vec::new();
    args.extend(wasm_bindgen_flags.iter().cloned());
    args.push("--target".into());
    args.push("web".into());
    args.push("--no-typescript".into());
    args.push("--out-dir".into());
    args.push("static".into());
    args.push("--out-name".into());
    args.push("wasm".into());
    args.push(wasm_path.into());

    run_command_get_result(project_root, "wasm-bindgen", args)
}

pub fn execute_cargo_build(
    is_release: bool,
    cargo_flags: &Vec<OsString>,
    target: String,
    project_root: &Path,
) -> Result<(), i32> {
    let mut args: Vec<OsString> = Vec::new();
    args.push("build".into());

    args.extend(cargo_flags.iter().cloned());
    if is_release {
        args.push("--release".into());
    }
    args.push("--target".into());
    args.push(target.into());

    run_command_get_result(project_root, "cargo", args)
}

pub fn execute_wasm_pack(
    is_release: bool,
    cargo_flags: &Vec<OsString>,
    wasm_pack_flags: &Vec<OsString>,
    project_root: &Path,
) -> Result<(), i32> {
    let mut args: Vec<OsString> = Vec::new();
    args.push("build".into());

    args.extend(wasm_pack_flags.iter().cloned());
    if is_release {
        args.push("--release".into());
    }
    args.push("--target".into());
    args.push("web".into());
    args.push("--out-name".into());
    args.push("wasm".into());
    args.push("--out-dir".into());
    args.push("static".into());

    if wasm_pack_flags.len() > 0 {
        args.extend(wasm_pack_flags.clone());
    }

    if cargo_flags.len() > 0 {
        args.push("--".into());
        args.extend(cargo_flags.clone());
    }

    run_command_get_result(project_root, "wasm-pack", args)
}

fn run_command_get_result(cwd: &Path, binary: &str, args: Vec<OsString>) -> Result<(), i32> {
    // TODO print cd to move into directory
    print_args(binary, args.clone());

    let status = Command::new(binary)
        .current_dir(cwd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect(&format!("failed to spawn {}", binary));

    let code = status.code();

    if !status.success() {
        if let Some(code) = code {
            return Err(code);
        } else {
            panic!("Killed by signal");
        }
    } else {
        return Ok(());
    }
}
