crate::blog_page!(
    &crate::BLOG_POSTS[0],
    Content::new(vec![
        p(vec![text(
            "The Yew team is thrilled to announce the release of Yew 0.22! After a \
             longer-than-expected journey, this release brings significant improvements to \
             ergonomics, performance, and developer experience."
        ),]),
        h2(vec![text("Highlights")]),
        h3(vec![text("New #[component] Attribute")]),
        p(vec![
            text("The "),
            code("#[function_component]"),
            text(" attribute has been renamed to "),
            code("#[component]"),
            text(" for brevity:"),
        ]),
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
        p(vec![
            text("The old "),
            code("#[function_component]"),
            text(" attribute is deprecated but still works, giving you time to migrate."),
        ]),
        h3(vec![text("For-Loops in html!")]),
        p(vec![
            text("You can now use for-loops directly in the "),
            code("html!"),
            text(" macro, making iteration more natural:"),
        ]),
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
        h3(vec![text("MSRV Raised to 1.84.0")]),
        p(vec![
            text("The minimum supported Rust version is now "),
            bold(vec![text("1.84.0")]),
            text(
                ". This allows us to use newer language features and provide better error \
                 messages."
            ),
        ]),
        h3(vec![text("WASI Support for SSR")]),
        p(vec![
            text("Server-side rendering now works on WASI targets. See the "),
            link(
                "/blog/2024/10/14/release-0-22",
                vec![text("original 0.22 announcement")]
            ),
            text(" for details."),
        ]),
        h3(vec![text("Better Cloning Ergonomics")]),
        ul(vec![li(vec![
            code("ImplicitClone"),
            text(" is implemented for more yew types. This means less "),
            code("&"),
            text(" and "),
            code("*"),
            text(" and "),
            code(".clone()"),
            text(" clutter in the html macro."),
        ]),]),
        h3(vec![text("yew-agent: Vendored gloo-workers")]),
        p(vec![
            text("The "),
            code("yew-agent"),
            text(
                " crate now includes its own web worker implementation, removing the external \
                 dependency on "
            ),
            code("gloo-worker"),
            text(". This also adds support for "),
            bold(vec![text("module-type web workers")]),
            text(":"),
        ]),
        code_block(
            "rust",
            r#"let spawner = WorkerSpawner::<MyWorker>::new()
    .as_module(true)  // Use ES module workers
    .spawn();"#
        ),
        h3(vec![text("yew-router: Query Parameter Traits")]),
        p(vec![
            text("The "),
            code("FromQuery"),
            text(" and "),
            code("ToQuery"),
            text(" traits from gloo are now re-exported via "),
            code("yew_router::query"),
            text(
                " for more flexible query parameter handling, along with dynamic basename support."
            ),
        ]),
        h2(vec![text("Migration Guide")]),
        p(vec![
            text("See the "),
            link(
                "/docs/next/migration-guides/yew/from-0_21_0-to-0_22_0",
                vec![text("migration guide")]
            ),
            text(" for detailed instructions on upgrading from 0.21."),
        ]),
        h2(vec![text("Contributors")]),
        p(vec![text(
            "Many thanks to everyone who contributed to this release! Special thanks to:"
        )]),
        ul(vec![
            li(vec![
                link("https://github.com/WorldSEnder", vec![text("@WorldSEnder")]),
                text(" for hydration fixes"),
            ]),
            li(vec![
                link(
                    "https://github.com/its-the-shrimp",
                    vec![text("@its-the-shrimp")]
                ),
                text(" for html macro enhancements"),
            ]),
            li(vec![
                link(
                    "https://github.com/kirillsemyonkin",
                    vec![text("@Kirill Semyonkin")]
                ),
                text(" for implicit clone library improvements"),
            ]),
            li(vec![
                link("https://github.com/langyo", vec![text("@langyo")]),
                text(" for WASI SSR support"),
            ]),
            li(vec![
                link("https://github.com/cecton", vec![text("@cecton")]),
                text(" for implicit clone improvements and ergonomics"),
            ]),
            li(vec![
                link("https://github.com/ranile", vec![text("@ranile")]),
                text(" for property improvements"),
            ]),
        ]),
        p(vec![text(
            "And all the other contributors who helped make this release possible!"
        )]),
        h2(vec![text("What's Next")]),
        p(vec![
            text(
                "We're continuing to work on improving Yew's performance, developer experience, \
                 and documentation. Join us on "
            ),
            link("https://discord.gg/VQck8X4", vec![text("Discord")]),
            text(" to get involved!"),
        ]),
        p(vec![
            text("See the "),
            link(
                "https://github.com/yewstack/yew/blob/master/CHANGELOG.md",
                vec![text("full changelog")]
            ),
            text(" for all changes."),
        ]),
    ])
);
