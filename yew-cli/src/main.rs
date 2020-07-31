use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use exitcode;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::{env, fs};

use std::fs::{remove_file, File};
use std::io::Write;
use webbrowser;

mod error;

use crate::error::RunError::SpawnServerError;
use crate::error::{BuildError, RunError, SubcommandError};
use actix_web::HttpServer;
use std::collections::VecDeque;

const STANDARD_HTML: &str = include_str!("standard_html.html");

// Usages:
//  yew run directory/
//  yew build --run directory/ (same as above)
//  yew build directory/ (only builds)
//  yew build examples/* (to build all examples)

// it was way easier to define a macro here than to try to deal with Clap's weird lifetime issues
macro_rules! common_flags {
    ($subcommand:expr) => (
        $subcommand
            .arg(
                Arg::from_usage("<scheme> 'The scheme for which to build (\"wasm-pack\" or \"wasm-bindgen\"). Default: wasm-bindgen'")
                    .possible_values(&["wasm-pack", "wasm-bindgen"])
                    .long("scheme")
                    .short("s")
                    .required(false)
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("project_dir")
                    .multiple(true)
                    .takes_value(true)
                    .value_name("project directory")
                    .help("Path(s) to the project directory(ies) for the Yew application(s) that will be built")
                    .required(true)
            )
            .arg(
                Arg::with_name("cargo_flags")
                    .help("List of flags, terminated by semicolon, to pass to `cargo build`")
                    .takes_value(true)
                    .value_terminator(";")
                    .multiple(true)
                    .min_values(1)
                    .value_name("cargo_flags")
                    .long("cargo-flags")
            )
            .arg(
                Arg::with_name("wasm_bindgen_flags")
                    .help("List of flags, terminated by semicolon, to pass to `wasm-bindgen`")
                    .takes_value(true)
                    .value_terminator(";")
                    .multiple(true)
                    .min_values(1)
                    .value_name("wb_flags")
                    .long("wb-flags")
            )
            .arg(
                Arg::with_name("wasm_pack_flags")
                    .help("List of flags, terminated by semicolon, to pass to `wasm-pack`")
                    .takes_value(true)
                    .value_terminator(";")
                    .multiple(true)
                    .min_values(1)
                    .value_name("wp_flags")
                    .long("wp-flags")
            )
    );
}

#[actix_rt::main]
async fn main() {
    let matches = App::new("Yew CLI")
        .version("0.2")
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
            cmd_run(matches)
                .await
                .map_err(|e| SubcommandError::RunError(e))?;
        }
        "build" => {
            if matches.is_present("run") {
                cmd_run(matches).await.map_err(SubcommandError::RunError)?;
            } else {
                cmd_build(matches).map_err(SubcommandError::BuildError)?;
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
    let paths = matches
        .values_of("project_dir")
        .unwrap()
        .map(|p| cwd().join(p))
        .collect::<Vec<PathBuf>>();
    let paths = paths
        .iter()
        .map(|p| canonicalize(p))
        .collect::<Vec<PathBuf>>();
    paths
}

async fn cmd_run<'a>(matches: ArgMatches<'a>) -> Result<(), RunError> {
    let projects = unwrap_project_dir(&matches);
    let project_count = projects.len();
    cmd_build(matches.clone()).map_err(RunError::BuildError)?;
    let run = match project_count {
        1 => {
            let project = &projects[0].join("static");
            let project = project.clone();
            let path = String::from(project.to_str().unwrap());
            let future = HttpServer::new(move || {
                actix_web::App::new().service(
                    actix_files::Files::new("/", path.as_str())
                        .use_last_modified(true)
                        .index_file("index.html"),
                )
            })
            .bind("127.0.0.1:3030")
            .unwrap()
            .run();
            println!();
            //TODO: make this a flag
            if webbrowser::open("http://127.0.0.1:3030/").is_err() {
                eprintln!("Could not open web browser");
            }
            println!("Server running at http://127.0.0.1:3030/");
            future.await
        }
        0 => panic!("this should never happen because projects are required by clap"),
        _ => {
            let future = HttpServer::new(move || {
                projects
                    .iter()
                    .map(|x| {
                        (
                            String::from(x.file_name().unwrap().to_str().unwrap()),
                            String::from(x.join("static").to_str().unwrap()),
                        )
                    })
                    .fold(actix_web::App::new(), |acc, (name, path)| {
                        acc.service(
                            actix_files::Files::new(format!("/{}", name).as_str(), path.as_str())
                                .use_last_modified(true)
                                .index_file("index.html"),
                        )
                    })
            })
            .bind("127.0.0.1:3030")
            .unwrap()
            .run();
            future.await
        }
    };
    match run {
        Ok(_) => Ok(()),
        Err(_) => Err(SpawnServerError),
    }
}

