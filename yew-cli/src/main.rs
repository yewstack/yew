use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};
use exitcode;
use lazy_static::lazy_static;
use maplit::hashmap;
use rayon::prelude::IntoParallelIterator;
use log::{error, info, warn};

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::sync::Mutex;
use std::{env, fs};

use std::io::{Stdin, Write};
use std::fs::{File, remove_file};
use std::thread::sleep;
use std::time::Duration;

mod error;

use crate::error::{SubcommandError, RunError, BuildError};

const STANDARD_HTML: &str = include_str!("standard_html.html");

// Usages:
//  yew run directory/
//  yew build --run directory/ (same as above)
//  yew build directory/ (only builds)
//  yew build examples/* (to build all examples)

macro_rules! common_flags {
    ($subcommand:expr) => (
        $subcommand
            .arg(
                Arg::with_name("release")
                    .help("Whether to invoke `cargo build` using the --release flag")
                    .long("release")
            )
            .arg(
                Arg::with_name("project_dir")
                    .long("path")
                    .short("p")
                    .multiple(true)
                    .takes_value(true)
                    .value_name("project directory")
                    .help("Path(s) to the project directory(ies) for the Yew application(s) that will be built")
                    .required(true)
            )
    );
}

#[tokio::main]
async fn main() {
    let matches = App::new("Yew CLI")
        .version("0.1")
        .about("Builds and runs Yew application projects")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            common_flags!(
                SubCommand::with_name("build")
                    .about("Compile a Yew application")
                    .arg(
                        Arg::with_name("run")
                            .help("Start a web server for the built project and open it in a browser window (equivalent to `yew-cli run`)")
                            .long("run")
                            .short("r")
                    )
            )
        )
        .subcommand(
            common_flags!(
                SubCommand::with_name("run")
                    .about("Compile and start serving a Yew application in the browser (equivalent to `yew-cli build --run`)")
            )
        )
        .get_matches();

    let subcommand = matches.subcommand_name().unwrap();
    let matches = matches.subcommand().1.unwrap();
    let matches = matches.clone();

    if let Err(err) = exec_subcommand(subcommand, matches).await {
        eprintln!("Fatal error: {}", err);
        let exit_code: i32 = err.into();
        exit(exit_code)
    }

    exit(exitcode::OK)
}

async fn exec_subcommand(subcommand: &str, matches: ArgMatches<'_>) -> Result<(), SubcommandError> {
    match subcommand {
        "run" => {
            cmd_run(matches).await.map_err(|e| SubcommandError::RunError(e))?;
        },
        "build" => {
            if matches.is_present("run") {
                cmd_run(matches).await.map_err(|e| SubcommandError::RunError(e))?;
            } else {
                cmd_build(matches);
            };
        }
        _ => panic!("unknown subcommand"),
    };
    Ok(())
}

fn canonicalize(path: &PathBuf) -> PathBuf {
    let can = fs::canonicalize(path).unwrap();
    if cfg!(target_os = "windows") {
        //this is done cause on rust for some reason puts a \\?\ prefix before all paths, which fucks up
        //dont know if its just windows, but it feels like one of those windows things
        let str = can.to_str().unwrap();
        PathBuf::from(String::from(&str[4..]))
    } else {
        can
    }
}

fn unwrap_project_dir(matches: &ArgMatches) -> Vec<PathBuf> {
    let paths = matches.values_of("project_dir").unwrap().map(|p| cwd().join(p)).collect::<Vec<PathBuf>>();
    let paths = paths.iter().map(|p| canonicalize(p)).collect::<Vec<PathBuf>>();
    paths
}

async fn cmd_run<'a>(matches: ArgMatches<'a>) -> Result<(), RunError> {
    cmd_build(matches.clone());
    let projects = unwrap_project_dir(&matches);
    let server = match projects.len() {
        0 => panic!("this should never happen because projects are required by clap"),
        1 => {
            let project = &projects[0].join("static");
            let project = project.clone();
            let path = String::from(project.to_str().unwrap());
            warp::serve(warp::fs::dir(path))
                .run(([127, 0, 0, 1], 3030))
        }
        _ => {
            Err(RunError::MultipleProjects)?
        }
    };
    server.await;
    Ok(())
}

fn cmd_build(matches: ArgMatches) -> Result<(), BuildError> {
    let has_release_flag = matches.is_present("release");
    let paths = unwrap_project_dir(&matches);

    for path in paths {
        let path_str = path.to_str().unwrap();
        if !path.join("Cargo.toml").exists() {
            println!("{} doesn't have a Cargo.toml file", path_str);
            Err(BuildError::NoCargoToml(path_str.to_string()))?
        }
        println!("starting building {}", path_str);
        execute_wasm_pack(has_release_flag, path.as_path());
        let static_path = path.join("static");
        let html_path = static_path.join("index.html");
        if !html_path.exists() {
            let mut file = File::create(html_path).expect("failed to make index.html file");
            file.write_all(STANDARD_HTML.as_bytes());
        }
        //TODO: make this a flag
        let gitignore_path = static_path.join(".gitignore");
        if gitignore_path.exists() {
            remove_file(gitignore_path).expect("failed to delete .gitignore");
        }
    }

    Ok(())
}

fn cwd() -> PathBuf {
    env::current_dir().expect("couldnt resolve current working directory")
}

fn execute_wasm_pack(has_release_flag: bool, path: &Path) {
    let name = path.file_name().unwrap().to_str().unwrap();
    //wasm-pack build --target web --out-name wasm --out-dir ./static
    Command::new("wasm-pack")
        .current_dir(path)
        .arg("build")
        .arg(if has_release_flag { "--release" } else { "--debug" })
        .arg("--target")
        .arg("web")
        .arg("--out-name")
        .arg("wasm")
        .arg("--out-dir")
        .arg("./static")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn wasm-pack").wait().unwrap();
}
