crate::doc_page!(
    "Hooks",
    "/zh-Hant/docs/concepts/function-components/hooks",
    Content::new(vec![
        h2![text("Hooks")],
        p![text(
            "Hooks are functions that let you store state and perform side-effects."
        )],
        p![
            text(
                "Yew comes with a few pre-defined Hooks. You can also create your own or discover \
                 many "
            ),
            link!("/community/awesome#hooks", text("community made hooks")),
            text("."),
        ],
        h2![text("Rules of hooks")],
        ol![
            li![
                text("A hook function name always has to start with "),
                code("use_"),
            ],
            li_blocks![
                p![text("Hooks can only be used in the following locations:")],
                ul![
                    li![text("Top level of a function / hook.")],
                    li![text(
                        "Blocks inside a function / hook, given it's not already branched."
                    )],
                    li![
                        text("In the condition of a top level "),
                        code("if"),
                        text(" expression inside a function / hook."),
                    ],
                    li![
                        text("In the scrutinee of a top level "),
                        code("match"),
                        text(" expression inside a function / hook."),
                    ],
                ],
            ],
            li![
                text(
                    "Hooks must be called in the same order for every render. Returning early is \
                     only allowed when using "
                ),
                link!("/zh-Hant/docs/concepts/suspense", text("Suspense")),
            ],
        ],
        p![text(
            "These rules are enforced by either compile time or run-time errors."
        )],
        h3![text("Pre-defined Hooks")],
        p![text("Yew comes with the following predefined Hooks:")],
        ul![
            li![code("use_state")],
            li![code("use_state_eq")],
            li![code("use_memo")],
            li![code("use_callback")],
            li![code("use_mut_ref")],
            li![code("use_node_ref")],
            li![code("use_reducer")],
            li![code("use_reducer_eq")],
            li![code("use_effect")],
            li![code("use_effect_with_deps")],
            li![code("use_context")],
            li![code("use_force_update")],
        ],
        p![
            text("The documentation for these hooks can be found in the "),
            link!(
                "https://docs.rs/yew/0.20.0/yew/functional/index.html",
                text("Yew API docs")
            ),
        ],
        h3![text("Custom Hooks")],
        p![
            text(
                "There are cases where you want to define your own Hooks to encapsulate \
                 potentially stateful logic from a component into reusable functions. See the "
            ),
            link!(
                "/zh-Hant/docs/concepts/function-components/hooks/custom-hooks#\
                 defining-custom-hooks",
                text("Defining custom hooks")
            ),
            text(" section for more information."),
        ],
        h2![text("Further reading")],
        ul![li![
            text("The React documentation has a section on "),
            link!(
                "https://reactjs.org/docs/hooks-intro.html",
                text("React hooks")
            ),
            text(
                ". These are not exactly the same as Yew's hooks, but the underlying concept is \
                 similar."
            ),
        ]],
    ])
);
