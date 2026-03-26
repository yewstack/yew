pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "你可以使用 ",
            code("html!"),
            " 宏编写类似 HTML 的表达式。Yew 会在后台将其转换为表达 DOM 的 Rust 代码。",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let my_header: Html = html! {
    <img src="img_girl.jpg" alt="Girl in a jacket" width="500" height="600" />
};"#,
        ),
        p!["类似于格式化表达式，您可以通过使用花括号将周围上下文的值嵌入 HTML 中："],
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
            " 有一个重要的规则 - 您只能返回一个包装节点。为了渲染多个元素的列表，",
            code("html!"),
            " 允许使用空标签（Fragments）。空标签是没有名称的标签，它们本身不会产生 HTML 元素。",
        ],
        tabs![
            "Invalid",
            tab![
                "Invalid",
                "Invalid",
                code_block(
                    "rust",
                    r#"use yew::html;

// 错误：只允许一个根 HTML 元素
html! {

    <div></div>
    <p></p>

};"#,
                ),
            ],
            tab![
                "Valid",
                "Valid",
                code_block(
                    "rust",
                    r#"use yew::html;

// 修复：使用 HTML 空标签包裹
html! {
    <>
        <div></div>
        <p></p>
    </>
};"#,
                ),
            ],
        ],
        p![
            "更多关于 Yew 和 HTML 的内容请参见",
            doc_link!(crate::pages::concepts::html::introduction, "更多 HTML"),
            "。",
        ],
    ])
}

crate::doc_page!(
    "使用 html! 宏处理 HTML",
    "/zh-Hans/docs/concepts/basic-web-technologies/html",
    page_content()
);
