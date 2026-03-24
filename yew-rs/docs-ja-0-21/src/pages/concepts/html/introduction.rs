crate::doc_page!("Introduction", "/ja/docs/concepts/html",
    Content::new(vec![
        p![
            code("html!"),
            text("マクロによって HTML と SVG のコードを宣言的に書くことができます。\
                  JSX (HTML のようなコードを JavaScript 内部に書くことができる JavaScript の拡張) に似ています。"),
        ],
        p![bold![text("重要な注意")]],
        ol![
            li![
                code("html!"),
                text("マクロはルートの HTML ノードのみ受け付けます ("),
                link!["/ja/docs/concepts/html/lists", text("フラグメントかイテレータを使う")],
                text("ことでやり取りできます)"),
            ],
            li![
                text("空の"),
                code("html! {}"),
                text("の呼び出しは可能ですが何もレンダリングしません"),
            ],
            li![
                text("リテラルはクオーテーションがつけられ、ブレースで囲う必要があります: "),
                code("html! { \"Hello, World\" }"),
            ],
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("html!"),
                text("マクロはコンパイラのデフォルトの再帰の上限に簡単に達してしまいます。\
                      もしコンパイラエラーに遭遇した場合はその値を押し出すといいかもしれません。\
                      クレートのルート(つまり、"),
                code("lib.rs"),
                text("か"),
                code("main.rs"),
                text(")で"),
                code("#![recursion_limit=\"1024\"]"),
                text("のような属性を使えば解決します。"),
            ],
            p![
                text("詳しくは"),
                link![
                    "https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute",
                    text("公式ドキュメント"),
                ],
                text("と"),
                link![
                    "https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it",
                    text("Stack Overflow の質問"),
                ],
                text("を見てみてください。"),
            ],
        ],
    ])
);
