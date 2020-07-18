use std::collections::{VecDeque, HashMap};
use maplit::hashmap;
use std::{env, fs};
use std::path::{Path, PathBuf};
use rayon::prelude::IntoParallelIterator;
use std::process::Command;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref RELEASE: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let vars = env::args().collect::<Vec<String>>();
    let (flags, vars): (Vec<&str>, Vec<&str>) = vars.iter().map(|s| s.as_str()).partition(|s| {
        let chars = s.chars().collect::<Vec<char>>();
        (chars.len() > 1 && chars[0] == '-')
            ||
            (chars.len() > 2 && chars[0] == '-' && chars[1] == '-')
    });
    let mut vars = vars.into_iter().collect::<VecDeque<&str>>();
    {
        let mut r = RELEASE.lock().unwrap();
        *r = flags.contains(&"--release");
    }
    vars.pop_front();
    println!("{:?}", vars);
    match vars.pop_front() {
        Some("examples") => {
            enum BuildType {
                Build,
                Run,
            }
            let build_type = match vars.pop_front() {
                Some("build") => BuildType::Build,
                Some("run") => BuildType::Run,
                None => panic!("you need to specify the build type"),
                _ => panic!("not a valid build type"),
                //TODO make these error messages better
            };
            let examples_path = cwd().join("examples");
            match vars.pop_front() {
                None => {
                    fs::read_dir(examples_path.as_path())
                        .expect("failed to read dir examples dir")
                        .into_iter()
                        .map(|dir|
                            dir.expect("failed to read individual example directory")
                                .path()
                        )
                        .filter(|dir|
                            vec!["static", "server", "target"].contains(&dir.as_path().file_name().unwrap().to_str().unwrap())
                        )
                        .for_each(|dir| {
                            build_example(dir.as_path());
                        });
                }
                Some(example) => {
                    build_example(examples_path.join(example).as_path());
                }
            };
        }
        _ => {
            println!("not a valid command line argument")
        }
    };
}

fn cwd() -> PathBuf {
    env::current_dir().expect("couldnt resolve current working directory")
}

fn build_example(path: &Path) {
    fn target_dir() -> PathBuf {
        cwd().join("target").join("wasm32-unknown-unknown")
    }
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.ends_with("_wp") {} else if file_name == "multi_thread" {} else {
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
        println!("{}", String::from_utf8(output.stdout).expect("failed to pass stdout from cargo build"));
        let output = Command::new("wasm-bindgen")
            .current_dir(path)
            .args(&["--target", "web", "--no-typescript", "--out-dir", "static/", "--out-name", "wasm",
                target_dir().join(format!("{}.wasm", file_name)).to_str().unwrap()])
            .output()
            .expect("failed to execute wasm-bindgen process");
        println!("{}", String::from_utf8(output.stdout).expect("failed to pass stdout from cargo build"));
    }
}
