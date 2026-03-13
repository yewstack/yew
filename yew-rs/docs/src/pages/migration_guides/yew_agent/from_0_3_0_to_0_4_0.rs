pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("MSRV raised to 1.84.0")]),
        p(vec![
            text("The minimum supported Rust version is now "),
            bold(vec![text("1.84.0")]),
            text(". Update your toolchain:"),
        ]),
        code_block("bash", "rustup update stable"),
        h2(vec![text("gloo-worker vendored")]),
        p(vec![
            text("The external dependency on "),
            code("gloo-worker"),
            text(" has been removed. All worker functionality is now built into "),
            code("yew-agent"),
            text("."),
        ]),
        h3(vec![text("Update imports")]),
        p(vec![
            text("If you were importing types from "),
            code("gloo-worker"),
            text(", update to import from "),
            code("yew_agent"),
            text(":"),
        ]),
        code_block_ignore(
            "rust",
            r#"// Before
use gloo_worker::{Spawnable, Worker, WorkerScope};

// After
use yew_agent::prelude::*;
// or
use yew_agent::{Spawnable, Worker, WorkerScope};"#,
        ),
        h3(vec![text("Codec trait")]),
        p(vec![
            text("The "),
            code("Codec"),
            text(" trait is now defined in "),
            code("yew-agent"),
            text(":"),
        ]),
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
