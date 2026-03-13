pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("イテレータ")]),
        p(vec![text(
            "イテレータから HTML を構築する方法は 3 つあります：",
        )]),
        tabs(
            "`for` loops",
            vec![
                tab(
                    "`for` loops",
                    "`for` ループ",
                    vec![
                        p(vec![text(
                            "主なアプローチは for ループを使用することです。これは Rust \
                             に既に存在する for ループと同じですが、2 つの重要な違いがあります：",
                        )]),
                        ol(vec![
                            li(vec![
                                text("通常の for ループは何も返せませんが、"),
                                code("html!"),
                                text(" 内の for ループはノードのリストに変換されます。"),
                            ]),
                            li(vec![
                                code("break"),
                                text("、"),
                                code("continue"),
                                text(" などの発散式は "),
                                code("html!"),
                                text(" 内の for ループの本体では許可されていません。"),
                            ]),
                        ]),
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

html! {
    for i in 0 .. 10 {
        <span>{i}</span>
    }
};"#,
                        ),
                    ],
                ),
                tab(
                    "`for` block",
                    "`for` ブロック",
                    vec![
                        p(vec![
                            text("もう一つの方法は "),
                            code("for"),
                            text(
                                " キーワードを使用することです。これはネイティブの Rust \
                                 構文ではなく、HTML \
                                 マクロによってイテレータを表示するために必要なコードを出力します。\
                                 この方法は、イテレータが既に計算されていて、\
                                 マクロに渡すだけでよい場合に最初の方法より適しています。",
                            ),
                        ]),
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { for items.iter() }
    </ul>
};"#,
                        ),
                    ],
                ),
                tab(
                    "`collect` method",
                    "`collect` メソッド",
                    vec![
                        p(vec![
                            text("最後の方法は、イテレータの最終変換で "),
                            code("collect::<Html>()"),
                            text(" を呼び出すことで、Yew が表示できるリストを返します。"),
                        ]),
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { items.iter().collect::<Html>() }
    </ul>
};"#,
                        ),
                    ],
                ),
            ],
        ),
        h2(vec![text("キー付きリスト")]),
        p(vec![
            text("キー付きリストは、すべての子要素にキーがある最適化されたリストです。 "),
            code("key"),
            text(
                " は Yew が提供する特別な属性で、HTML \
                 要素やコンポーネントに一意の識別子を与え、Yew 内部での最適化に使用されます。",
            ),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("キーは各リスト内で一意である必要があり、HTML の "),
                code("id"),
                text(
                    " のようにグローバルに一意である必要はありません。\
                     リストの順序に依存してはいけません。",
                ),
            ])],
        ),
        p(vec![text("リストにキーを追加することを常にお勧めします。")]),
        p(vec![
            text("一意の "),
            code("String"),
            text("、"),
            code("str"),
            text("、または整数を特別な "),
            code("key"),
            text(" 属性に渡すことでキーを追加できます。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let names = vec!["Sam","Bob","Ray"]

html! {
    <div id="introductions">
        {
            names.into_iter().map(|name| {
                html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
            }).collect::<Html>()
        }
    </div>
};
"#,
        ),
        h3(vec![text("パフォーマンスの最適化")]),
        p(vec![
            text("キー付きリストのパフォーマンス向上をテストするための"),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                vec![text("例")],
            ),
            text("があります。以下は簡単なテスト手順です："),
        ]),
        ol(vec![
            li(vec![
                link(
                    "https://examples.yew.rs/keyed_list",
                    vec![text("オンラインデモ")],
                ),
                text("にアクセスします。"),
            ]),
            li(vec![text("500個の要素を追加します。")]),
            li(vec![text("キーを無効にします。")]),
            li(vec![text("リストを反転します。")]),
            li(vec![text(
                "\"最後のレンダリングにかかった時間 Xms\" \
                 を確認します（この記事の執筆時点では約60ms）。",
            )]),
            li(vec![text("キーを有効にします。")]),
            li(vec![text("再度リストを反転します。")]),
            li(vec![text(
                "\"最後のレンダリングにかかった時間 Xms\" \
                 を確認します（この記事の執筆時点では約30ms）。",
            )]),
        ]),
        p(vec![text(
            "この記事の執筆時点では、500個のコンポーネントに対して速度が2倍に向上しました。",
        )]),
        h3(vec![text("原理の説明")]),
        p(vec![text(
            "通常、リストを反復処理する際には、各リスト項目にキーを追加するだけで済みます。\
             データの順序が変わる可能性があるためです。 \
             リストを再レンダリングする際に、キーは調整プロセスを高速化するために使用されます。",
        )]),
        p(vec![
            text("キーがない場合、例えば "),
            code("[\"bob\", \"sam\", \"rob\"]"),
            text(" を反復処理すると、最終的に以下のようなHTMLが生成されます："),
        ]),
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#,
        ),
        p(vec![
            text("次のレンダリングでリストが "),
            code("[\"bob\", \"rob\"]"),
            text(
                " に変更された場合、Yew は id=\"rob\" の要素を削除し、id=\"sam\" を id=\"rob\" \
                 に更新できます。",
            ),
        ]),
        p(vec![
            text("各要素にキーを追加すると、初期の HTML は変わりませんが、変更後のリスト "),
            code("[\"bob\", \"rob\"]"),
            text(
                " をレンダリングすると、Yew は2番目の HTML \
                 要素のみを削除し、他の要素はそのまま残ります。\
                 キーを使用して要素を関連付けることができるためです。",
            ),
        ]),
        p(vec![text(
            "コンポーネントから別のコンポーネントに切り替える際に、\
             両方に最高レンダリング要素として div がある場合にバグ/\"機能\" に遭遇した場合。 Yew \
             はこれらの状況で最適化として既にレンダリングされた HTML div を再利用します。 その \
             div を再利用せずに再作成する必要がある場合は、\
             異なるキーを追加することで再利用されなくなります。",
        )]),
        h2(vec![text("さらなる読み物")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/todomvc",
                vec![text("TodoMVC の例")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                vec![text("キー付きリストの例")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/router",
                vec![text("ルーティングの例")],
            )]),
        ]),
    ])
}

crate::doc_page!("リスト", "/ja/docs/concepts/html/lists", page_content());
