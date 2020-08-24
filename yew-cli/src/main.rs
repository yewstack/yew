mod WASM32_TARGET_NAME;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use exitcode;

use std::env::current_dir;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{exit};

use std::fs::{remove_file, File, create_dir_all};
use std::io::{Write};
use webbrowser;

mod env;
mod error;
mod execute;

use crate::error::{BuildError, RunError, SubcommandError};
use crate::execute::{execute_wasm_pack, execute_wasm_bindgen};
use actix_rt::System;
use actix_web::HttpServer;
use crate::error::RunError::SpawnServerError;
use include_dir::Dir;
use serde_json::Value;
use std::fs;
use include_dir::{include_dir};
use handlebars::Handlebars;

const STANDARD_HTML: &str = include_str!("standard_html.html");
const STANDARD_YEW_PROJECT: Dir = include_dir!("./standard_yew_project");

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
                Arg::with_name("release")
                    .long("release")
                    .takes_value(false)
                    .help("Create a release build. Enable optimizations and disable debug info.")
            )
            .arg(
                Arg::with_name("cargo_flags")
                    .help("(Advanced) List of flags, terminated by semicolon, to pass to `cargo build`")
                    .takes_value(true)
                    .value_terminator(";")
                    .multiple(true)
                    .min_values(1)
                    .value_name("cargo_flags")
                    .long("cargo-flags")
            )
            .arg(
                Arg::with_name("wasm_bindgen_flags")
                    .help("(Advanced) List of flags, terminated by semicolon, to pass to `wasm-bindgen`")
                    .takes_value(true)
                    .value_terminator(";")
                    .multiple(true)
                    .min_values(1)
                    .value_name("wb_flags")
                    .long("wb-flags")
            )
            .arg(
                Arg::with_name("wasm_pack_flags")
                    .help("(Advanced) List of flags, terminated by semicolon, to pass to `wasm-pack`")
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
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        System::with_current(|sys| sys.stop_with_code(1));
        default_hook(info);
    }));

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
        eprintln!("Fatal error: {:?}", err);
        let exit_code: i32 = err.into();

        System::with_current(|sys| sys.stop_with_code(exit_code));
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
        "new" => {
            let path = cwd().join(matches.value_of_os("name").unwrap());
            create_new_project(path);
        }
        _ => panic!("unknown subcommand"),
    };
    Ok(())
}

fn create_new_project(path: PathBuf) {
    let json_data = serde_json::json!({
        "project_name": path.file_name().unwrap().to_str().unwrap(),
        "username": whoami::realname()
        //add email
    });
    fn create(path: PathBuf, dir: Dir, values: &Value) {
        create_dir_all(path.join(dir.path()).clone()).expect("failed to create dir");
        for dir in dir.dirs().to_vec() {
            create(path.clone(), dir, values);
        }
        for file in dir.files().to_vec() {
            let file_contents = file.contents_utf8().expect("counldnt unwrap file contents");
            let file_contents = Handlebars::new()
                .render_template(file_contents, values).expect("couldnt use templating engine");
            let mut file = File::create(path.join(file.path())).expect("couldnt create file");
            file.write_all(file_contents.as_bytes());
        }
    }
    create_dir_all(path.clone()).expect("failed to create project dir");
    create(path.clone(), STANDARD_YEW_PROJECT, &json_data);
}

fn canonicalize(path: &PathBuf) -> PathBuf {
    let can = fs::canonicalize(path).unwrap();
    if cfg!(target_os = "windows") {
        // The \\?\ prefix on Windows is not compatible with cargo metadata, which is used
        // by wasm-bindgen and wasm-pack:
        //      https://github.com/rust-lang/cargo/issues/8626
        // So, we remove the prefix and hope that the path is not too long.
        let str = can.to_str().expect("Malformed path");
        PathBuf::from(String::from(&str[4..]))
    } else {
        can
    }
}

fn unwrap_project_dir(matches: &ArgMatches) -> Vec<PathBuf> {
    let paths = matches
        .values_of_os("project_dir")
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
    let is_release = matches.is_present("release");
    if is_release {
        eprintln!("WARNING: `yew run` is not a substitute for a production HTTP server; use it for development purposes only!");
    }
    //TODO: make the port of the webserver a flag
    let projects = unwrap_project_dir(&matches);
    let project_count = projects.len();
    cmd_build(matches.clone()).map_err(RunError::BuildError)?;
    let run = match project_count {
        1 => {
            let project = &projects[0].join("static");
            let project = project.clone();
            let path = String::from(project.to_str().unwrap());
            let future = HttpServer::new(move || {
                let project = &projects[0].join("static");
                actix_web::App::new().service(
                    actix_files::Files::new("/", project)
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
        Err(_) => Err(RunError::SpawnServerError),
    }
}

fn cmd_build(matches: ArgMatches) -> Result<(), BuildError> {
    let is_release = matches.is_present("release");
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
        let path_str = path.to_string_lossy();
        let cargo_toml = path.join("Cargo.toml");
        if !cargo_toml.exists() {
            Err(BuildError::NoCargoToml(path_str.to_string()))?
        }

        println!("starting building {}", path_str);

        let task = if is_wasm_pack {
            let wasm_pack_flags: Vec<OsString> = match matches.values_of_os("wp_flags") {
                Some(flags) => flags.map(|flag| flag.to_os_string()).collect(),
                None => vec![],
            };
            execute_wasm_pack(is_release, &cargo_flags, &wasm_pack_flags, path.as_path())
        } else {
            let wasm_bindgen_flags: Vec<OsString> = match matches.values_of_os("wb_flags") {
                Some(flags) => flags.map(|flag| flag.to_os_string()).collect(),
                None => vec![],
            };
            execute_wasm_bindgen(
                is_release,
                &cargo_flags,
                &wasm_bindgen_flags,
                path.as_path(),
            )
        };

        if let Err(exit_code) = task {
            Err(BuildError::BuildExitCode(exit_code))?
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
    std::env::current_dir().expect("couldnt resolve current working directory")
}
