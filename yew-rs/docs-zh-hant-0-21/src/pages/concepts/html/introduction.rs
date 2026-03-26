crate::doc_page!(
    "使用 html!",
    "/zh-Hant/docs/concepts/html",
    Content::new(vec![
        p![
            code("html!"),
            " 巨集可以讓你用 HTML 與 SVG 寫元件。如果你寫過 React 的 JSX（一種 JavaScript \
             的擴展，可以讓你在 JavaScript 中寫 HTML），應該會覺得這兩者十分相似。",
        ],
        p![bold!["重要提示"]],
        ol![
            li![
                "在 ",
                code("html!"),
                " 裡，只能有一個根結點（但你可以用 ",
                link!(
                    "https://yew.rs/concepts/html/lists",
                    "Fragment 或是 Iterators",
                ),
                " 來繞過這個限制。）",
            ],
            li![
                "空的 ",
                code("html! {}"),
                " 是合法的，且他不會渲染任何東西在畫面上",
            ],
            li![
                "字串必須被雙引號與大括號包裹住：",
                code("html! { \"Hello, World\" }"),
            ],
        ],
    ])
);
