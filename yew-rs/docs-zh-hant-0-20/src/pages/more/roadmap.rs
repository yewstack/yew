crate::doc_page!(
    "Roadmap",
    "/zh-Hant/docs/more/roadmap",
    Content::new(vec![
        h3!["優先順序"],
        p![
            "即將推出的新功能和重點開發方向的優先順序將由社群決定。在 2020 \
             的春季，我們會發出一個開發者調查，收集專案方向的回饋。你可以在 ",
            link!(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                "Yew Wiki",
            ),
            " 中找到結果。",
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "你可以在 Yew GitHub 追蹤我們主要的開發方向 ",
                link!("https://github.com/yewstack/yew/projects", "Project board",),
            ],
        ],
        h3!["重點"],
        ol![
            li!["需求最多的功能"],
            li!["產品準備"],
            li!["文件"],
            li!["痛點"],
        ],
        h4!["需求最多的功能"],
        ol![
            li![link!(
                "https://github.com/yewstack/yew/projects/3",
                "函數式元件",
            )],
            li![link!(
                "https://github.com/yewstack/yew/projects/4",
                "元件函式庫",
            )],
            li!["更好的狀態管理器"],
            li![link!(
                "https://github.com/yewstack/yew/projects/5",
                "Server side rendering",
            )],
        ],
        h4!["產品準備"],
        ul![
            li!["提升 Yew 的測試覆蓋率"],
            li!["減少二進位檔的大小"],
            li![link!(
                "https://github.com/yewstack/yew/issues/5",
                "Benchmark performance",
            )],
        ],
        h4!["文件"],
        ul![li!["建立教學文件"], li!["簡化專案設定"],],
        h4!["痛點"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/issues/830",
                "元件模板",
            )],
            li!["Fetch API"],
            li!["Agents"],
        ],
    ])
);
