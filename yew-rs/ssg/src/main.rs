use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use clap::Parser;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

fn file_hash(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let mut hasher = DefaultHasher::new();
    hasher.write(&bytes);
    Ok(format!("{:016x}", hasher.finish()))
}

const BASE_URL: &str = "https://yew.rs";
const GA_MEASUREMENT_ID: &str = "G-DENCL8P4YP";

fn derive_locale(url_path: &str) -> &str {
    if url_path.starts_with("/ja/") || url_path == "/ja" {
        "ja"
    } else if url_path.starts_with("/zh-Hans/") || url_path == "/zh-Hans" {
        "zh-Hans"
    } else if url_path.starts_with("/zh-Hant/") || url_path == "/zh-Hant" {
        "zh-Hant"
    } else {
        "en"
    }
}

fn locale_to_og(locale: &str) -> &str {
    match locale {
        "zh-Hans" => "zh_Hans",
        "zh-Hant" => "zh_Hant",
        _ => locale,
    }
}

fn strip_locale_prefix(url_path: &str) -> &str {
    for prefix in &["/ja/", "/zh-Hans/", "/zh-Hant/"] {
        if let Some(rest) = url_path.strip_prefix(prefix) {
            return rest;
        }
    }
    url_path
}

fn derive_doc_version(url_path: &str) -> Option<&'static str> {
    let path = strip_locale_prefix(url_path);
    let path = path.strip_prefix('/').unwrap_or(path);
    if !path.starts_with("docs") {
        return None;
    }
    if path.starts_with("docs/next/") || path == "docs/next" {
        Some("next")
    } else if path.starts_with("docs/0.22/") {
        Some("0.22")
    } else if path.starts_with("docs/0.21/") {
        Some("0.21")
    } else if path.starts_with("docs/0.20/") {
        Some("0.20")
    } else {
        Some("0.23")
    }
}

fn is_localizable(base_path: &str) -> bool {
    if base_path.starts_with("/docs") || base_path.starts_with("/tutorial") {
        return true;
    }
    let trimmed = base_path.trim_matches('/');
    if trimmed.is_empty() {
        return true;
    }
    let parts: Vec<&str> = trimmed.split('/').collect();
    matches!(
        parts.as_slice(),
        ["next"]
            | ["0.22"]
            | ["0.21"]
            | ["0.20"]
            | ["next", "tutorial"]
            | ["0.22", "tutorial"]
            | ["0.21", "tutorial"]
            | ["0.20", "tutorial"]
    )
}

fn generate_hreflang_tags(url_path: &str) -> String {
    let base_path = strip_locale_prefix(url_path);
    let base_path = if base_path.starts_with('/') {
        base_path.to_string()
    } else {
        format!("/{base_path}")
    };

    if !is_localizable(&base_path) {
        return format!(
            "    <link rel=\"alternate\" href=\"{BASE_URL}{url_path}\" hreflang=\"en\" />\n\x20   \
             <link rel=\"alternate\" href=\"{BASE_URL}{url_path}\" hreflang=\"x-default\" />\n"
        );
    }

    let mut tags = String::new();
    for (lang, prefix) in [
        ("en", ""),
        ("ja", "/ja"),
        ("zh-Hans", "/zh-Hans"),
        ("zh-Hant", "/zh-Hant"),
    ] {
        tags.push_str(&format!(
            "    <link rel=\"alternate\" href=\"{BASE_URL}{prefix}{base_path}\" \
             hreflang=\"{lang}\" />\n"
        ));
    }
    tags.push_str(&format!(
        "    <link rel=\"alternate\" href=\"{BASE_URL}{base_path}\" hreflang=\"x-default\" />\n"
    ));
    tags
}

fn generate_og_locale_tags(url_path: &str) -> String {
    let locale = derive_locale(url_path);
    let og_locale = locale_to_og(locale);
    let mut tags = format!("    <meta property=\"og:locale\" content=\"{og_locale}\" />\n");

    let base_path = strip_locale_prefix(url_path);
    let base_path = if base_path.starts_with('/') {
        base_path
    } else {
        return tags;
    };

    if base_path.starts_with("/docs") {
        for alt in ["en", "ja", "zh_Hans", "zh_Hant"] {
            if alt != og_locale {
                tags.push_str(&format!(
                    "    <meta property=\"og:locale:alternate\" content=\"{alt}\" />\n"
                ));
            }
        }
    }
    tags
}

fn generate_breadcrumb_jsonld(url_path: &str, title: &str) -> String {
    let base_path = strip_locale_prefix(url_path);
    let base_path = base_path.strip_prefix('/').unwrap_or(base_path);
    let segments: Vec<&str> = base_path.split('/').filter(|s| !s.is_empty()).collect();

    if segments.len() <= 1 {
        return String::new();
    }

    let mut items = Vec::new();
    let mut accumulated = String::new();

    for (i, seg) in segments.iter().enumerate() {
        accumulated.push('/');
        accumulated.push_str(seg);

        let name = if i == segments.len() - 1 {
            title.to_string()
        } else {
            seg.split('-')
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        Some(ch) => format!("{}{}", ch.to_uppercase(), c.as_str()),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        };

        let name = name.replace('"', "\\\"");
        items.push(format!(
            r#"{{"@type":"ListItem","position":{},"name":"{}","item":"{BASE_URL}{accumulated}"}}"#,
            i + 1,
            name,
        ));
    }

    format!(
        "    <script type=\"application/ld+json\">{{\"@context\":\"https://schema.org\",\"@type\":\"BreadcrumbList\",\"itemListElement\":[{}]}}</script>\n",
        items.join(",")
    )
}