fn cmd_build(matches: ArgMatches) -> Result<(), BuildError> {
    let cargo_flags: Vec<OsString> = match matches.values_of_os("cargo_flags") {
        Some(flags) => flags.map(|flag| flag.to_os_string()).collect(),
        None => vec![],
    };
    let paths = unwrap_project_dir(&matches);
    let is_wasm_pack = {
        let scheme = matches
            .value_of("scheme")
            .unwrap_or("wasm-bindgen")
            .to_string();
        if &scheme != "wasm-bindgen" && &scheme != "wasm-pack" {
            Err(BuildError::InvalidScheme(scheme.clone()))?
        }
        scheme == "wasm-pack"
    };

    for path in paths {
        let path_str = path.to_str().unwrap();
        if !path.join("Cargo.toml").exists() {
            println!("{} doesn't have a Cargo.toml file", path_str);
            Err(BuildError::NoCargoToml(path_str.to_string()))?
        }
        println!("starting building {}", path_str);
        if is_wasm_pack {
            let wasm_pack_flags: Vec<OsString> = match matches.values_of_os("wp_flags") {
                Some(flags) => flags.map(|flag| flag.to_os_string()).collect(),
                None => vec![],
            };
            execute_wasm_pack(&cargo_flags, &wasm_pack_flags, path.as_path());
        } else {
            let wasm_bindgen_flags: Vec<OsString> = match matches.values_of_os("wb_flags") {
                Some(flags) => flags.map(|flag| flag.to_os_string()).collect(),
                None => vec![],
            };
            execute_wasm_bindgen(&cargo_flags, &wasm_bindgen_flags, path.as_path());
        }
        let static_path = path.join("static");
        let html_path = static_path.join("index.html");
        if !html_path.exists() {
            let mut file = File::create(html_path).expect("failed to make index.html file");
            file.write_all(STANDARD_HTML.as_bytes())
                .expect("failed to write index.html file");
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

fn print_args(binary: &str, args: Vec<OsString>) {
    let mut output_str = String::from(binary);
    for arg in args.clone() {
        output_str.push_str(format!(" {}", arg.to_string_lossy()).as_ref());
    }
    println!("{}", output_str);
}

fn execute_wasm_bindgen(
    cargo_flags: &Vec<OsString>,
    wasm_bindgen_flags: &Vec<OsString>,
    path: &Path,
) {
    // TODO: first run cargo build [--release] --target wasm32-unknown-unknown, then
    // wasm-bindgen --target web --no-typescript --out-dir ./static/ --out-name wasm "$TARGET_DIR/$EXAMPLE.wasm"
    eprintln!("wasm-bindgen support is TODO");
    exit(1);
}

fn execute_wasm_pack(cargo_flags: &Vec<OsString>, wasm_pack_flags: &Vec<OsString>, path: &Path) {
    //wasm-pack build --target web --out-name wasm --out-dir ./static

    let binary = "wasm-pack";

    let mut args: Vec<OsString> = Vec::new();
    args.push("build".into());
    args.extend(wasm_pack_flags.iter().cloned());
    args.push("--target".into());
    args.push("web".into());
    args.push("--out-name".into());
    args.push("wasm".into());
    args.push("--out-dir".into());
    args.push("static".into());
    if cargo_flags.len() > 0 {
        args.push("--".into());
        args.extend(cargo_flags.clone());
    }

    print_args(binary, args.clone());

    Command::new(binary)
        .current_dir(path)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn wasm-pack")
        .wait()
        .unwrap();
}
