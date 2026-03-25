crate::doc_page!(
    "透過範例學習",
    "/zh-Hant/docs/getting-started/examples",
    Content::new(vec![
        p![text(
            "我們有各種範例（都持續在維護中），建議你可以仔細閱讀他們，\
             以了解如何使用各種不同的框架功能。當遇到問題或需要幫忙時，我們也很歡迎大家 \
             pull-requests 或開 issues",
        )],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/std_web/todomvc",
                bold![text("Todo App (stdweb)")],
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/web_sys/todomvc",
                bold![text("Todo App (web_sys)")],
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/custom_components",
                bold![text("Custom Components")],
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/std_web/multi_thread",
                bold![text("Multi-threading (Agents) (stdweb)")],
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/web_sys/multi_thread",
                bold![text("Multi-threading (Agents) (web_sys)")],
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/timer",
                bold![text("Timer Service")],
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/nested_list",
                bold![text("Nested Components")],
            )],
        ],
    ])
);