#[derive(Parser)]
struct Args {
    /// Build wasm crates in release mode (optimized, slower to compile).
    #[arg(long)]
    release: bool,

    #[arg(long)]
    skip_capture: bool,

    #[arg(long)]
    skip_wasm_opt: bool,

    /// After building, serve the output directory on this port.
    /// Example: --serve 8080
    #[arg(long)]
    serve: Option<u16>,

    /// Build all locales (ja, zh-Hans, zh-Hant). Default is English only.
    #[arg(long)]
    all_locales: bool,
}

struct PageBinary {
    bin_name: String,
    url_path: String,
    title: String,
    crate_name: String,
}

// (content_crate_dir, url_prefix, bin_prefix, spa_bin_name, spa_crate_name)
// spa_bin_name: if non-empty, this content crate's pages are served by a SPA binary
const CRATES: &[(&str, &str, &str, &str, &str)] = &[
    ("docs", "docs/next", "", "spa-en", "yew-site-spa-en"),
    ("docs-0-23", "docs", "v0-23-", "spa-en", "yew-site-spa-en"),
    (
        "docs-0-22",
        "docs/0.22",
        "v0-22-",
        "spa-en",
        "yew-site-spa-en",
    ),
    (
        "docs-0-21",
        "docs/0.21",
        "v0-21-",
        "spa-en",
        "yew-site-spa-en",
    ),
    (
        "docs-0-20",
        "docs/0.20",
        "v0-20-",
        "spa-en",
        "yew-site-spa-en",
    ),
    (
        "docs-ja",
        "ja/docs/next",
        "ja-",
        "spa-ja",
        "yew-site-spa-ja",
    ),
    (
        "docs-ja-0-23",
        "ja/docs",
        "ja-0-23-",
        "spa-ja",
        "yew-site-spa-ja",
    ),
    (
        "docs-ja-0-22",
        "ja/docs/0.22",
        "ja-0-22-",
        "spa-ja",
        "yew-site-spa-ja",
    ),
    (
        "docs-ja-0-21",
        "ja/docs/0.21",
        "ja-0-21-",
        "spa-ja",
        "yew-site-spa-ja",
    ),
    (
        "docs-ja-0-20",
        "ja/docs/0.20",
        "ja-0-20-",
        "spa-ja",
        "yew-site-spa-ja",
    ),
    (
        "docs-zh-hans",
        "zh-Hans/docs/next",
        "zh-hans-",
        "spa-zh-hans",
        "yew-site-spa-zh-hans",
    ),
    (
        "docs-zh-hans-0-23",
        "zh-Hans/docs",
        "zh-hans-0-23-",
        "spa-zh-hans",
        "yew-site-spa-zh-hans",
    ),
    (
        "docs-zh-hans-0-22",
        "zh-Hans/docs/0.22",
        "zh-hans-0-22-",
        "spa-zh-hans",
        "yew-site-spa-zh-hans",
    ),
    (
        "docs-zh-hans-0-21",
        "zh-Hans/docs/0.21",
        "zh-hans-0-21-",
        "spa-zh-hans",
        "yew-site-spa-zh-hans",
    ),
    (
        "docs-zh-hans-0-20",
        "zh-Hans/docs/0.20",
        "zh-hans-0-20-",
        "spa-zh-hans",
        "yew-site-spa-zh-hans",
    ),
    (
        "docs-zh-hant",
        "zh-Hant/docs/next",
        "zh-hant-",
        "spa-zh-hant",
        "yew-site-spa-zh-hant",
    ),
    (
        "docs-zh-hant-0-23",
        "zh-Hant/docs",
        "zh-hant-0-23-",
        "spa-zh-hant",
        "yew-site-spa-zh-hant",
    ),
    (
        "docs-zh-hant-0-22",
        "zh-Hant/docs/0.22",
        "zh-hant-0-22-",
        "spa-zh-hant",
        "yew-site-spa-zh-hant",
    ),
    (
        "docs-zh-hant-0-21",
        "zh-Hant/docs/0.21",
        "zh-hant-0-21-",
        "spa-zh-hant",
        "yew-site-spa-zh-hant",
    ),
    (
        "docs-zh-hant-0-20",
        "zh-Hant/docs/0.20",
        "zh-hant-0-20-",
        "spa-zh-hant",
        "yew-site-spa-zh-hant",
    ),
    ("community", "community", "", "", "yew-site-community"),
];

