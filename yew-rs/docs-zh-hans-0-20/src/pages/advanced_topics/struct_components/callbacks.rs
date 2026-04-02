crate::doc_page!(
    "",
    "/zh-Hans/docs/advanced-topics/struct-components/callbacks",
    Content::new(vec![
        h1!["回调（Callbacks）"],
        p!["组件\"link\"是一种机制，通过该机制，组件可以注册回调并自行更新。"],
        h2!["ComponentLink API"],
        h3!["callback"],
        p![
            "注册一个回调，该回调将在执行时将消息发送到组件的更新机制。在内部，\
             它将使用提供的闭包返回的消息调用 ",
            code("send_self"),
            "。提供 ",
            code("Fn(IN) -> Vec<COMP::Message>"),
            "，返回 ",
            code("Callback<IN>"),
            "。",
        ],
        h3!["send_message"],
        p!["当前循环结束后立即向组件发送消息，导致另一个更新循环启动。"],
        h3!["send_message_batch"],
        p![
            "注册一个回调，该回调在执行时立即发送一批消息。\
             如果其中任何一个消息将导致组件重新渲染，那么组件会在该批次所有消息被处理后重新渲染。\
             提供 ",
            code("Fn(IN) -> COMP::Message"),
            "，返回 ",
            code("Callback<IN>"),
            "。",
        ],
        h2!["Callbacks"],
        p![
            "Callbacks 用于与 Yew 中的 services，agents 和父组件进行通信。它们仅仅是个 ",
            code("Fn"),
            "，并由 ",
            code("Rc"),
            " 包裹以允许被克隆。",
        ],
        p![
            "它们有一个 ",
            code("emit"),
            " 函数，该函数将它的 ",
            code("<IN>"),
            " 类型作为参数并将其转换为目标所期望的消息。如果一个回调从父组件中通过 props \
             提供给子组件，则子组件可以在其 ",
            code("update"),
            " 生命周期钩子中对该回调调用 ",
            code("emit"),
            "，以将消息发送回父组件。在 ",
            code("html!"),
            " 宏内被提供作为 props 的闭包或函数会自动转换为 Callbacks。",
        ],
    ])
);
