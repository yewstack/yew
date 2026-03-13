crate::community_page!(
    "Libraries",
    Content::new(vec![
        h2(vec![text("Malvolio")]),
        p(vec![
            link("https://crates.io/crates/malvolio", vec![text("Malvolio")]),
            text(
                " is a library with a \"builder-syntax\" for creating complex HTML documents with \
                 ease. It runs both on servers (and renders to strings) or in browsers (with Yew)."
            ),
        ]),
        h2(vec![text("Weblog")]),
        p(vec![
            link("https://crates.io/crates/weblog", vec![text("weblog")]),
            text(" is a crate that defines a set of macros for calling "),
            code("console.log()"),
            text(", "),
            code("console.error()"),
            text(" and other members of the browser's console API when targeting WASM."),
        ]),
        h2(vec![text("Gloo")]),
        p(vec![
            link("https://crates.io/crates/gloo", vec![text("Gloo")]),
            text(
                " is a modular toolkit for building fast, reliable Web applications and libraries \
                 with Rust and Wasm. Gloo provides ergonomic Rust APIs for working with:"
            ),
        ]),
        ul(vec![
            li(vec![link(
                "https://crates.io/crates/gloo-console-timer",
                vec![text("Console timers")]
            )]),
            li(vec![link(
                "https://crates.io/crates/gloo-dialogs",
                vec![text("Dialogs")]
            )]),
            li(vec![link(
                "https://crates.io/crates/gloo-events",
                vec![text("Events")]
            )]),
            li(vec![link(
                "https://crates.io/crates/gloo-file",
                vec![text("Files")]
            )]),
            li(vec![link(
                "https://crates.io/crates/gloo-net",
                vec![text("Requests")]
            )]),
            li(vec![link(
                "https://crates.io/crates/gloo-timers",
                vec![text("Timers")]
            )]),
            li(vec![link(
                "https://crates.io/crates/gloo-storage",
                vec![text("Web Storage")]
            )]),
        ]),
        h2(vec![text("Looking For")]),
        p(vec![text(
            "Libraries that the ecosystem needs, but doesn't have yet."
        )]),
        p(vec![text(
            "Bootstrap/MaterialUi/arbitrary css framework component wrappers."
        )]),
    ])
);
