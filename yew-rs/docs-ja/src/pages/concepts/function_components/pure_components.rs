pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("すべての関数コンポーネントは、プロパティオブジェクトを受け取り、"),
            code("Html"),
            text(" オブジェクトを返す"),
            link!["https://ja.wikipedia.org/wiki/%E7%B4%94%E9%96%A2%E6%95%B0", text("純粋")],
            text("関数です。純粋関数とは、同じ入力に対して常に同じ出力を返す関数のことです。"),
        ],
        p![
            text("この例は純粋コンポーネントです。与えられたプロパティ "),
            code("is_loading"),
            text(" に対して、常に同じ "),
            code("Html"),
            text(" を返し、副作用はありません。"),
        ],
        code_block("rust", r#"use yew::{Properties, component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}"#),
        admonition![AdmonitionType::Note, None,
            p![
                text("内部の純粋コンポーネントがフックや他のコンポーネントメカニズムを使用しない場合、それを "),
                code("Html"),
                text(" を返す通常の関数として記述することができ、Yew がコンポーネントのライフサイクルに関連するオーバーヘッドを回避することができます。"),
                link!["/ja/docs/concepts/html/literals-and-expressions#expressions", text("式構文")],
                text(" を使用して "),
                code("html!"),
                text(" 内でそれらをレンダリングします。"),
            ],
        ],
        h2![text("非純粋コンポーネント")],
        p![
            text("コンポーネントがグローバル変数を使用しない場合、それが「純粋」でない可能性があるかどうか疑問に思うかもしれません。なぜなら、それは毎回レンダリングされる固定関数として呼び出されるだけだからです。 これが次のトピック - "),
            link!["/ja/docs/concepts/function-components/hooks", text("フック")],
            text(" の出番です。"),
        ],
    ])
}

crate::doc_page!(
    "純粋コンポーネント",
    "/ja/docs/concepts/function-components/pure-components",
    page_content()
);
