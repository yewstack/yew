use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};
use lazy_static::lazy_static;
use maplit::hashmap;
use rayon::prelude::IntoParallelIterator;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::sync::Mutex;
use std::{env, fs};

use log::{error, info, warn};
use std::io::Stdin;

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
                        .short("r")
                )
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
        )
        .get_matches();

    let subcommand = matches.subcommand_name().unwrap();
    let matches = matches.subcommand().1.unwrap();
    match subcommand {
        "run" => cmd_run(matches),
        "build" => if matches.is_present("run") { cmd_run(matches) } else { cmd_build(matches) }
        _ => panic!("unknown subcommand"),
    }
}

fn cmd_run(matches: &ArgMatches) {
    cmd_build(matches);

    // TODO: run
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

fn cmd_build(matches: &ArgMatches) {
    let has_release_flag = matches.is_present("release");
    let paths = matches.values_of("project_dir").unwrap().map(|p|cwd().join(p)).collect::<Vec<PathBuf>>();
    let paths = paths.iter().map(|p|fs::canonicalize(p).unwrap()).collect::<Vec<PathBuf>>();
    paths.into_iter().for_each(|path| {
        let path_str = path.to_str().unwrap();
        if !path.join("Cargo.toml").exists() {
            println!("{} doesn have a Cargo.toml file", path_str);
            return;
        }
        println!("starting building {}", path_str);
        execute_wasm_pack(has_release_flag, path.as_path());
    })
}

fn cwd() -> PathBuf {
    env::current_dir().expect("couldnt resolve current working directory")
}

fn execute_wasm_pack(has_release_flag: bool, path: &Path) {
    let name = path.file_name().unwrap().to_str().unwrap();
    //wasm-pack build --target web --out-name wasm --out-dir ./static
    Command::new("wasm-pack")
        .current_dir(&path.to_str().unwrap()[4..]) //this is done cause on rust for some reason puts a \\?\ prefix before all paths, which fucks up
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
        .stdout(Stdio::inherit())
        .spawn()
        .expect("failed to spawn wasm-pack");
}
