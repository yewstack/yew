use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use glob::glob;

type Result<T = ()> = core::result::Result<T, Box<dyn Error + 'static>>;

macro_rules! e {
    ($($fmt:tt),* $(,)?) => {
        return Err(format!($($fmt),*).into())
    };
}

macro_rules! assert {
    ($condition:expr, $($fmt:tt),* $(,)?) => {
        if !$condition { e!($($fmt),*) }
    };
}

#[derive(Debug, Default)]
struct Level {
    nested: HashMap<String, Level>,
    files: Vec<PathBuf>,
}

fn should_combine_code_blocks(path: &Path) -> io::Result<bool> {
    const FLAG: &[u8] = b"<!-- COMBINE CODE BLOCKS -->";

    let mut file = File::open(path)?;
    match file.seek(SeekFrom::End(-32)) {
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::InvalidInput => return Ok(false),
        Err(e) => return Err(e),
    }
    let mut buf = [0u8; 32];
    file.read_exact(&mut buf)?;
    // TODO: Use trim_ascii_end() when MSRV is updated to 1.80+
    let trimmed = buf
        .iter()
        .rposition(|&b| !b.is_ascii_whitespace())
        .map(|i| &buf[..=i])
        .unwrap_or(&buf[..0]);
    Ok(trimmed.ends_with(FLAG))
}

fn apply_diff(src: &mut String, preamble: &str, added: &str, removed: &str) -> Result {
    assert!(
        !preamble.is_empty() || !removed.is_empty(),
        "Failure on applying a diff: \nNo preamble or text to remove provided, unable to find \
         location to insert:\n{added}\nIn the following text:\n{src}",
    );

    let mut matches = src
        .match_indices(preamble)
        .filter_map(|(chunk_start, chunk)| {
            let removed_start = chunk_start + chunk.len();
            let removed_end = removed_start + removed.len();
            src.get(removed_start..removed_end)
                .eq(&Some(removed))
                .then_some((removed_start, removed_end))
        });

    let Some((removed_start, removed_end)) = matches.next() else {
        e!(
            "Failure on applying a diff: \nCouldn't find the following preamble:\n{preamble}\nIn \
             the following text:\n{src}\nWhile trying to remove the following \
             text:\n{removed}\nAnd add the following:\n{added}"
        )
    };

    assert!(
        matches.next().is_none(),
        "Failure on applying a diff: \nAmbiguous preamble:\n{preamble}\nIn the following \
         text:\n{src}\nWhile trying to remove the following text:\n{removed}\nAnd add the \
         following:\n{added}"
    );

    src.replace_range(removed_start..removed_end, added);
    Ok(())
}

fn combined_code_blocks(path: &Path) -> Result<String> {
    let file = BufReader::new(File::open(path)?);
    let mut res = String::new();

    let mut err = Ok(());
    let mut lines = file
        .lines()
        .filter_map(|i| i.map_err(|e| err = Err(e)).ok());
    while let Some(line) = lines.next() {
        if !line.starts_with("```rust") {
            continue;
        }

        let mut preamble = String::new();
        let mut added = String::new();
        let mut removed = String::new();
        let mut diff_applied = false;
        for line in &mut lines {
            if line.starts_with("```") {
                if !added.is_empty() || !removed.is_empty() {
                    apply_diff(&mut res, &preamble, &added, &removed)?;
                } else if !diff_applied {
                    // if no diff markers were found, just add the contents
                    res += &preamble;
                }
                break;
            } else if let Some(line) = line.strip_prefix('+') {
                if line.starts_with(char::is_whitespace) {
                    added += " ";
                }
                added += line;
                added += "\n";
            } else if let Some(line) = line.strip_prefix('-') {
                if line.starts_with(char::is_whitespace) {
                    removed += " ";
                }
                removed += line;
                removed += "\n";
            // TODO: Use trim_ascii() when MSRV is updated to 1.80+
            } else if line.trim() == "// ..." {
                // disregard the preamble
                preamble.clear();
            } else {
                if !added.is_empty() || !removed.is_empty() {
                    diff_applied = true;
                    apply_diff(&mut res, &preamble, &added, &removed)?;
                    preamble += &added;
                    added.clear();
                    removed.clear();
                }
                preamble += &line;
                preamble += "\n";
            }
        }
    }

    Ok(res)
}

impl Level {
    fn insert(&mut self, path: PathBuf, rel: &[&str]) {
        if rel.len() == 1 {
            self.files.push(path);
        } else {
            let nested = self.nested.entry(rel[0].to_string()).or_default();
            nested.insert(path, &rel[1..]);
        }
    }

    fn to_contents(&self) -> Result<String> {
        let mut dst = String::new();

        self.write_inner(&mut dst, 0)?;
        Ok(dst)
    }

    fn write_into(&self, dst: &mut String, name: &str, level: usize) -> Result {
        self.write_space(dst, level);
        let name = name.replace(['-', '.'], "_");
        writeln!(dst, "pub mod {name} {{")?;

        self.write_inner(dst, level + 1)?;

        self.write_space(dst, level);
        writeln!(dst, "}}")?;

        Ok(())
    }

    fn write_inner(&self, dst: &mut String, level: usize) -> Result {
        for (name, nested) in &self.nested {
            nested.write_into(dst, name, level)?;
        }

        for file in &self.files {
            let stem = file
                .file_stem()
                .ok_or_else(|| format!("no filename in path {file:?}"))?
                .to_str()
                .ok_or_else(|| format!("non-UTF8 path: {file:?}"))?
                .replace('-', "_");

            if should_combine_code_blocks(file)? {
                let res = combined_code_blocks(file)?;
                self.write_space(dst, level);
                writeln!(dst, "/// ```rust, no_run")?;
                for line in res.lines() {
                    self.write_space(dst, level);
                    writeln!(dst, "/// {line}")?;
                }
                self.write_space(dst, level);
                writeln!(dst, "/// ```")?;
            } else {
                self.write_space(dst, level);
                writeln!(dst, "#[doc = include_str!(r\"{}\")]", file.display())?;
            }
            self.write_space(dst, level);
            writeln!(dst, "pub fn {stem}_md() {{}}")?;
        }

        Ok(())
    }

    fn write_space(&self, dst: &mut String, level: usize) {
        for _ in 0..level {
            dst.push_str("    ");
        }
    }
}

fn inner_main() -> Result {
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

        level.insert(path.clone(), &parts[..]);
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
