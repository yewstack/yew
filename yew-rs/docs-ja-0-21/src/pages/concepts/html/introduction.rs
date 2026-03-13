crate::doc_page!("Introduction", "/ja/docs/concepts/html",
    Content::new(vec![
        p(vec![
            code("html!"),
            text("マクロによって HTML と SVG のコードを宣言的に書くことができます。\
                  JSX (HTML のようなコードを JavaScript 内部に書くことができる JavaScript の拡張) に似ています。"),
        ]),
        p(vec![bold(vec![text("重要な注意")])]),
        ol(vec![
            li(vec![
                code("html!"),
                text("マクロはルートの HTML ノードのみ受け付けます ("),
                link("/ja/docs/concepts/html/lists", vec![text("フラグメントかイテレータを使う")]),
                text("ことでやり取りできます)"),
            ]),
            li(vec![
                text("空の"),
                code("html! {}"),
                text("の呼び出しは可能ですが何もレンダリングしません"),
            ]),
            li(vec![
                text("リテラルはクオーテーションがつけられ、ブレースで囲う必要があります: "),
                code("html! { \"Hello, World\" }"),
            ]),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![
                p(vec![
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
                ]),
                p(vec![
                    text("詳しくは"),
                    link(
                        "https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute",
                        vec![text("公式ドキュメント")]
                    ),
                    text("と"),
                    link(
                        "https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it",
                        vec![text("Stack Overflow の質問")]
                    ),
                    text("を見てみてください。"),
                ]),
            ]
        ),
    ])
);
