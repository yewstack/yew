crate::doc_page!(
    "State",
    "/zh-Hant/docs/concepts/function-components/state",
    Content::new(vec![
        h2!["General view of how to store state"],
        p![
            "This table can be used as a guide when deciding what state storing type fits best \
             for your use case:"
        ],
        table(
            vec![
                vec!["Hook".into()],
                vec!["Type".into()],
                vec!["Rerender when?".into()],
                vec!["Scope".into()],
            ],
            vec![
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_state.html",
                        "use_state",
                    )],
                    vec![code("T")],
                    vec!["got set".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_state_eq.html",
                        "use_state_eq",
                    )],
                    vec![code("T: PartialEq")],
                    vec!["got set with diff. value".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html",
                        "use_reducer",
                    )],
                    vec![code("T: Reducible")],
                    vec!["got reduced".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer_eq.html",
                        "use_reducer_eq",
                    )],
                    vec![code("T: Reducible + PartialEq")],
                    vec!["got reduced with diff. value".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_memo.html",
                        "use_memo",
                    )],
                    vec![code("Deps -> T")],
                    vec!["dependencies changed".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_callback.html",
                        "use_callback",
                    )],
                    vec![code("Deps -> Callback<E>")],
                    vec!["dependencies changed".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec![link!(
                        "https://yew-rs-api.web.app/next/yew/functional/fn.use_mut_ref.html",
                        "use_mut_ref",
                    )],
                    vec![code("T")],
                    vec!["-".into()],
                    vec!["component instance".into()],
                ],
                vec![
                    vec!["a static global variable".into()],
                    vec![code("T")],
                    vec!["-".into()],
                    vec!["global, used by all".into()],
                ],
            ],
        ),
    ])
);
