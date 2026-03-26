pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["イテレータ"],
        p!["イテレータから HTML を構築する方法は 3 つあります："],
        tabs![
            "`for` loops",
            tab![
                "`for` loops",
                "`for` ループ",
                p![
                    "主なアプローチは for ループを使用することです。これは Rust に既に存在する \
                     for ループと同じですが、2 つの重要な違いがあります："
                ],
                ol![
                    li![
                        "通常の for ループは何も返せませんが、",
                        code("html!"),
                        " 内の for ループはノードのリストに変換されます。",
                    ],
                    li![
                        code("break"),
                        "、",
                        code("continue"),
                        " などの発散式は ",
                        code("html!"),
                        " 内の for ループの本体では許可されていません。",
                    ],
                ],
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
            tab![
                "`for` block",
                "`for` ブロック",
                p![
                    "もう一つの方法は ",
                    code("for"),
                    " キーワードを使用することです。これはネイティブの Rust 構文ではなく、HTML \
                     マクロによってイテレータを表示するために必要なコードを出力します。\
                     この方法は、イテレータが既に計算されていて、\
                     マクロに渡すだけでよい場合に最初の方法より適しています。",
                ],
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
            tab![
                "`collect` method",
                "`collect` メソッド",
                p![
                    "最後の方法は、イテレータの最終変換で ",
                    code("collect::<Html>()"),
                    " を呼び出すことで、Yew が表示できるリストを返します。",
                ],
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
        ],
        h2!["キー付きリスト"],
        p![
            "キー付きリストは、すべての子要素にキーがある最適化されたリストです。 ",
            code("key"),
            " は Yew が提供する特別な属性で、HTML 要素やコンポーネントに一意の識別子を与え、Yew \
             内部での最適化に使用されます。",
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "キーは各リスト内で一意である必要があり、HTML の ",
                code("id"),
                " のようにグローバルに一意である必要はありません。\
                 リストの順序に依存してはいけません。",
            ],
        ],
        p!["リストにキーを追加することを常にお勧めします。"],
        p![
            "一意の ",
            code("String"),
            "、",
            code("str"),
            "、または整数を特別な ",
            code("key"),
            " 属性に渡すことでキーを追加できます。",
        ],
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
        h3!["パフォーマンスの最適化"],
        p![
            "キー付きリストのパフォーマンス向上をテストするための",
            link![
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "例",
            ],
            "があります。以下は簡単なテスト手順です：",
        ],
        ol![
            li![
                link!["https://examples.yew.rs/keyed_list", "オンラインデモ",],
                "にアクセスします。",
            ],
            li!["500個の要素を追加します。"],
            li!["キーを無効にします。"],
            li!["リストを反転します。"],
            li!["\"最後のレンダリングにかかった時間 Xms\" \
                 を確認します（この記事の執筆時点では約60ms）。"],
            li!["キーを有効にします。"],
            li!["再度リストを反転します。"],
            li!["\"最後のレンダリングにかかった時間 Xms\" \
                 を確認します（この記事の執筆時点では約30ms）。"],
        ],
        p!["この記事の執筆時点では、500個のコンポーネントに対して速度が2倍に向上しました。"],
        h3!["原理の説明"],
        p![
            "通常、リストを反復処理する際には、各リスト項目にキーを追加するだけで済みます。\
             データの順序が変わる可能性があるためです。 \
             リストを再レンダリングする際に、キーは調整プロセスを高速化するために使用されます。"
        ],
        p![
            "キーがない場合、例えば ",
            code("[\"bob\", \"sam\", \"rob\"]"),
            " を反復処理すると、最終的に以下のようなHTMLが生成されます：",
        ],
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#,
        ),
        p![
            "次のレンダリングでリストが ",
            code("[\"bob\", \"rob\"]"),
            " に変更された場合、Yew は id=\"rob\" の要素を削除し、id=\"sam\" を id=\"rob\" \
             に更新できます。",
        ],
        p![
            "各要素にキーを追加すると、初期の HTML は変わりませんが、変更後のリスト ",
            code("[\"bob\", \"rob\"]"),
            " をレンダリングすると、Yew は2番目の HTML \
             要素のみを削除し、他の要素はそのまま残ります。\
             キーを使用して要素を関連付けることができるためです。",
        ],
        p!["コンポーネントから別のコンポーネントに切り替える際に、\
            両方に最高レンダリング要素として div がある場合にバグ/\"機能\" に遭遇した場合。 Yew \
            はこれらの状況で最適化として既にレンダリングされた HTML div を再利用します。 その \
            div を再利用せずに再作成する必要がある場合は、\
            異なるキーを追加することで再利用されなくなります。"],
        h2!["さらなる読み物"],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/todomvc",
                "TodoMVC の例",
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "キー付きリストの例",
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/router",
                "ルーティングの例",
            ]],
        ],
    ])
}

crate::doc_page!("リスト", "/ja/docs/concepts/html/lists", page_content());
