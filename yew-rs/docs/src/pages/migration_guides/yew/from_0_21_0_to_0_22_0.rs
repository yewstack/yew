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
        h2![
            code("#[function_component]"),
            " renamed to ",
            code("#[component]"),
        ],
        p![
            "The ",
            code("#[function_component]"),
            " attribute has been renamed to ",
            code("#[component]"),
            " for brevity. The old name is deprecated but still works.",
        ],
        h3!["Automated refactor"],
        code_block(
            "bash",
            r##"# Using sed (simple but also replaces in comments/strings)
find . -name "*.rs" -exec sed -i 's/#\[function_component\]/#[component]/g' {} +
find . -name "*.rs" -exec sed -i 's/#\[function_component(/#[component(/g' {} +

# Or using ast-grep (recommended - AST-aware, preserves comments/strings)
# Important: Run the named pattern FIRST to preserve component names
ast-grep run -p '#[function_component($$$ARGS)]' -r '#[component($$$ARGS)]' -l rust --update-all .
ast-grep run -p '#[function_component]' -r '#[component]' -l rust --update-all ."##,
        ),
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "The sed commands will also replace occurrences in comments and strings. Use \
                 ast-grep for more precise refactoring."
            ],
        ],
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "Before",
                    vec![code_block_ignore(
                        "rust",
                        r##"#[function_component]
fn MyComponent() -> Html {
    html! { <div>{"Hello"}</div> }
}

#[function_component(Named)]
fn AnotherComponent() -> Html {
    html! { <div>{"World"}</div> }
}"##,
                    )],
                ),
                tab(
                    "after",
                    "After",
                    vec![code_block_ignore(
                        "rust",
                        r##"#[component]
fn MyComponent() -> Html {
    html! { <div>{"Hello"}</div> }
}

#[component(Named)]
fn AnotherComponent() -> Html {
    html! { <div>{"World"}</div> }
}"##,
                    )],
                ),
            ],
        ),
        h2![code("class=(...)"), " syntax removed"],
        p![
            "The deprecated ",
            code("class=(expr)"),
            " syntax has been removed. Use ",
            code("class={classes!(...)}"),
            " instead.",
        ],
        h3!["Finding occurrences"],
        code_block(
            "bash",
            r##"# Find all files using the old class=(...) syntax
grep -rn "class=(" --include="*.rs" ."##,
        ),
        h3!["Manual refactor"],
        p![
            "The transformation is straightforward: wrap the tuple contents with ",
            code("classes!()"),
            " and change parentheses to braces:",
        ],
        ul![
            li![code("class=(a, b)"), " → ", code("class={classes!(a, b)}"),],
            li![code("class=(expr)"), " → ", code("class={classes!(expr)}"),],
        ],
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "Before",
                    vec![code_block_ignore(
                        "rust",
                        r#"html! {
    <div class=(some_class, other_class)>{"Content"}</div>
}"#,
                    )],
                ),
                tab(
                    "after",
                    "After",
                    vec![code_block_ignore(
                        "rust",
                        r#"html! {
    <div class={classes!(some_class, other_class)}>{"Content"}</div>
}"#,
                    )],
                ),
            ],
        ),
        h2![code("ToHtml"), " trait removed"],
        p![
            "The ",
            code("ToHtml"),
            " trait has been removed. Use ",
            code("IntoPropValue"),
            " for custom type conversions.",
        ],
        h2!["For-loops in ", code("html!"), " macro"],
        p![
            "You can now use for-loops directly in the ",
            code("html!"),
            " macro. This is optional but provides cleaner syntax:",
        ],
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "Before (still works)",
                    vec![code_block_ignore(
                        "rust",
                        r#"html! {
    <ul>
        { for items.iter().map(|item| html! { <li key={item.id}>{ &item.name }</li> }) }
    </ul>
}"#,
                    )],
                ),
                tab(
                    "after",
                    "After (new syntax)",
                    vec![code_block_ignore(
                        "rust",
                        r#"html! {
    <ul>
        for item in items {
            <li key={item.id}>{ &item.name }</li>
        }
    </ul>
}"#,
                    )],
                ),
            ],
        ),
        h2![
            code("use_effect_with"),
            " no longer requires ",
            code("|| ()"),
            " return",
        ],
        p![
            "Effect hooks no longer require returning ",
            code("|| ()"),
            " when there's no cleanup:",
        ],
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "Before",
                    vec![code_block_ignore(
                        "rust",
                        r#"use_effect_with(deps, |deps| {
    // do something
    || ()  // had to return this
});"#,
                    )],
                ),
                tab(
                    "after",
                    "After",
                    vec![code_block_ignore(
                        "rust",
                        r#"use_effect_with(deps, |deps| {
    // do something
    // no return needed!
});"#,
                    )],
                ),
            ],
        ),
    ])
}

crate::doc_page!(
    "From 0.21.0 to 0.22.0",
    "/docs/migration-guides/yew/from-0-21-0-to-0-22-0",
    page_content()
);
