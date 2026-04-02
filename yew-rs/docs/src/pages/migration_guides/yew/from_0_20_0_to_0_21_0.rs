pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![
            "Dependencies as first hook argument and ",
            code("use_effect_with"),
        ],
        ul![
            li![
                "Replace ",
                code("use_effect_with_deps"),
                " with new ",
                code("use_effect_with"),
            ],
            li![
                code("use_effect_with"),
                ", ",
                code("use_callback"),
                ", ",
                code("use_memo"),
                " now take dependencies as their first argument",
            ],
        ],
        h3!["Automated refactor"],
        p![
            "With the help of ",
            link![
                "https://ast-grep.github.io/guide/quick-start.html",
                "https://ast-grep.github.io",
            ],
        ],
        p!["Here are commands that can do the refactoring for you."],
        code_block(
            "bash",
            r#"sg --pattern 'use_effect_with_deps($CALLBACK,$$$DEPENDENCIES)' --rewrite 'use_effect_with($$$DEPENDENCIES, $CALLBACK)' -l rs -i
sg --pattern 'use_effect_with($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_effect_with($DEPENDENCIES,$$$CALLBACK)' -l rs -i

sg --pattern 'use_callback($CALLBACK,$$$DEPENDENCIES)' --rewrite 'use_callback($$$DEPENDENCIES, $CALLBACK)' -l rs -i
sg --pattern 'use_callback($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_callback($DEPENDENCIES,$$$CALLBACK)' -l rs -i

sg --pattern 'use_memo($CALLBACK,$$$DEPENDENCIES)' --rewrite 'use_memo($$$DEPENDENCIES, $CALLBACK)' -l rs -i
sg --pattern 'use_memo($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_memo($DEPENDENCIES,$$$CALLBACK)' -l rs -i

sg --pattern 'use_future_with_deps($CALLBACK,$$$DEPENDENCIES)' --rewrite 'use_future_with($$$DEPENDENCIES, $CALLBACK)' -l rs -i
sg --pattern 'use_future_with($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_future_with($DEPENDENCIES,$$$CALLBACK)' -l rs -i

sg --pattern 'use_transitive_state!($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_transitive_state!($DEPENDENCIES,$$$CALLBACK)' -l rs -i
sg --pattern 'use_transitive_state!($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_transitive_state!($DEPENDENCIES,$$$CALLBACK)' -l rs -i

sg --pattern 'use_prepared_state!($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_prepared_state!($DEPENDENCIES,$$$CALLBACK)' -l rs -i
sg --pattern 'use_prepared_state!($DEPENDENCIES,,$$$CALLBACK)' --rewrite 'use_prepared_state!($DEPENDENCIES,$$$CALLBACK)' -l rs -i"#,
        ),
        h3!["Reasoning"],
        p!["This will enable more ergonomic use of hooks, consider:"],
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "Before",
                    vec![code_block_ignore(
                        "rust",
                        r#"impl SomeLargeStruct {
    fn id(&self) -> u32; // Only need to use the id as cache key
}
let some_dep: SomeLargeStruct = todo!();

{
    let id = some_dep.id(); // Have to extract it in advance, some_dep is moved already in the second argument
    use_effect_with_dep(move |_| { todo!(); drop(some_dep); }, id);
}"#,
                    )],
                ),
                tab(
                    "after",
                    "After",
                    vec![code_block_ignore(
                        "rust",
                        r#"impl SomeLargeStruct {
    fn id(&self) -> u32; // Only need to use the id as cache key
}
let some_dep: SomeLargeStruct = todo!();

use_effect_with(some_dep.id(), move |_| { todo!(); drop(some_dep); });"#,
                    )],
                ),
            ],
        ),
    ])
}

crate::doc_page!(
    "From 0.20.0 to 0.21.0",
    "/docs/migration-guides/yew/from-0-20-0-to-0-21-0",
    page_content()
);