fn discover_pages(source_dir: &Path) -> Result<Vec<PageBinary>> {
    let home = "yew-site-home".to_string();

    let home_spa_langs: &[(&str, &str, &str)] = &[
        ("", "spa-en", "yew-site-spa-en"),
        ("ja", "spa-ja", "yew-site-spa-ja"),
        ("zh-Hans", "spa-zh-hans", "yew-site-spa-zh-hans"),
        ("zh-Hant", "spa-zh-hant", "yew-site-spa-zh-hant"),
    ];

    let home_versions: &[&str] = &["next", "0.22", "0.21", "0.20"];

    let mut pages = Vec::new();

    for &(lang, spa_bin, spa_crate) in home_spa_langs {
        let url_path = if lang.is_empty() {
            "/".to_string()
        } else {
            format!("/{lang}/")
        };
        pages.push(PageBinary {
            bin_name: spa_bin.to_string(),
            url_path,
            title: "Yew".to_string(),
            crate_name: spa_crate.to_string(),
        });

        for version_slug in home_versions {
            let lang_prefix = if lang.is_empty() {
                String::new()
            } else {
                format!("/{lang}")
            };
            let url_path = format!("{lang_prefix}/{version_slug}/");
            pages.push(PageBinary {
                bin_name: spa_bin.to_string(),
                url_path,
                title: "Yew".to_string(),
                crate_name: spa_crate.to_string(),
            });
        }
    }

    let tutorial_versions: &[&str] = &["", "next", "0.22", "0.21", "0.20"];
    for &(lang, spa_bin, spa_crate) in home_spa_langs {
        for version_slug in tutorial_versions {
            let lang_prefix = if lang.is_empty() {
                String::new()
            } else {
                format!("/{lang}")
            };
            let url_path = if version_slug.is_empty() {
                format!("{lang_prefix}/tutorial")
            } else {
                format!("{lang_prefix}/{version_slug}/tutorial")
            };
            pages.push(PageBinary {
                bin_name: spa_bin.to_string(),
                url_path,
                title: "Tutorial".to_string(),
                crate_name: spa_crate.to_string(),
            });
        }
    }

    pages.push(PageBinary {
        bin_name: "search".to_string(),
        url_path: "/search".to_string(),
        title: "Search".to_string(),
        crate_name: home.clone(),
    });
    pages.push(PageBinary {
        bin_name: "not-found".to_string(),
        url_path: "/404".to_string(),
        title: "Page Not Found".to_string(),
        crate_name: home,
    });

    for &(crate_name, url_prefix, bin_prefix, spa_bin, spa_crate) in CRATES {
        let crate_dir = source_dir.join(crate_name);
        let pages_dir = crate_dir.join("src/pages");
        if !pages_dir.exists() {
            continue;
        }

        let is_spa = !spa_bin.is_empty();
        let effective_crate_name = if is_spa {
            spa_crate.to_string()
        } else {
            format!("yew-site-{crate_name}")
        };

        let mut page_files = Vec::new();
        collect_page_files(&pages_dir, &pages_dir, &mut page_files);

        for rel_path in page_files {
            let mut url_segments: Vec<String> = rel_path
                .iter()
                .map(|s| s.to_string_lossy().replace('_', "-"))
                .collect();
            if url_segments.first().is_some_and(|s| s == "tutorial") {
                continue;
            }
            if url_segments.last().is_some_and(|s| s == "introduction") {
                url_segments.pop();
            }
            let is_migration_guide = url_segments
                .first()
                .is_some_and(|s| s == "migration-guides");
            let effective_prefix = if is_migration_guide {
                url_prefix
                    .replace("/next", "")
                    .replace("next/", "")
                    .replace("next", "docs")
            } else {
                url_prefix.to_string()
            };
            let url_path = if url_segments.is_empty() {
                format!("/{}", effective_prefix)
            } else {
                format!("/{}/{}", effective_prefix, url_segments.join("/"))
            };
            let bin_name = if is_spa {
                spa_bin.to_string()
            } else {
                format!(
                    "{}{}",
                    bin_prefix,
                    rel_path
                        .iter()
                        .map(|s| s.to_string_lossy().replace('_', "-"))
                        .collect::<Vec<_>>()
                        .join("-")
                )
            };
            let title = url_segments
                .last()
                .map(|s| {
                    s.split('-')
                        .map(|w| {
                            let mut c = w.chars();
                            match c.next() {
                                Some(ch) => format!("{}{}", ch.to_uppercase(), c.as_str()),
                                None => String::new(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();

            pages.push(PageBinary {
                bin_name,
                url_path,
                title,
                crate_name: effective_crate_name.clone(),
            });
        }
    }

    const BLOG_PAGES: &[(&str, &str, &str)] = &[
        ("blog-index", "/blog", "Blog"),
        ("hello-yew", "/blog/2022/01/20/hello-yew", "Hello Yew"),
        (
            "release-0-20",
            "/blog/2022/11/24/release-0-20",
            "Releasing Yew 0.20",
        ),
        (
            "release-0-21",
            "/blog/2023/09/23/release-0-21",
            "Announcing Yew 0.21",
        ),
        (
            "release-0-22",
            "/blog/2024/10/14/release-0-22",
            "Announcing Yew 0.22",
        ),
        (
            "release-0-22-1",
            "/blog/2025/11/29/release-0-22",
            "Yew 0.22 - For Real This Time",
        ),
    ];

    for &(bin_name, url_path, title) in BLOG_PAGES {
        pages.push(PageBinary {
            bin_name: bin_name.to_string(),
            url_path: url_path.to_string(),
            title: title.to_string(),
            crate_name: "yew-site-blog".to_string(),
        });
    }

    Ok(pages)
}

fn collect_page_files(base: &Path, dir: &Path, results: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let mod_rs = path.join("mod.rs");
            if mod_rs.exists() {
                let has_page = std::fs::read_to_string(&mod_rs)
                    .map(|c| {
                        c.contains("pub fn Page")
                            || c.contains("doc_page!")
                            || c.contains("blog_page!")
                            || c.contains("community_page!")
                            || c.contains("simple_page!")
                    })
                    .unwrap_or(false);
                if has_page {
                    let rel = path.strip_prefix(base).unwrap();
                    results.push(rel.to_path_buf());
                }
            }
            collect_page_files(base, &path, results);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let name = path.file_stem().unwrap().to_string_lossy();
            if name != "mod" {
                let rel = path.strip_prefix(base).unwrap().with_extension("");
                results.push(rel);
            }
        }
    }
}

fn cargo_build_all(source_dir: &Path, pages: &[PageBinary], release: bool) -> Result<()> {
    let mut crate_names: Vec<&str> = pages.iter().map(|p| p.crate_name.as_str()).collect();
    crate_names.sort();
    crate_names.dedup();

    let mut args = vec![
        "build".to_string(),
        "--target".to_string(),
        "wasm32-unknown-unknown".to_string(),
    ];
    if release {
        args.push("--release".to_string());
    }
    for name in &crate_names {
        args.push("-p".to_string());
        args.push(name.to_string());
    }

    println!("Compiling {} crates...", crate_names.len());
    let mut cmd = Command::new("cargo");
    cmd.args(&args).current_dir(source_dir.join(".."));
    if release {
        cmd.env("CARGO_PROFILE_RELEASE_CODEGEN_UNITS", "1");
    }
    let status = cmd.status().context("Failed to run cargo build")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    Ok(())
}

struct ProcessedBinary {
    js_hashed_name: String,
    wasm_hashed_name: String,
    js_content: String,
    wasm_source: PathBuf,
}

fn process_binary(
    bin_name: &str,
    staging_dir: &Path,
    target_dir: &Path,
    release: bool,
    skip_wasm_opt: bool,
) -> Result<ProcessedBinary> {
    std::fs::create_dir_all(staging_dir)?;

    let profile = if release { "release" } else { "debug" };
    let wasm_input = target_dir
        .join(format!("wasm32-unknown-unknown/{profile}"))
        .join(format!("{bin_name}.wasm"));

    if !wasm_input.exists() {
        bail!("WASM binary not found: {}", wasm_input.display());
    }

    let bindgen_out = Command::new("wasm-bindgen")
        .args([
            "--target",
            "web",
            "--no-typescript",
            "--out-dir",
            &staging_dir.display().to_string(),
            &wasm_input.display().to_string(),
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .context("Failed to run wasm-bindgen")?;

    if !bindgen_out.status.success() {
        bail!(
            "wasm-bindgen failed for {bin_name}:\n{}",
            String::from_utf8_lossy(&bindgen_out.stderr)
        );
    }

    let wasm_bg = staging_dir.join(format!("{bin_name}_bg.wasm"));
    if !skip_wasm_opt && wasm_bg.exists() {
        let opt_out = Command::new("wasm-opt")
            .args(["-Oz", "-o"])
            .arg(&wasm_bg)
            .arg(&wasm_bg)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .context("Failed to run wasm-opt")?;

        if !opt_out.status.success() {
            bail!(
                "wasm-opt failed for {bin_name}:\n{}",
                String::from_utf8_lossy(&opt_out.stderr)
            );
        }
    }

    let wasm_hash = file_hash(&wasm_bg)?;
    let wasm_hashed_name = format!("{bin_name}_bg-{wasm_hash}.wasm");
    let wasm_final = staging_dir.join(&wasm_hashed_name);
    std::fs::rename(&wasm_bg, &wasm_final)?;

    let js_path = staging_dir.join(format!("{bin_name}.js"));
    let js_content = std::fs::read_to_string(&js_path)?;
    let js_content = js_content.replace(&format!("{bin_name}_bg.wasm"), &wasm_hashed_name);
    let js_hash = {
        let mut hasher = DefaultHasher::new();
        hasher.write(js_content.as_bytes());
        format!("{:016x}", hasher.finish())
    };
    let js_hashed_name = format!("{bin_name}-{js_hash}.js");
    std::fs::remove_file(&js_path)?;

    Ok(ProcessedBinary {
        js_hashed_name,
        wasm_hashed_name,
        js_content,
        wasm_source: wasm_final,
    })
}

fn emit_page_files(page: &PageBinary, output_dir: &Path, pb: &ProcessedBinary) -> Result<()> {
    let page_output = if page.url_path == "/" {
        output_dir.to_path_buf()
    } else {
        output_dir.join(page.url_path.trim_start_matches('/'))
    };
    std::fs::create_dir_all(&page_output)?;

    let js_dest = page_output.join(&pb.js_hashed_name);
    std::fs::write(&js_dest, &pb.js_content)?;

    let wasm_dest = page_output.join(&pb.wasm_hashed_name);
    if std::fs::hard_link(&pb.wasm_source, &wasm_dest).is_err() {
        std::fs::copy(&pb.wasm_source, &wasm_dest)?;
    }

    let public_url = if page.url_path == "/" {
        String::new()
    } else {
        page.url_path.clone()
    };

    let locale = derive_locale(&page.url_path);
    let lang = locale;
    let canonical_url = format!("{BASE_URL}{}", page.url_path);
    let display_title = if page.title.is_empty() || page.title == "Yew" {
        "Yew".to_string()
    } else {
        format!("{} | Yew", page.title)
    };
    let og_locale_tags = generate_og_locale_tags(&page.url_path);
    let hreflang_tags = generate_hreflang_tags(&page.url_path);
    let breadcrumb_jsonld = generate_breadcrumb_jsonld(&page.url_path, &page.title);
    let docsearch_meta = derive_doc_version(&page.url_path)
        .map(|v| {
            format!(
                "    <meta name=\"docsearch:language\" content=\"{locale}\" />\n\x20   <meta \
                 name=\"docsearch:version\" content=\"{v}\" />\n"
            )
        })
        .unwrap_or_default();

    let ga_id = GA_MEASUREMENT_ID;
    let js_hashed_name = &pb.js_hashed_name;
    let wasm_hashed_name = &pb.wasm_hashed_name;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" dir="ltr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{display_title}</title>
    <meta property="og:title" content="{display_title}" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta property="og:url" content="{canonical_url}" />
{og_locale_tags}{docsearch_meta}    <link rel="icon" href="/img/logo.svg" />
    <link rel="canonical" href="{canonical_url}" />
{hreflang_tags}    <link rel="preconnect" href="https://F8S2ICRD2T-dsn.algolia.net" crossorigin="anonymous" />
{breadcrumb_jsonld}    <link rel="alternate" type="application/rss+xml" href="/blog/rss.xml" title="Yew RSS Feed" />
    <link rel="alternate" type="application/atom+xml" href="/blog/atom.xml" title="Yew Atom Feed" />
    <link rel="preconnect" href="https://www.google-analytics.com" />
    <link rel="preconnect" href="https://www.googletagmanager.com" />
    <script async src="https://www.googletagmanager.com/gtag/js?id={ga_id}"></script>
    <script>function gtag(){{dataLayer.push(arguments)}}window.dataLayer=window.dataLayer||[],gtag("js",new Date),gtag("config","{ga_id}",{{anonymize_ip:!0}})</script>
    <link rel="search" type="application/opensearchdescription+xml" title="Yew" href="/opensearch.xml" />
    <script>
(function(){{var c=localStorage.getItem('theme')||'system',d=document.documentElement;function a(v){{var t=v==='system'?(matchMedia('(prefers-color-scheme:dark)').matches?'dark':'light'):v;d.setAttribute('data-theme',t);d.setAttribute('data-theme-choice',v)}}a(c);matchMedia('(prefers-color-scheme:dark)').addEventListener('change',function(){{var s=localStorage.getItem('theme');if(!s||s==='system')a('system')}});}})();
    </script>
    <script type="module">
import init, * as bindings from '{public_url}/{js_hashed_name}';
const wasm = await init({{ module_or_path: '{public_url}/{wasm_hashed_name}' }});
window.wasmBindings = bindings;
dispatchEvent(new CustomEvent("TrunkApplicationStarted", {{detail: {{wasm}}}}));
    </script>
    <link rel="modulepreload" href="{public_url}/{js_hashed_name}" />
    <link rel="preload" href="{public_url}/{wasm_hashed_name}" as="fetch" type="application/wasm" crossorigin />
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@docsearch/css@3" />
</head>
<body></body>
</html>"#,
    );

    std::fs::write(page_output.join("index.html"), html)?;
    Ok(())
}

fn build_pages_parallel(
    pages: &[PageBinary],
    output_dir: &Path,
    target_dir: &Path,
    release: bool,
    skip_wasm_opt: bool,
) -> Result<()> {
    let jobs = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    use std::collections::HashMap;
    use std::sync::Mutex;

    let mut unique_bins: Vec<String> = pages.iter().map(|p| p.bin_name.clone()).collect();
    unique_bins.sort();
    unique_bins.dedup();

    println!(
        "Processing {} unique binaries for {} pages...",
        unique_bins.len(),
        pages.len()
    );

    let staging_base = output_dir.join("_spa_staging");
    let _ = std::fs::remove_dir_all(&staging_base);

    let processed: Mutex<HashMap<String, ProcessedBinary>> = Mutex::new(HashMap::new());
    let counter = Arc::new(AtomicUsize::new(0));
    let next_bin = Arc::new(AtomicUsize::new(0));
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());
    let total_bins = unique_bins.len();

    std::thread::scope(|s| {
        let mut handles = Vec::new();

        for _ in 0..jobs.min(total_bins) {
            let counter = counter.clone();
            let next_bin = next_bin.clone();
            let errors = &errors;
            let processed = &processed;
            let staging_base = &staging_base;
            let unique_bins = &unique_bins;

            handles.push(s.spawn(move || loop {
                let idx = next_bin.fetch_add(1, Ordering::Relaxed);
                if idx >= unique_bins.len() {
                    break;
                }
                let bin_name = &unique_bins[idx];
                let n = counter.fetch_add(1, Ordering::Relaxed) + 1;
                println!("[{n}/{total_bins}] Processing binary: {bin_name}");

                let staging_dir = staging_base.join(bin_name);
                match process_binary(bin_name, &staging_dir, target_dir, release, skip_wasm_opt) {
                    Ok(pb) => {
                        processed.lock().unwrap().insert(bin_name.clone(), pb);
                    }
                    Err(e) => {
                        eprintln!("  FAILED: {e}");
                        errors.lock().unwrap().push(format!("{bin_name}: {e}"));
                    }
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    });

    let errs_so_far = errors.lock().unwrap().clone();
    if !errs_so_far.is_empty() {
        bail!("Binary processing failures:\n{}", errs_so_far.join("\n"));
    }

    let processed = processed.into_inner().unwrap();

    println!("Emitting {} page files...", pages.len());
    for (i, page) in pages.iter().enumerate() {
        if let Some(pb) = processed.get(&page.bin_name) {
            if let Err(e) = emit_page_files(page, output_dir, pb) {
                eprintln!(
                    "[{}/{}] FAILED emitting {}: {e}",
                    i + 1,
                    pages.len(),
                    page.url_path
                );
                errors
                    .lock()
                    .unwrap()
                    .push(format!("{}: {e}", page.url_path));
            }
        }
    }

    let _ = std::fs::remove_dir_all(&staging_base);

    let errs = errors.into_inner().unwrap();
    if !errs.is_empty() {
        bail!("Build failures:\n{}", errs.join("\n"));
    }

    Ok(())
}

async fn render_and_inject(output_dir: &Path, all_locales: bool) -> Result<()> {
    let all_rendered = render_all_pages(all_locales).await;

    for page in &all_rendered {
        let rel = page.url.trim_start_matches('/');
        let html_path = if rel.is_empty() {
            output_dir.join("index.html")
        } else {
            output_dir.join(rel).join("index.html")
        };

        if !html_path.exists() {
            println!("Warning: no index.html for {}, skipping", page.url);
            continue;
        }

        println!("Injecting: {}", page.url);
        let original = std::fs::read_to_string(&html_path)?;
        let description = if page.description.is_empty() {
            "A framework for creating reliable and efficient web applications.".to_string()
        } else {
            page.description.clone()
        };
        let escaped_desc = description
            .replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        let mut extra_head = page.styles.clone();
        extra_head.push_str(&format!(
            "    <meta name=\"description\" content=\"{escaped_desc}\" />\n\x20   <meta \
             property=\"og:description\" content=\"{escaped_desc}\" />\n"
        ));
        let updated = inject_styles(&inject_body(&original, &page.body), &extra_head);
        std::fs::write(&html_path, updated)?;
    }

    Ok(())
}

async fn render_all_pages(all_locales: bool) -> Vec<yew_site_lib::RenderedPage> {
    let mut all = Vec::new();
    all.extend(yew_site_spa_en::render_pages().await);
    if all_locales {
        all.extend(yew_site_spa_ja::render_pages().await);
        all.extend(yew_site_spa_zh_hans::render_pages().await);
        all.extend(yew_site_spa_zh_hant::render_pages().await);
    }
    all.extend(yew_site_home::render_search_and_404().await);
    all.extend(yew_site_blog::render_pages().await);
    all.extend(yew_site_community::render_pages().await);
    all
}

fn inject_styles(html: &str, styles: &str) -> String {
    if let Some(pos) = html.find("</head>") {
        format!("{}{styles}{}", &html[..pos], &html[pos..])
    } else {
        html.to_string()
    }
}

fn inject_body(html: &str, body_content: &str) -> String {
    if let Some(body_start) = html.find("<body>") {
        if let Some(body_end) = html.find("</body>") {
            return format!(
                "{}<body>{}</body>{}",
                &html[..body_start],
                body_content,
                &html[body_end + "</body>".len()..]
            );
        }
    }
    html.replace("<body></body>", &format!("<body>{body_content}</body>"))
}

fn generate_sitemap(pages: &[PageBinary], output_dir: &Path, base_url: &str) -> Result<()> {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );

    for page in pages {
        let loc = if page.url_path == "/" {
            base_url.to_string()
        } else {
            format!("{}{}", base_url, page.url_path)
        };
        xml.push_str(&format!(
            "<url><loc>{loc}</loc><changefreq>weekly</changefreq><priority>0.5</priority></url>\n"
        ));
    }

    xml.push_str("</urlset>\n");
    std::fs::write(output_dir.join("sitemap.xml"), xml)?;
    Ok(())
}

fn date_to_rfc2822(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    let (y, m, d): (i32, u32, u32) = (
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap(),
        parts[2].parse().unwrap(),
    );
    let months = [
        "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let a = (14 - m) / 12;
    let y2 = y as u32 + 4800 - a;
    let m2 = m + 12 * a - 3;
    let jdn = d + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045;
    let dow = (jdn % 7) as usize;
    format!(
        "{}, {:02} {} {} 00:00:00 GMT",
        days[dow], d, months[m as usize], y
    )
}

fn generate_blog_feeds(output_dir: &Path) -> Result<()> {
    let blog_dir = output_dir.join("blog");
    std::fs::create_dir_all(&blog_dir)?;

    let posts = yew_site_blog::BLOG_POSTS;

    {
        use rss::{ChannelBuilder, GuidBuilder, ItemBuilder};

        let items: Vec<_> = posts
            .iter()
            .map(|post| {
                let url = format!("{BASE_URL}{}", post.url_path());
                let pub_date = date_to_rfc2822(post.date);
                ItemBuilder::default()
                    .title(post.title.to_string())
                    .link(url.clone())
                    .description(post.description.to_string())
                    .pub_date(pub_date)
                    .guid(GuidBuilder::default().value(url).permalink(true).build())
                    .build()
            })
            .collect();

        let last_build_date = date_to_rfc2822(posts[0].date);

        let channel = ChannelBuilder::default()
            .title("Yew Blog")
            .link(format!("{BASE_URL}/blog"))
            .description("Yew Blog")
            .last_build_date(Some(last_build_date))
            .docs(Some(
                "https://validator.w3.org/feed/docs/rss2.html".to_string(),
            ))
            .language("en".to_string())
            .items(items)
            .build();

        std::fs::write(blog_dir.join("rss.xml"), channel.to_string())?;
    }

    {
        use atom_syndication::{EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder, Text};

        let entries: Vec<_> = posts
            .iter()
            .map(|post| {
                let url = format!("{BASE_URL}{}", post.url_path());
                let updated = format!("{}T00:00:00+00:00", post.date);
                EntryBuilder::default()
                    .title(post.title)
                    .id(url.clone())
                    .updated(updated.parse::<atom_syndication::FixedDateTime>().unwrap())
                    .published(updated.parse::<atom_syndication::FixedDateTime>().ok())
                    .summary(Some(Text::plain(post.description)))
                    .authors(vec![PersonBuilder::default()
                        .name(post.author_name)
                        .uri(Some(post.author_url.to_string()))
                        .build()])
                    .links(vec![LinkBuilder::default()
                        .href(url)
                        .rel("alternate")
                        .build()])
                    .build()
            })
            .collect();

        let feed = FeedBuilder::default()
            .title("Yew Blog")
            .id(format!("{BASE_URL}/blog"))
            .updated(
                format!("{}T00:00:00+00:00", posts[0].date)
                    .parse::<atom_syndication::FixedDateTime>()
                    .unwrap(),
            )
            .subtitle(Some(Text::plain("Yew Blog")))
            .icon(Some(format!("{BASE_URL}/img/logo.svg")))
            .links(vec![
                LinkBuilder::default()
                    .href(format!("{BASE_URL}/blog"))
                    .rel("alternate")
                    .build(),
                LinkBuilder::default()
                    .href(format!("{BASE_URL}/blog/atom.xml"))
                    .rel("self")
                    .build(),
            ])
            .entries(entries)
            .build();

        std::fs::write(blog_dir.join("atom.xml"), feed.to_string())?;
    }

    Ok(())
}

fn generate_redirects(output_dir: &Path) -> Result<()> {
    const REDIRECTS: &[(&str, &str)] = &[("/docs/next", "/docs/getting-started")];

    for &(from, to) in REDIRECTS {
        let dir = output_dir.join(from.trim_start_matches('/'));
        std::fs::create_dir_all(&dir)?;
        let html = format!(
            r#"<!DOCTYPE html><html><head><meta charset="utf-8"/><meta http-equiv="refresh" content="0;url={to}"/><link rel="canonical" href="{BASE_URL}{to}"/></head><body><a href="{to}">Redirect</a></body></html>"#,
        );
        std::fs::write(dir.join("index.html"), html)?;
        println!("  {from} -> {to}");
    }
    Ok(())
}

fn copy_static_assets(source_dir: &Path, output_dir: &Path) -> Result<()> {
    let static_dir = source_dir.join("static");
    if static_dir.exists() {
        println!("Copying static assets from {}", static_dir.display());
        copy_dir_recursive(&static_dir, output_dir)?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

async fn serve_dir(dir: &Path, port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    let dir = dir.to_path_buf();

    loop {
        let (mut stream, _) = listener.accept().await?;
        let dir = dir.clone();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 8192];
            let n = match tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await {
                Ok(n) if n > 0 => n,
                _ => return,
            };
            let request = String::from_utf8_lossy(&buf[..n]);
            let path = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap_or("/");

            let decoded = percent_decode(path.split('?').next().unwrap_or(path));

            let mut file_path = dir.join(decoded.trim_start_matches('/'));
            if file_path.is_dir() {
                file_path = file_path.join("index.html");
            }
            if !file_path.exists() && file_path.extension().is_none() {
                let with_html = file_path.with_extension("html");
                if with_html.exists() {
                    file_path = with_html;
                }
            }
            if !file_path.exists() {
                let not_found = dir.join("404.html");
                let body = std::fs::read(&not_found).unwrap_or_else(|_| b"404 Not Found".to_vec());
                let resp = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: \
                     text/html\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(resp.as_bytes()).await;
                let _ = stream.write_all(&body).await;
                return;
            }

            let mime = match file_path.extension().and_then(|e| e.to_str()) {
                Some("html") => "text/html; charset=utf-8",
                Some("js") => "application/javascript",
                Some("wasm") => "application/wasm",
                Some("css") => "text/css",
                Some("svg") => "image/svg+xml",
                Some("png") => "image/png",
                Some("jpg" | "jpeg") => "image/jpeg",
                Some("json") => "application/json",
                Some("xml") => "application/xml",
                Some("ico") => "image/x-icon",
                Some("txt") => "text/plain",
                Some("woff2") => "font/woff2",
                _ => "application/octet-stream",
            };

            match std::fs::read(&file_path) {
                Ok(body) => {
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: \
                         {mime}\r\nCache-Control: no-cache\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(resp.as_bytes()).await;
                    let _ = stream.write_all(&body).await;
                }
                Err(_) => {
                    let _ = stream
                        .write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n")
                        .await;
                }
            }
        });
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&String::from_utf8_lossy(&bytes[i + 1..i + 3]), 16)
            {
                result.push(byte);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let cwd = std::env::current_dir()?;
    let source_dir = if cwd.join("ssg").is_dir() && cwd.join("lib").is_dir() {
        cwd.clone()
    } else if cwd.join("yew-rs").is_dir() {
        cwd.join("yew-rs")
    } else {
        bail!("Cannot find yew-rs directory. Run from the workspace root or from yew-rs/.");
    };

    let output_dir = source_dir.join("build");
    std::fs::create_dir_all(&output_dir)?;

    let output_dir = if output_dir.is_absolute() {
        output_dir
    } else {
        cwd.join(&output_dir)
    };

    let total_start = std::time::Instant::now();

    println!("Discovering pages...");
    let all_pages = discover_pages(&source_dir)?;

    let all_pages = if args.all_locales {
        all_pages
    } else {
        all_pages
            .into_iter()
            .filter(|p| {
                !p.url_path.starts_with("/ja/")
                    && !p.url_path.starts_with("/zh-Hans/")
                    && !p.url_path.starts_with("/zh-Hant/")
            })
            .collect()
    };

    println!("Found {} pages", all_pages.len());

    let pages = all_pages;
    println!("Building {} pages", pages.len());

    let t = std::time::Instant::now();
    println!("\n=== Compile phase ===");
    cargo_build_all(&source_dir, &pages, args.release)?;
    println!("Compile phase: {:.1}s", t.elapsed().as_secs_f64());

    let target_dir = source_dir.join("..").join("target");

    let t = std::time::Instant::now();
    println!(
        "\n=== Bundle phase{} ===",
        if args.skip_wasm_opt {
            " (wasm-opt skipped)"
        } else {
            ""
        }
    );
    build_pages_parallel(
        &pages,
        &output_dir,
        &target_dir,
        args.release,
        args.skip_wasm_opt,
    )?;
    println!("Bundle phase: {:.1}s", t.elapsed().as_secs_f64());

    println!("\nCopying static assets...");
    copy_static_assets(&source_dir, &output_dir)?;

    println!("Generating sitemap.xml...");
    generate_sitemap(&pages, &output_dir, "https://yew.rs")?;

    println!("Generating blog feeds...");
    generate_blog_feeds(&output_dir)?;

    println!("Generating redirects...");
    generate_redirects(&output_dir)?;

    if !args.skip_capture {
        let t = std::time::Instant::now();
        println!("\n=== SSR Capture phase ===");
        println!("Rendering pages via ServerRenderer...");
        render_and_inject(&output_dir, args.all_locales).await?;
        println!("SSR phase: {:.1}s", t.elapsed().as_secs_f64());
    }

    let not_found_src = output_dir.join("404/index.html");
    if not_found_src.exists() {
        std::fs::copy(&not_found_src, output_dir.join("404.html"))?;
        println!("Copied 404/index.html -> 404.html");
    }

    println!("\nTotal: {:.1}s", total_start.elapsed().as_secs_f64());
    println!("Done! Static site at: {}", output_dir.display());

    if let Some(port) = args.serve {
        println!("\nServing on http://localhost:{port}");
        serve_dir(&output_dir, port).await?;
    }

    Ok(())
}
