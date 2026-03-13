pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![text("以前のスローガンをもう一度見てみましょう：")]),
        blockquote(vec![p(vec![text(
            "Yew の核心思想は、再利用可能な UI 部分に必要なすべての内容を 1 つの場所 - Rust ファイルに集中させることです。"
        )])]),
        p(vec![text("この声明を完成させるために、アプリケーションのロジックとレンダリングの動作を定義する概念を導入します：\"コンポーネント\"。")]),
        h2(vec![text("コンポーネントとは？")]),
        p(vec![text("コンポーネントは Yew の構成要素です。")]),
        p(vec![text("それらは次のことを行うべきです：")]),
        ul(vec![
            li(vec![
                link("/ja/docs/concepts/function-components/properties", vec![text("Props")]),
                text(" の形式でパラメータを受け取る"),
            ]),
            li(vec![text("独自の状態を持つことができる")]),
            li(vec![text("ユーザーに見える HTML フラグメント（DOM）を計算する")]),
        ]),
        h2(vec![text("Yew コンポーネントの 2 つのフレーバー")]),
        p(vec![text("現在、関数コンポーネントについて読んでいます - これは Yew を使い始めるときや、シンプルなレンダリングロジックを書くときにコンポーネントを書くための推奨方法です。")]),
        p(vec![
            text("もう一つの、より高度ですがアクセスしにくいコンポーネントの書き方があります - "),
            link("/ja/docs/advanced-topics/struct-components", vec![text("構造コンポーネント")]),
            text("。それらは非常に詳細な制御を可能にしますが、ほとんどの場合、そこまで詳細な制御は必要ありません。"),
        ]),
        h2(vec![text("関数コンポーネントの作成")]),
        p(vec![
            text("関数コンポーネントを作成するには、関数に "),
            code("#[component]"),
            text(" 属性を追加します。慣例として、関数の名前は PascalCase を使用し、"),
            code("html!"),
            text(" マクロ内の通常の html 要素と対比させます。"),
        ]),
        code_block("rust", r#"use yew::{component, html, Html};

#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// そして他の場所で、`html!` 内でコンポーネントを使用できます
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#),
        h2(vec![text("コンポーネント内部で何が起こっているのか")]),
        p(vec![
            text("レンダリング時に、Yew はこれらのコンポーネントの仮想ツリーを構築します。各（関数）コンポーネントの view 関数を呼び出して、DOM の仮想バージョン（VDOM）を計算します。ライブラリのユーザーとして、これを "),
            code("Html"),
            text(" 型として扱います。上記の例では、次のようになります："),
        ]),
        code_block("xhtml", r#"<App>
    <HelloWorld>
        <p>"Hello world"</p>
    </HelloWorld>
</App>"#),
        p(vec![
            text("更新が必要な場合、Yew は再び view 関数を呼び出し、新しい仮想 DOM を以前のバージョンと調整し、新しい/変更された/必要な部分のみを実際の DOM に伝播します。これが "),
            bold(vec![text("レンダリング")]),
            text(" と呼ばれるものです。"),
        ]),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![
                text("実際には、"),
                code("Html"),
                text(" は "),
                code("VNode"),
                text(" の別名に過ぎません - 仮想ノードです。"),
            ]),
        ]),
    ])
}

crate::doc_page!(
    "関数コンポーネント",
    "/ja/docs/concepts/function-components",
    page_content()
);
