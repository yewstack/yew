pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![
            code("_as_body"),
            " variant of ",
            code("start_app"),
            " is removed",
        ],
        p![
            "This method of controlling body has caused issues in event registration and SSR \
             hydration. They have been removed. Read more in the ",
            link!["https://github.com/yewstack/yew/pull/2346", "github issue",],
            ".",
        ],
        h2!["New Hooks and Function Components API"],
        p!["The Function Components and Hooks API are re-implemented with a different mechanism:"],
        ul![
            li![
                "User-defined hooks are now required to have a prefix ",
                code("use_"),
                " and must be marked with the ",
                code("#[hook]"),
                " attribute.",
            ],
            li![
                "Hooks will now report compile errors if they are not called from the top level \
                 of a function component or a user defined hook. The limitation existed in the \
                 previous version of Yew as well. In this version, It is reported as a compile \
                 time error."
            ],
        ],
        h2!["Automatic Message Batching"],
        p![
            "The scheduler now schedules its start to the end of the browser event loop. All \
             messages queued during in the meantime will be run in batch. The running order of \
             messages between components are no longer guaranteed, but messages sent to the same \
             component is still acknowledged in an FIFO order. If multiple updates will result in \
             a render, the component will be rendered once."
        ],
        admonition![
            AdmonitionType::Info,
            Some("What this means to developers?"),
            p![
                "For struct components, this means that if you send 2 messages to 2 different \
                 components, they will not be guaranteed to be seen in the same order they are \
                 sent. If you send 2 messages to the same component, they will still be passed to \
                 the component in the order they are sent. The messages are not sent to the \
                 component immediately so you should not assume that when the component receives \
                 a message it still has the same state at the time the message is created."
            ],
            p![
                "For function components, if you store states with ",
                code("use_state(_eq)"),
                " and the new value of that state depends on the previous value, you may want to \
                 switch to ",
                code("use_reducer(_eq)"),
                ". The new value of the state will not be visible / acknowledged until the next \
                 time the component is rendered. The reducer action works similar to messages for \
                 struct components and will be sent to the reducer function in the same order as \
                 they are dispatched. The reducer function can see all previous changes at the \
                 time they are run.",
            ],
        ],
        h2!["Yew Renderer"],
        p![
            code("start_app*"),
            " has been replaced by ",
            code("yew::Renderer"),
            ".",
        ],
        p![
            "You need to enable feature ",
            code("csr"),
            " to use ",
            code("yew::Renderer"),
            ".",
        ],
        h2![code("ref"), " prop for Components"],
        p![
            "Components no longer have a ",
            code("ref"),
            " prop. Trying to add a node ref to a component will result in a compile error",
        ],
        p![
            "Previously node ref passed to a component was bound to the first element rendered by \
             it. If this behavior is still desired, it is recommended to use add a ",
            code("r#ref"),
            " field to the component's properties and bind it manually",
        ],
        h2![code("changed"), " Method on Components"],
        p![
            "The method ",
            code("fn changed()"),
            " has now a new argument to provide the old properties to the function.",
        ],
        p!["The old method's signature was:"],
        code_block_ignore(
            "rust",
            r#"fn changed(&mut self, ctx: &Context<Self>) -> bool"#,
        ),
        p!["The new method's signature is now:"],
        code_block_ignore(
            "rust",
            r#"fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool"#,
        ),
        p![
            "This can be adjusted automatically in your code using this bash script (save your \
             code before running this!):"
        ],
        code_block(
            "bash",
            r#"perl -p -i -e  's/fn changed\(&mut self, (\w+): &Context<Self>\)/fn changed(&mut self, $1: &Context<Self>, _old_props: &Self::Properties)/g' $(find . -name \*.rs)"#,
        ),
    ])
}

crate::doc_page!(
    "From 0.19.0 to 0.20.0",
    "/docs/migration-guides/yew/from-0-19-0-to-0-20-0",
    page_content()
);
