crate::blog_page!(
    &crate::BLOG_POSTS[0],
    Content::new(vec![
        p![
            "The Yew team is thrilled to announce the release of Yew 0.22! After a \
             longer-than-expected journey, this release brings significant improvements to \
             ergonomics, performance, and developer experience.",
        ],
        h2!["Highlights"],
        h3!["New #[component] Attribute"],
        p![
            "The ",
            code("#[function_component]"),
            " attribute has been renamed to ",
            code("#[component]"),
            " for brevity:",
        ],
        code_block(
            "rust",
            r##"// Before
#[function_component]
fn MyComponent() -> Html {
    html! { <div>{"Hello!"}</div> }
}

// After (0.22+)
#[component]
fn MyComponent() -> Html {
    html! { <div>{"Hello!"}</div> }
}"##
        ),
        p![
            "The old ",
            code("#[function_component]"),
            " attribute is deprecated but still works, giving you time to migrate.",
        ],
        h3!["For-Loops in html!"],
        p![
            "You can now use for-loops directly in the ",
            code("html!"),
            " macro, making iteration more natural:",
        ],
        code_block(
            "rust",
            r#"// Before - using iterator adapters
html! {
    <ul>
        { for items.iter().map(|item| html! { <li>{ item }</li> }) }
    </ul>
}

// After (0.22+) - native for-loop syntax
html! {
    <ul>
        for item in items {
            <li>{ item }</li>
        }
    </ul>
}"#
        ),
        h3!["MSRV Raised to 1.84.0"],
        p![
            "The minimum supported Rust version is now ",
            bold!["1.84.0"],
            ". This allows us to use newer language features and provide better error messages.",
        ],
        h3!["WASI Support for SSR"],
        p![
            "Server-side rendering now works on WASI targets. See the ",
            link![
                "/blog/2024/10/14/release-0-22",
                "original 0.22 announcement"
            ],
            " for details.",
        ],
        h3!["Better Cloning Ergonomics"],
        ul![li![
            code("ImplicitClone"),
            " is implemented for more yew types. This means less ",
            code("&"),
            " and ",
            code("*"),
            " and ",
            code(".clone()"),
            " clutter in the html macro.",
        ],],
        h3!["yew-agent: Vendored gloo-workers"],
        p![
            "The ",
            code("yew-agent"),
            " crate now includes its own web worker implementation, removing the external \
             dependency on ",
            code("gloo-worker"),
            ". This also adds support for ",
            bold!["module-type web workers"],
            ":",
        ],
        code_block(
            "rust",
            r#"let spawner = WorkerSpawner::<MyWorker>::new()
    .as_module(true)  // Use ES module workers
    .spawn();"#
        ),
        h3!["yew-router: Query Parameter Traits"],
        p![
            "The ",
            code("FromQuery"),
            " and ",
            code("ToQuery"),
            " traits from gloo are now re-exported via ",
            code("yew_router::query"),
            " for more flexible query parameter handling, along with dynamic basename support.",
        ],
        h2!["Migration Guide"],
        p![
            "See the ",
            link![
                "/docs/next/migration-guides/yew/from-0_21_0-to-0_22_0",
                "migration guide"
            ],
            " for detailed instructions on upgrading from 0.21.",
        ],
        h2!["Contributors"],
        p!["Many thanks to everyone who contributed to this release! Special thanks to:"],
        ul![
            li![
                link!["https://github.com/WorldSEnder", "@WorldSEnder"],
                " for hydration fixes",
            ],
            li![
                link!["https://github.com/its-the-shrimp", "@its-the-shrimp"],
                " for html macro enhancements",
            ],
            li![
                link!["https://github.com/kirillsemyonkin", "@Kirill Semyonkin"],
                " for implicit clone library improvements",
            ],
            li![
                link!["https://github.com/langyo", "@langyo"],
                " for WASI SSR support",
            ],
            li![
                link!["https://github.com/cecton", "@cecton"],
                " for implicit clone improvements and ergonomics",
            ],
            li![
                link!["https://github.com/ranile", "@ranile"],
                " for property improvements",
            ],
        ],
        p!["And all the other contributors who helped make this release possible!"],
        h2!["What's Next"],
        p![
            "We're continuing to work on improving Yew's performance, developer experience, and \
             documentation. Join us on ",
            link!["https://discord.gg/VQck8X4", "Discord"],
            " to get involved!",
        ],
        p![
            "See the ",
            link![
                "https://github.com/yewstack/yew/blob/master/CHANGELOG.md",
                "full changelog"
            ],
            " for all changes.",
        ],
    ])
);
