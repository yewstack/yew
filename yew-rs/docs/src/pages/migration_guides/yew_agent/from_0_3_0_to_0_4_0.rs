pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["MSRV raised to 1.84.0"],
        p![
            "The minimum supported Rust version is now ",
            bold!["1.84.0"],
            ". Update your toolchain:",
        ],
        code_block("bash", "rustup update stable"),
        h2!["gloo-worker vendored"],
        p![
            "The external dependency on ",
            code("gloo-worker"),
            " has been removed. All worker functionality is now built into ",
            code("yew-agent"),
            ".",
        ],
        h3!["Update imports"],
        p![
            "If you were importing types from ",
            code("gloo-worker"),
            ", update to import from ",
            code("yew_agent"),
            ":",
        ],
        code_block_ignore(
            "rust",
            r#"// Before
use gloo_worker::{Spawnable, Worker, WorkerScope};

// After
use yew_agent::prelude::*;
// or
use yew_agent::{Spawnable, Worker, WorkerScope};"#,
        ),
        h3!["Codec trait"],
        p![
            "The ",
            code("Codec"),
            " trait is now defined in ",
            code("yew-agent"),
            ":",
        ],
        code_block_ignore(
            "rust",
            r#"// Before
use gloo_worker::Codec;

// After
use yew_agent::Codec;"#,
        ),
    ])
}

crate::doc_page!(
    "From 0.3.0 to 0.4.0",
    "/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0",
    page_content()
);
