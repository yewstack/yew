pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("優先權")]),
        p(vec![
            text(
                "框架即將推出的功能和重點的優先順序由社群決定。 在 2020 \
                 年春季，我們發送了一份開發者調查，以收集關於專案方向的回饋。 您可以在 ",
            ),
            link(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                vec![text("Yew Wiki")],
            ),
            text(" 中找到調查摘要。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("所有主要倡議的狀態都可以在 Yew Github "),
                link(
                    "https://github.com/yewstack/yew/projects",
                    vec![text("專案看板")],
                ),
                text(" 上跟踪"),
            ])],
        ),
        h2(vec![text("重點")]),
        ol(vec![
            li(vec![text("最受歡迎的功能")]),
            li(vec![text("生產就緒")]),
            li(vec![text("文件")]),
            li(vec![text("痛點")]),
        ]),
        h3(vec![text("最受歡迎的功能")]),
        ol(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/projects/3",
                vec![text("函數組件")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/4",
                vec![text("元件庫")],
            )]),
            li(vec![text("更好的狀態管理")]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/5",
                vec![text("伺服器端渲染")],
            )]),
        ]),
        h3(vec![text("生產就緒所需的問題")]),
        ul(vec![
            li(vec![text("提高 Yew 測試覆蓋率")]),
            li(vec![text("減少二進位檔案大小")]),
            li(vec![link(
                "https://github.com/yewstack/yew/issues/5",
                vec![text("效能基準測試")],
            )]),
        ]),
        h3(vec![text("文件")]),
        ul(vec![
            li(vec![text("建立教程")]),
            li(vec![text("簡化項目設置")]),
        ]),
        h3(vec![text("痛點")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/issues/830",
                vec![text("組件樣板")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/6",
                vec![text("代理")],
            )]),
        ]),
    ])
}

crate::doc_page!("路線圖", "/zh-Hant/docs/more/roadmap", page_content());
