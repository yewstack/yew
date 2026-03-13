use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use doc_test_util::{Level, TestFile};
use glob::glob;

fn extract_mdx_code_blocks(path: &Path) -> doc_test_util::Result<Vec<String>> {
    let file = BufReader::new(File::open(path)?);
    let mut blocks = Vec::new();

    let mut err = Ok(());
    let mut lines = file
        .lines()
        .filter_map(|i| i.map_err(|e| err = Err(e)).ok());
    while let Some(line) = lines.next() {
        if !line.starts_with("```rust") {
            continue;
        }

        let mut block = String::new();
        for line in &mut lines {
            if line.starts_with("```") {
                break;
            }
            block += &line;
            block += "\n";
        }
        blocks.push(block);
    }

    Ok(blocks)
}

fn combined_code_blocks(path: &Path) -> doc_test_util::Result<String> {
    let blocks = extract_mdx_code_blocks(path)?;
    let refs: Vec<&str> = blocks.iter().map(|s| s.as_str()).collect();
    doc_test_util::combine_blocks(&refs)
}

fn inner_main() -> doc_test_util::Result {
    let home = env::var("CARGO_MANIFEST_DIR")?;
    let pattern = format!("{home}/../../website/docs/**/*.md*");
    let base = format!("{home}/../../website");
    let base = Path::new(&base).canonicalize()?;
    let dir_pattern = format!("{home}/../../website/docs/**");
    for dir in glob(&dir_pattern)? {
        println!("cargo:rerun-if-changed={}", dir?.display());
    }

    let mut level = Level::default();

    for entry in glob(&pattern)? {
        let path = entry?.canonicalize()?;
        println!("cargo:rerun-if-changed={}", path.display());
        let rel = path.strip_prefix(&base)?;

        let mut parts = vec![];
        for part in rel {
            parts.push(
                part.to_str()
                    .ok_or_else(|| format!("Non-UTF8 path: {rel:?}"))?,
            );
        }

        let stem = path
            .file_stem()
            .ok_or_else(|| format!("no filename in path {path:?}"))?
            .to_str()
            .ok_or_else(|| format!("non-UTF8 path: {path:?}"))?
            .replace('-', "_");

        let test_file = if doc_test_util::should_combine_mdx(&path)? {
            let code = combined_code_blocks(&path)?;
            TestFile::Combined { stem, code }
        } else {
            TestFile::IncludeStr {
                stem,
                path: path.clone(),
            }
        };

        level.insert_file(test_file, &parts[..]);
    }

    let out = format!("{}/website_tests.rs", env::var("OUT_DIR")?);

    fs::write(out, level.to_contents()?)?;
    Ok(())
}

fn main() -> ExitCode {
    match inner_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
