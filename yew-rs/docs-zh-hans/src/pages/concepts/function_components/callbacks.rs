pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "回调函数用于在组件树中向上传递信息，以及在事件处理期间与其他组件（如代理或 \
             DOM）进行通信。在内部，回调函数的类型只是一个 ",
            code("Fn"),
            "，并且被包装在 ",
            code("Rc"),
            " 中，以便它们可以被廉价地克隆。",
        ],
        p![
            "如果您想手动调用回调函数，可以使用 ",
            code("emit"),
            " 函数。",
        ],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html, Callback};

let cb: Callback<String, String> = Callback::from(move |name: String| {
    format!("Bye {}", name)
});

let result = cb.emit(String::from("Bob"));  // 调用回调函数
// web_sys::console::log_1(&result.into()); // 如果取消注释，将打印 "Bye Bob""#,
        ),
        h2!["将回调函数作为属性传递"],
        p!["在 yew 中的一个常见模式是创建一个回调函数，并将其作为属性传递给子组件。"],
        code_block(
            "rust",
            r#"use yew::{component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[component]
fn HelloWorld(props: &Props) -> Html {

    props.on_name_entry.emit(String::from("Bob"));

    html! { "Hello" }
}

// 然后提供属性 (Props)
#[component]
fn App() -> Html {
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("Hey, {}!", name);
        // web_sys::console::log_1(&greeting.into()); // 如果取消注释，这里会打印文本
    });

    html! { <HelloWorld {on_name_entry} /> }
}"#,
        ),
        h2!["DOM 事件和回调函数"],
        p!["回调函数也用于连接到 DOM 事件。"],
        p!["例如，这里我们定义了一个回调函数，当用户点击按钮时将会调用："],
        code_block(
            "rust",
            r#"use yew::{component, html, Html, Properties, Callback};

#[component]
fn App() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        // web_sys::console::log_1(&greeting.into()); // 如果取消注释，这里会打印文本
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}"#,
        ),
    ])
}

crate::doc_page!(
    "回调 (Callbacks)",
    "/zh-Hans/docs/concepts/function-components/callbacks",
    page_content()
);
