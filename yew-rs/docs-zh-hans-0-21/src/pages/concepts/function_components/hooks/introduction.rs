crate::doc_page!(
    "Hooks",
    "/zh-Hans/docs/concepts/function-components/hooks",
    Content::new(vec![
        h2(vec![text("Hooks")]),
        p(vec![text(
            "Hooks are functions that let you store state and perform side effects."
        )]),
        p(vec![
            text(
                "Yew comes with a few pre-defined hooks. You can also create your own or discover \
                 many "
            ),
            link(
                "/community/awesome#hooks",
                vec![text("community-made hooks")]
            ),
            text(".")
        ]),
        h2(vec![text("Rules of hooks")]),
        ol(vec![
            li(vec![
                text("A hook function name always has to start with "),
                code("use_")
            ]),
            li_blocks(vec![
                p(vec![text(
                    "Hooks can only be used in the following locations:"
                )]),
                ul(vec![
                    li(vec![text("Top-level of a function/hook.")]),
                    li(vec![text(
                        "Blocks inside a function/hook, given it is not already branched."
                    )]),
                    li(vec![
                        text("In the condition of a top-level "),
                        code("if"),
                        text(" expression inside a function/hook.")
                    ]),
                    li(vec![
                        text("In the scrutinee of a top-level "),
                        code("match"),
                        text(" expression inside a function/hook.")
                    ])
                ])
            ]),
            li(vec![
                text(
                    "Hooks must be called in the same order for every render. Returning early is \
                     only allowed when using "
                ),
                link("/docs/0.21/concepts/suspense", vec![text("Suspense")])
            ])
        ]),
        p(vec![text(
            "These rules are enforced by either compile-time or run-time errors."
        )]),
        h3(vec![text("Pre-defined Hooks")]),
        p(vec![text("Yew comes with the following predefined Hooks:")]),
        ul(vec![
            li(vec![code("use_state")]),
            li(vec![code("use_state_eq")]),
            li(vec![code("use_memo")]),
            li(vec![code("use_callback")]),
            li(vec![code("use_mut_ref")]),
            li(vec![code("use_node_ref")]),
            li(vec![code("use_reducer")]),
            li(vec![code("use_reducer_eq")]),
            li(vec![code("use_effect")]),
            li(vec![code("use_effect_with")]),
            li(vec![code("use_context")]),
            li(vec![code("use_force_update")])
        ]),
        p(vec![
            text("The documentation for these hooks can be found in the "),
            link(
                "https://yew-rs-api.web.app/next/yew/functional/",
                vec![text("Yew API docs")]
            )
        ]),
        h3(vec![text("Custom Hooks")]),
        p(vec![
            text(
                "There are cases where you want to define your own Hooks to encapsulate \
                 potentially stateful logic from a component into reusable functions. See the "
            ),
            link(
                "/docs/0.21/concepts/function-components/hooks/custom-hooks#defining-custom-hooks",
                vec![text("Defining custom hooks")]
            ),
            text(" section for more information.")
        ]),
        h2(vec![text("Further reading")]),
        ul(vec![li(vec![
            text("The React documentation has a section on "),
            link(
                "https://reactjs.org/docs/hooks-intro.html",
                vec![text("React hooks")]
            ),
            text(". These are not the same as Yew's hooks, but the underlying concept is similar.")
        ])])
    ])
);
