crate::doc_page!("Introduction", "/ja/docs/concepts/html",
    Content::new(vec![
        p![
            code("html!"),
            "マクロによって HTML と SVG のコードを宣言的に書くことができます。\
                  JSX (HTML のようなコードを JavaScript 内部に書くことができる JavaScript の拡張) に似ています。",
        ],
        p![bold!["重要な注意"]],
        ol![
            li![
                code("html!"),
                "マクロはルートの HTML ノードのみ受け付けます (",
                doc_link![crate::pages::concepts::html::lists, "フラグメントかイテレータを使う"],
                "ことでやり取りできます)",
            ],
            li![
                "空の",
                code("html! {}"),
                "の呼び出しは可能ですが何もレンダリングしません",
            ],
            li![
                "リテラルはクオーテーションがつけられ、ブレースで囲う必要があります: ",
                code("html! { \"Hello, World\" }"),
            ],
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("html!"),
                "マクロはコンパイラのデフォルトの再帰の上限に簡単に達してしまいます。\
                      もしコンパイラエラーに遭遇した場合はその値を押し出すといいかもしれません。\
                      クレートのルート(つまり、",
                code("lib.rs"),
                "か",
                code("main.rs"),
                ")で",
                code("#![recursion_limit=\"1024\"]"),
                "のような属性を使えば解決します。",
            ],
            p![
                "詳しくは",
                link![
                    "https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute",
                    "公式ドキュメント",
                ],
                "と",
                link![
                    "https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it",
                    "Stack Overflow の質問",
                ],
                "を見てみてください。",
            ],
        ],
    ])
    .with_description("The procedural macro for generating HTML and SVG")
);
