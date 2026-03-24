pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("html!"),
            text(
                " マクロを使用して、HTML に似た式を記述できます。Yew はバックグラウンドでそれを \
                 DOM を表現する Rust コードに変換します。",
            ),
        ],
        code_block(
            "rust",
            "use yew::prelude::*;

let my_header: Html = html! {
    <img src=\"img_girl.jpg\" alt=\"Girl in a jacket\" width=\"500\" height=\"600\" />
};",
        ),
        p![text(
            "フォーマットされた式と同様に、波括弧を使用して周囲のコンテキストの値を HTML \
             に埋め込むことができます：",
        )],
        code_block(
            "rust",
            "use yew::prelude::*;

let header_text = \"Hello world\".to_string();
let header_html: Html = html! {
    <h1>{header_text}</h1>
};

let count: usize = 5;
let counter_html: Html = html! {
    <p>{\"My age is: \"}{count}</p>
};

let combined_html: Html = html! {
    <div>{header_html}{counter_html}</div>
};",
        ),
        p![
            code("html!"),
            text(
                " を使用する際の重要なルールの 1 つは、1 \
                 つのラッピングノードしか返せないということです。\
                 複数の要素のリストをレンダリングするために、",
            ),
            code("html!"),
            text(
                " は空のタグ（フラグメント）の使用を許可しています。空のタグは名前のないタグで、\
                 それ自体は HTML 要素を生成しません。",
            ),
        ],
        tabs(
            "Invalid",
            vec![
                tab(
                    "Invalid",
                    "Invalid",
                    vec![code_block(
                        "rust",
                        r#"use yew::html;

// エラー：ルート HTML 要素は1つだけ許可されています
html! {

    <div></div>
    <p></p>

};"#,
                    )],
                ),
                tab(
                    "Valid",
                    "Valid",
                    vec![code_block(
                        "rust",
                        r#"use yew::html;

// 修正：HTML 空のタグを使用してラップする
html! {
    <>
        <div></div>
        <p></p>
    </>
};"#,
                    )],
                ),
            ],
        ),
        p![
            text("詳細については、"),
            link!("/ja/docs/concepts/html", text("HTML の詳細")),
            text("を参照してください。"),
        ],
    ])
}

crate::doc_page!(
    "html! マクロを使用してHTMLを処理する",
    "/ja/docs/concepts/basic-web-technologies/html",
    page_content()
);
