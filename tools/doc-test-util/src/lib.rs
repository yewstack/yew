use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub type Result<T = ()> = core::result::Result<T, Box<dyn Error + 'static>>;

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

// ---------------------------------------------------------------------------
// Combine-marker detection
// ---------------------------------------------------------------------------

pub fn should_combine_mdx(path: &Path) -> io::Result<bool> {
    should_combine_impl(path, b"<!-- COMBINE CODE BLOCKS -->")
}

pub fn should_combine_rs(path: &Path) -> io::Result<bool> {
    should_combine_impl(path, b"// COMBINE CODE BLOCKS")
}

fn should_combine_impl(path: &Path, flag: &[u8]) -> io::Result<bool> {
    let buf_len = flag.len() + 4;
    let mut file = File::open(path)?;
    match file.seek(SeekFrom::End(-(buf_len as i64))) {
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::InvalidInput => return Ok(false),
        Err(e) => return Err(e),
    }
    let mut buf = vec![0u8; buf_len];
    file.read_exact(&mut buf)?;
    Ok(buf.trim_ascii_end().ends_with(flag))
}

// ---------------------------------------------------------------------------
// Diff application
// ---------------------------------------------------------------------------

pub fn apply_diff(src: &mut String, preamble: &str, added: &str, removed: &str) -> Result {
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

// ---------------------------------------------------------------------------
// Code block combination (shared logic for +/- diffs, `// ...` reset)
// ---------------------------------------------------------------------------

pub fn combine_blocks(blocks: &[&str]) -> Result<String> {
    let mut res = String::new();

    for block in blocks {
        let mut preamble = String::new();
        let mut added = String::new();
        let mut removed = String::new();
        let mut diff_applied = false;

        for line in block.lines() {
            if let Some(rest) = line.strip_prefix('+') {
                if rest.starts_with(char::is_whitespace) {
                    added += " ";
                }
                added += rest;
                added += "\n";
            } else if let Some(rest) = line.strip_prefix('-') {
                if rest.starts_with(char::is_whitespace) {
                    removed += " ";
                }
                removed += rest;
                removed += "\n";
            } else if line.trim_ascii() == "// ..." {
                preamble.clear();
            } else {
                if !added.is_empty() || !removed.is_empty() {
                    diff_applied = true;
                    apply_diff(&mut res, &preamble, &added, &removed)?;
                    preamble += &added;
                    added.clear();
                    removed.clear();
                }
                preamble += line;
                preamble += "\n";
            }
        }

        if !added.is_empty() || !removed.is_empty() {
            apply_diff(&mut res, &preamble, &added, &removed)?;
        } else if !diff_applied {
            res += &preamble;
        }
    }

    Ok(res)
}

// ---------------------------------------------------------------------------
// Highlight-comment stripping (shared between rendering and test generation)
// ---------------------------------------------------------------------------

pub fn strip_highlight_comments(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed == "// highlight-next-line"
            || trimmed == "//highlight-next-line"
            || trimmed == "// highlight-start"
            || trimmed == "//highlight-start"
            || trimmed == "// highlight-end"
            || trimmed == "//highlight-end"
        {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

// ---------------------------------------------------------------------------
// Test-file output & module hierarchy
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum TestFile {
    IncludeStr { stem: String, path: PathBuf },
    GeneratedMarkdown { stem: String, markdown: String },
    Combined { stem: String, code: String },
}

impl TestFile {
    pub fn stem(&self) -> &str {
        match self {
            TestFile::IncludeStr { stem, .. }
            | TestFile::GeneratedMarkdown { stem, .. }
            | TestFile::Combined { stem, .. } => stem,
        }
    }
}

#[derive(Debug, Default)]
pub struct Level {
    pub nested: HashMap<String, Level>,
    pub files: Vec<TestFile>,
}

impl Level {
    pub fn insert_file(&mut self, test_file: TestFile, rel: &[&str]) {
        if rel.len() == 1 {
            self.files.push(test_file);
        } else {
            let nested = self.nested.entry(rel[0].to_string()).or_default();
            nested.insert_file(test_file, &rel[1..]);
        }
    }

    pub fn to_contents(&self) -> Result<String> {
        let mut dst = String::new();
        self.write_inner(&mut dst, 0)?;
        Ok(dst)
    }

    pub fn write_into(&self, dst: &mut String, name: &str, level: usize) -> Result {
        write_space(dst, level);
        let name = name.replace(['-', '.'], "_");
        writeln!(dst, "pub mod {name} {{")?;

        self.write_inner(dst, level + 1)?;

        write_space(dst, level);
        writeln!(dst, "}}")?;

        Ok(())
    }

    fn write_inner(&self, dst: &mut String, level: usize) -> Result {
        for (name, nested) in &self.nested {
            nested.write_into(dst, name, level)?;
        }

        for file in &self.files {
            let stem = file.stem();
            match file {
                TestFile::IncludeStr { path, .. } => {
                    write_space(dst, level);
                    writeln!(dst, "#[doc = include_str!(r\"{}\")]", path.display())?;
                }
                TestFile::GeneratedMarkdown { markdown, .. } => {
                    for line in markdown.lines() {
                        write_space(dst, level);
                        writeln!(dst, "/// {line}")?;
                    }
                }
                TestFile::Combined { code, .. } => {
                    write_space(dst, level);
                    writeln!(dst, "/// ```rust, no_run")?;
                    for line in code.lines() {
                        write_space(dst, level);
                        writeln!(dst, "/// {line}")?;
                    }
                    write_space(dst, level);
                    writeln!(dst, "/// ```")?;
                }
            }
            write_space(dst, level);
            writeln!(dst, "pub fn {stem}_md() {{}}")?;
        }

        Ok(())
    }
}

fn write_space(dst: &mut String, level: usize) {
    for _ in 0..level {
        dst.push_str("    ");
    }
}
