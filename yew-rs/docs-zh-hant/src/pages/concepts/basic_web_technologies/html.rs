pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "你可以使用 ",
            code("html!"),
            " 巨集來寫類似 HTML 的表達式。 Yew 會在背景轉換為表達 DOM 的 Rust 程式碼。",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let my_header: Html = html! {
    <img src="img_girl.jpg" alt="Girl in a jacket" width="500" height="600" />
};"#,
        ),
        p!["類似於格式化表達式，您可以透過使用花括號將周圍上下文的值嵌入 HTML 中："],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let header_text = "Hello world".to_string();
let header_html: Html = html! {
    <h1>{header_text}</h1>
};

let count: usize = 5;
let counter_html: Html = html! {
    <p>{"My age is: "}{count}</p>
};

let combined_html: Html = html! {
    <div>{header_html}{counter_html}</div>
};"#,
        ),
        p![
            "使用 ",
            code("html!"),
            " 有一個重要的規則 - 您只能傳回一個包裝節點。為了渲染多個元素的列表，",
            code("html!"),
            " 允許使用空標籤（Fragments）。空標籤是沒有名稱的標籤，它們本身不會產生 HTML 元素。",
        ],
        tabs!(
            "Invalid",
            tab!(
                "Invalid",
                "Invalid",
                code_block(
                    "rust",
                    r#"use yew::html;

// 錯誤：只允許一個根 HTML 元素
html! {

    <div></div>
    <p></p>

};"#,
                ),
            ),
            tab!(
                "Valid",
                "Valid",
                code_block(
                    "rust",
                    r#"use yew::html;

// 修正：使用 HTML 空標籤包裹
html! {
    <>
        <div></div>
        <p></p>
    </>
};"#,
                ),
            ),
        ),
        p![
            "更多關於 Yew 和 HTML 的內容請參考",
            link!("/zh-Hant/docs/concepts/html/introduction", "更多 HTML"),
            "。",
        ],
    ])
}

crate::doc_page!(
    "使用 html! 巨集處理 HTML",
    "/zh-Hant/docs/concepts/basic-web-technologies/html",
    page_content()
);
