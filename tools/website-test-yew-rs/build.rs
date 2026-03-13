use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use doc_test_util::{Level, TestFile};
use glob::glob;

// ---------------------------------------------------------------------------
// Raw-string literal parser
// ---------------------------------------------------------------------------

fn parse_raw_string(src: &str, start: usize) -> Option<(String, usize)> {
    let bytes = src.as_bytes();
    let mut pos = start;

    if pos >= bytes.len() || bytes[pos] != b'r' {
        return None;
    }
    pos += 1;

    let mut hashes = 0usize;
    while pos < bytes.len() && bytes[pos] == b'#' {
        hashes += 1;
        pos += 1;
    }

    if pos >= bytes.len() || bytes[pos] != b'"' {
        return None;
    }
    pos += 1; // skip opening "

    let content_start = pos;

    // Build the closing delimiter: " followed by `hashes` #s
    let mut closing = vec![b'"'];
    closing.extend(std::iter::repeat_n(b'#', hashes));

    loop {
        if pos + closing.len() > bytes.len() {
            return None;
        }
        if &bytes[pos..pos + closing.len()] == closing.as_slice() {
            let content = &src[content_start..pos];
            return Some((content.to_string(), pos + closing.len()));
        }
        pos += 1;
    }
}

fn parse_regular_string(src: &str, start: usize) -> Option<(String, usize)> {
    let bytes = src.as_bytes();
    if start >= bytes.len() || bytes[start] != b'"' {
        return None;
    }
    let mut pos = start + 1;
    let mut result = String::new();
    while pos < bytes.len() {
        match bytes[pos] {
            b'\\' => {
                pos += 1;
                if pos < bytes.len() {
                    match bytes[pos] {
                        b'n' => result.push('\n'),
                        b't' => result.push('\t'),
                        b'\\' => result.push('\\'),
                        b'"' => result.push('"'),
                        c => {
                            result.push('\\');
                            result.push(c as char);
                        }
                    }
                }
            }
            b'"' => return Some((result, pos + 1)),
            c => result.push(c as char),
        }
        pos += 1;
    }
    None
}

fn parse_any_string(src: &str, pos: usize) -> Option<(String, usize)> {
    let bytes = src.as_bytes();
    if pos >= bytes.len() {
        return None;
    }
    if bytes[pos] == b'r'
        && pos + 1 < bytes.len()
        && (bytes[pos + 1] == b'#' || bytes[pos + 1] == b'"')
    {
        parse_raw_string(src, pos)
    } else if bytes[pos] == b'"' {
        parse_regular_string(src, pos)
    } else {
        None
    }
}

fn skip_whitespace(src: &str, mut pos: usize) -> usize {
    let bytes = src.as_bytes();
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

// ---------------------------------------------------------------------------
// Code block extraction from .rs source files
// ---------------------------------------------------------------------------

struct ExtractedBlock {
    code: String,
    annotation: &'static str,
}

fn annotation_for_fn(name: &str) -> Option<&'static str> {
    match name {
        "code_block" | "code_block_title" => Some(""),
        "code_block_no_run" | "code_block_title_no_run" => Some(",no_run"),
        "code_block_ignore" => Some(",ignore"),
        "code_block_compile_fail" => Some(",compile_fail"),
        _ => None,
    }
}

fn has_title(name: &str) -> bool {
    name == "code_block_title" || name == "code_block_title_no_run"
}

fn extract_rust_code_blocks(source: &str) -> Vec<ExtractedBlock> {
    let mut results = Vec::new();
    let fn_names = [
        "code_block_title_no_run",
        "code_block_compile_fail",
        "code_block_no_run",
        "code_block_ignore",
        "code_block_title",
        "code_block",
    ];

    let mut search_start = 0;
    loop {
        // Find the earliest occurrence of any code_block function call
        let mut earliest: Option<(usize, &str)> = None;
        for name in &fn_names {
            if let Some(idx) = source[search_start..].find(name) {
                let abs = search_start + idx;
                // Make sure it's a real call: preceded by a non-alphanumeric char (or start)
                // and followed by (
                let ok_before = abs == 0
                    || !source.as_bytes()[abs - 1].is_ascii_alphanumeric()
                        && source.as_bytes()[abs - 1] != b'_';
                let after = abs + name.len();
                let ok_after = after < source.len() && source.as_bytes()[after] == b'(';
                if ok_before && ok_after && (earliest.is_none() || abs < earliest.unwrap().0) {
                    earliest = Some((abs, name));
                }
            }
        }

        let Some((idx, fn_name)) = earliest else {
            break;
        };

        let annotation = annotation_for_fn(fn_name).unwrap();
        let pos = idx + fn_name.len() + 1; // skip past (

        // Parse first arg: language string
        let pos = skip_whitespace(source, pos);
        let Some((lang, pos)) = parse_any_string(source, pos) else {
            search_start = idx + fn_name.len();
            continue;
        };

        if lang != "rust" {
            search_start = pos;
            continue;
        }

        // Skip comma
        let pos = skip_whitespace(source, pos);
        if pos >= source.len() || source.as_bytes()[pos] != b',' {
            search_start = pos;
            continue;
        }
        let pos = skip_whitespace(source, pos + 1);

        // If this is a title variant, skip the title string + comma
        let pos = if has_title(fn_name) {
            // Title might be a string or a format!() call - skip string
            if let Some((_title, pos)) = parse_any_string(source, pos) {
                let pos = skip_whitespace(source, pos);
                if pos >= source.len() || source.as_bytes()[pos] != b',' {
                    search_start = pos;
                    continue;
                }
                skip_whitespace(source, pos + 1)
            } else {
                // Title is a format!() or other expression - skip this block
                search_start = pos;
                continue;
            }
        } else {
            pos
        };

        // Parse code argument (the last string before closing paren)
        // It might be wrapped in format!() - skip those
        if pos < source.len() && source[pos..].starts_with("format!") {
            search_start = pos + 7;
            continue;
        }

        if let Some((code, end)) = parse_any_string(source, pos) {
            results.push(ExtractedBlock { code, annotation });
            search_start = end;
        } else {
            search_start = pos;
        }
    }

    results
}

// ---------------------------------------------------------------------------
// Markdown generation from extracted blocks
// ---------------------------------------------------------------------------

fn blocks_to_markdown(blocks: &[ExtractedBlock]) -> String {
    let mut md = String::new();
    for block in blocks {
        let cleaned = doc_test_util::strip_highlight_comments(&block.code);
        md.push_str(&format!("```rust{}\n", block.annotation));
        md.push_str(cleaned.trim_end_matches('\n'));
        md.push('\n');
        md.push_str("```\n\n");
    }
    md
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn inner_main() -> doc_test_util::Result {
    let home = env::var("CARGO_MANIFEST_DIR")?;
    let pages_dir = format!("{home}/../../yew-rs/docs/src/pages");
    let pattern = format!("{pages_dir}/**/*.rs");
    let base = Path::new(&pages_dir).canonicalize()?;

    // Watch for changes
    let dir_pattern = format!("{pages_dir}/**");
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

        let source = fs::read_to_string(&path)?;
        let blocks = extract_rust_code_blocks(&source);

        if blocks.is_empty() {
            continue;
        }

        let test_file = if doc_test_util::should_combine_rs(&path)? {
            let refs: Vec<&str> = blocks.iter().map(|b| b.code.as_str()).collect();
            let code = doc_test_util::combine_blocks(&refs)?;
            TestFile::Combined { stem, code }
        } else {
            let markdown = blocks_to_markdown(&blocks);
            TestFile::GeneratedMarkdown { stem, markdown }
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
