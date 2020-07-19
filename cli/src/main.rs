use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};
use lazy_static::lazy_static;
use maplit::hashmap;
use rayon::prelude::IntoParallelIterator;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::Mutex;
use std::{env, fs};

use log::{info, warn, error};

lazy_static! {
    static ref RELEASE: Mutex<bool> = Mutex::new(false);
}

// Usages:
//  yew run directory/
//  yew build --run directory/ (same as above)
//  yew build directory/ (only builds)
//  yew build examples/* (to build all examples) 

fn main() {
    let matches = App::new("Yew CLI")
        .version("0.1")
        .about("Builds and runs Yew application projects")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("build")
                .about("compiles a Yew application")
                .arg(
                    Arg::with_name("run")
                        .help("Start a webserver for the built project and open it in a browser window")
                        .long("run")
                        .required(true)
                        .short("r")
                )
                .arg(
                    Arg::with_name("release")
                        .help("Whether to invoke `cargo build` using the --release flag")
                        .long("release")
                )
                .arg(
                    Arg::with_name("PROJECT_DIR")
                        .help("Path to the project directory for the Yew application that will be built")
                        .required(true)
                )
        )
        .get_matches();
    
    let subcommand = matches.subcommand_name().unwrap();
    match subcommand {
        "run" => cmd_run(matches),
        "build" => cmd_build(matches),
        _ => panic!("unknown subcommand")
    }
}

fn cmd_run(matches: ArgMatches) {

}

enum BuildType {
    Build,
    Run,
}

// build all examples
// fs::read_dir(examples_path.as_path())
//     .expect("failed to read dir examples dir")
//     .into_iter()
//     .map(|dir| {
//         dir.expect("failed to read individual example directory")
//             .path()
//     })
//     .filter(|dir| {
//         vec!["static", "server", "target"]
//             .contains(&dir.as_path().file_name().unwrap().to_str().unwrap())
//     })
//     .for_each(|dir| {
//         build_example(dir.as_path());
//     });

fn cmd_build(matches: ArgMatches) {
    let has_run_flag = matches.is_present("run");

    let examples_path = cwd().join("examples");

    // build single example
    build_example(examples_path.as_path());
}

fn cwd() -> PathBuf {
    env::current_dir().expect("couldnt resolve current working directory")
}

fn build_example(path: &Path) {
    fn target_dir() -> PathBuf {
        cwd().join("target").join("wasm32-unknown-unknown")
    }
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.ends_with("_wp") {
    } else if file_name == "multi_thread" {
    } else {
        let mut args = vec!["build"];
        if *RELEASE.lock().unwrap() {
            args.push("--release")
        }
        args.append(&mut vec!["--target", "wasm32-unknown-unknown"]);
        let output = Command::new("cargo")
            .current_dir(path)
            .args(&args[0..])
            .output()
            .expect("failed to execute cargo build process");
        println!(
            "{}",
            String::from_utf8(output.stdout).expect("failed to pass stdout from cargo build")
        );
        let output = Command::new("wasm-bindgen")
            .current_dir(path)
            .args(&[
                "--target",
                "web",
                "--no-typescript",
                "--out-dir",
                "static/",
                "--out-name",
                "wasm",
                target_dir()
                    .join(format!("{}.wasm", file_name))
                    .to_str()
                    .unwrap(),
            ])
            .output()
            .expect("failed to execute wasm-bindgen process");
        println!(
            "{}",
            String::from_utf8(output.stdout).expect("failed to pass stdout from cargo build")
        );
    }
}
