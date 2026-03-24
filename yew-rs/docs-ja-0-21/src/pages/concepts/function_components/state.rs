crate::doc_page!(
    "State",
    "/ja/docs/concepts/function-components/state",
    Content::new(vec![
        h2![text("General view of how to store state")],
        p![text(
            "This table can be used as a guide when deciding what state-storing type fits best \
             for your use case:"
        )],
        table(
            vec![
                vec![text("Hook")],
                vec![text("Type")],
                vec![text("Rerender when?")],
                vec![text("Scope")],
            ],
            vec![
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_state.html",
                        text("use_state")
                    )],
                    vec![code("T")],
                    vec![text("got set")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_state_eq.html",
                        text("use_state_eq")
                    )],
                    vec![code("T: PartialEq")],
                    vec![text("got set with diff. value")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html",
                        text("use_reducer")
                    )],
                    vec![code("T: Reducible")],
                    vec![text("got reduced")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer_eq.html",
                        text("use_reducer_eq")
                    )],
                    vec![code("T: Reducible + PartialEq")],
                    vec![text("got reduced with diff. value")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_memo.html",
                        text("use_memo")
                    )],
                    vec![code("Deps -> T")],
                    vec![text("dependencies changed")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_callback.html",
                        text("use_callback")
                    )],
                    vec![code("Deps -> Callback<E>")],
                    vec![text("dependencies changed")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_mut_ref.html",
                        text("use_mut_ref")
                    )],
                    vec![code("T")],
                    vec![text("-")],
                    vec![text("component instance")],
                ],
                vec![
                    vec![text("a static global variable")],
                    vec![code("T")],
                    vec![text("-")],
                    vec![text("global, used by all")],
                ],
            ]
        ),
    ])
);
