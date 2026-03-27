use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let examples_dir = Path::new(&manifest_dir).join("../../examples");

    println!("cargo:rerun-if-changed={}", examples_dir.display());
    println!(
        "cargo:rerun-if-changed={}",
        examples_dir.join("README.md").display()
    );

    let mut dirs: Vec<String> = fs::read_dir(&examples_dir)
        .expect("cannot read examples directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| !name.starts_with('.'))
        .collect();
    dirs.sort();

    let readme = fs::read_to_string(examples_dir.join("README.md")).unwrap_or_default();
    let meta = parse_readme(&readme);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("examples_data.rs");

    let mut code = String::from("&[\n");
    for name in &dirs {
        let display = prettify_name(name);
        let (desc, ct) = meta.get(name.as_str()).cloned().unwrap_or_default();
        let desc = strip_markdown(&desc);
        let desc = desc.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!(
            "    ExampleData {{ name: \"{name}\", display_name: \"{display}\", description: \
             \"{desc}\", component_type: \"{ct}\" }},\n"
        ));
    }
    code.push(']');

    fs::write(dest, code).unwrap();
}

fn parse_readme(content: &str) -> HashMap<&str, (String, String)> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if !line.starts_with("| [") {
            continue;
        }
        let cols: Vec<&str> = line.split('|').collect();
        if cols.len() < 5 {
            continue;
        }
        let name = cols[1]
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.split(']').next());
        let ct = cols[2]
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or("F")
            .to_string();
        let desc = cols[3..cols.len() - 1].join("|");
        let desc = desc.trim().to_string();
        if let Some(name) = name {
            map.insert(name, (desc, ct));
        }
    }
    map
}

fn strip_markdown(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '[' {
            let mut text = String::new();
            for ch in chars.by_ref() {
                if ch == ']' {
                    break;
                }
                text.push(ch);
            }
            if chars.peek() == Some(&'(') {
                chars.next();
                let mut depth = 1u32;
                for ch in chars.by_ref() {
                    match ch {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            out.push_str(&text.replace('`', ""));
        } else if ch == '`' {
            // strip backtick delimiters, keep content between them
        } else {
            out.push(ch);
        }
    }
    out
}

fn prettify_name(name: &str) -> String {
    name.split('_')
        .map(|word| match word {
            "ssr" => "SSR".to_string(),
            "todomvc" => "TodoMVC".to_string(),
            "webgl" => "WebGL".to_string(),
            "wasi" => "WASI".to_string(),
            "js" => "JS".to_string(),
            "hoc" => "HOC".to_string(),
            "fib" => "Fib".to_string(),
            other => {
                let mut c = other.chars();
                match c.next() {
                    None => String::new(),
                    Some(first) => {
                        let upper: String = first.to_uppercase().collect();
                        format!("{upper}{}", c.as_str())
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
