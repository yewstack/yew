crate::doc_page!(
    "Hooks",
    "/ja/docs/concepts/function-components/hooks",
    Content::new(vec![
        h2!["Hooks"],
        p!["Hooks are functions that let you store state and perform side-effects."],
        p![
            "Yew comes with a few pre-defined Hooks. You can also create your own or discover \
             many ",
            link!["/community/awesome#hooks", "community made hooks"],
            ".",
        ],
        h2!["Rules of hooks"],
        ol![
            li![
                "A hook function name always has to start with ",
                code("use_"),
            ],
            li_blocks![
                p!["Hooks can only be used in the following locations:"],
                ul![
                    li!["Top level of a function / hook."],
                    li!["Blocks inside a function / hook, given it's not already branched."],
                    li![
                        "In the condition of a top level ",
                        code("if"),
                        " expression inside a function / hook.",
                    ],
                    li![
                        "In the scrutinee of a top level ",
                        code("match"),
                        " expression inside a function / hook.",
                    ],
                ],
            ],
            li![
                "Hooks must be called in the same order for every render. Returning early is only \
                 allowed when using ",
                link!["/ja/docs/concepts/suspense", "Suspense"],
            ],
        ],
        p!["These rules are enforced by either compile time or run-time errors."],
        h3!["Pre-defined Hooks"],
        p!["Yew comes with the following predefined Hooks:"],
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
            "The documentation for these hooks can be found in the ",
            link![
                "https://docs.rs/yew/0.20.0/yew/functional/index.html",
                "Yew API docs"
            ],
        ],
        h3!["Custom Hooks"],
        p![
            "There are cases where you want to define your own Hooks to encapsulate potentially \
             stateful logic from a component into reusable functions. See the ",
            link![
                "/ja/docs/concepts/function-components/hooks/custom-hooks#defining-custom-hooks",
                "Defining custom hooks"
            ],
            " section for more information.",
        ],
        h2!["Further reading"],
        ul![li![
            "The React documentation has a section on ",
            link!["https://reactjs.org/docs/hooks-intro.html", "React hooks"],
            ". These are not exactly the same as Yew's hooks, but the underlying concept is \
             similar.",
        ]],
    ])
);
