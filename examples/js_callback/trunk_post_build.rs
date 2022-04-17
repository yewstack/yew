use std::fs;
use std::io::{Read, Write};

fn read_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|e| panic!("can't read {} env var: {}", env, e))
}

fn main() {
    let stage_dir = read_env("TRUNK_STAGING_DIR");
    let mut res = fs::read_dir(format!("{stage_dir}/snippets")).expect("no snippets dir in stage");
    let dir = res
        .next()
        .expect("there must be one snippets dir present")
        .expect("can't read snippets dir");
    let dir_name = dir.file_name().to_string_lossy().to_string();
    let mut index_html =
        fs::File::open(format!("{stage_dir}/index.html")).expect("can't open index.html");
    let mut html = String::new();
    index_html
        .read_to_string(&mut html)
        .expect("can't read index.html");

    let mut split = html
        .split("</head>")
        .map(|it| it.to_string())
        .collect::<Vec<String>>();

    let public_url = read_env("TRUNK_PUBLIC_URL");
    let public_url = public_url.strip_suffix("/").unwrap_or(&public_url);
    let wasm_bindgen_snippets_path = format!("{public_url}/snippets/{dir_name}");

    split.insert(1,  format!("<script>window.wasmBindgenSnippetsPath = '{wasm_bindgen_snippets_path}';</script></head>"));
    let joined = split.join("");
    drop(index_html);
    let mut index_html = fs::File::options()
        .write(true)
        .truncate(true)
        .open(format!("{stage_dir}/index.html"))
        .expect("can't open index.html");
    index_html
        .write_all(joined.as_ref())
        .expect("can't write index.html")
}
