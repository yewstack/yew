crate::community_page!(
    "Libraries",
    Content::new(vec![
        h2!["Malvolio"],
        p![
            link!["https://crates.io/crates/malvolio", "Malvolio"],
            " is a library with a \"builder-syntax\" for creating complex HTML documents with \
             ease. It runs both on servers (and renders to strings) or in browsers (with Yew).",
        ],
        h2!["Weblog"],
        p![
            link!["https://crates.io/crates/weblog", "weblog"],
            " is a crate that defines a set of macros for calling ",
            code("console.log()"),
            ", ",
            code("console.error()"),
            " and other members of the browser's console API when targeting WASM.",
        ],
        h2!["Gloo"],
        p![
            link!["https://crates.io/crates/gloo", "Gloo"],
            " is a modular toolkit for building fast, reliable Web applications and libraries \
             with Rust and Wasm. Gloo provides ergonomic Rust APIs for working with:",
        ],
        ul![
            li![link![
                "https://crates.io/crates/gloo-console-timer",
                "Console timers"
            ]],
            li![link!["https://crates.io/crates/gloo-dialogs", "Dialogs"]],
            li![link!["https://crates.io/crates/gloo-events", "Events"]],
            li![link!["https://crates.io/crates/gloo-file", "Files"]],
            li![link!["https://crates.io/crates/gloo-net", "Requests"]],
            li![link!["https://crates.io/crates/gloo-timers", "Timers"]],
            li![link![
                "https://crates.io/crates/gloo-storage",
                "Web Storage"
            ]],
        ],
        h2!["Looking For"],
        p!["Libraries that the ecosystem needs, but doesn't have yet."],
        p!["Bootstrap/MaterialUi/arbitrary css framework component wrappers."],
    ])
);
