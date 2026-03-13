crate::doc_page!(
    "Roadmap",
    "/zh-Hant/docs/more/roadmap",
    Content::new(vec![
        h3(vec![text("優先順序")]),
        p(vec![
            text(
                "即將推出的新功能和重點開發方向的優先順序將由社群決定。在 2020 \
                 的春季，我們會發出一個開發者調查，收集專案方向的回饋。你可以在 "
            ),
            link(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                vec![text("Yew Wiki")],
            ),
            text(" 中找到結果。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("你可以在 Yew GitHub 追蹤我們主要的開發方向 "),
                link(
                    "https://github.com/yewstack/yew/projects",
                    vec![text("Project board")],
                ),
            ])],
        ),
        h3(vec![text("重點")]),
        ol(vec![
            li(vec![text("需求最多的功能")]),
            li(vec![text("產品準備")]),
            li(vec![text("文件")]),
            li(vec![text("痛點")]),
        ]),
        h4(vec![text("需求最多的功能")]),
        ol(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/projects/3",
                vec![text("函數式元件")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/4",
                vec![text("元件函式庫")],
            )]),
            li(vec![text("更好的狀態管理器")]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/5",
                vec![text("Server side rendering")],
            )]),
        ]),
        h4(vec![text("產品準備")]),
        ul(vec![
            li(vec![text("提升 Yew 的測試覆蓋率")]),
            li(vec![text("減少二進位檔的大小")]),
            li(vec![link(
                "https://github.com/yewstack/yew/issues/5",
                vec![text("Benchmark performance")],
            )]),
        ]),
        h4(vec![text("文件")]),
        ul(vec![
            li(vec![text("建立教學文件")]),
            li(vec![text("簡化專案設定")]),
        ]),
        h4(vec![text("痛點")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/issues/830",
                vec![text("元件模板")],
            )]),
            li(vec![text("Fetch API")]),
            li(vec![text("Agents")]),
        ]),
    ])
);
