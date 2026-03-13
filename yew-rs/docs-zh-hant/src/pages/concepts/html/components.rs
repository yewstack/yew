pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("基礎")]),
        p(vec![
            text("元件可以在 "),
            code("html!"),
            text(" 巨集中使用："),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    html! {
        { "This component has no properties!" }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    user_first_name: String,
    user_last_name: String,
}

#[component]
fn MyComponentWithProps(props: &Props) -> Html {
    let Props { user_first_name, user_last_name } = props;
    html! {
        <>{"user_first_name: "}{user_first_name}{" and user_last_name: "}{user_last_name}</>
    }
}

let props = Props {
    user_first_name: "Bob".to_owned(),
    user_last_name: "Smith".to_owned(),
};

html!{
    <>
        // 沒有屬性
        <MyComponent />

        // 使用屬性
        <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

        // 一次提供所有屬性
        <MyComponentWithProps ..props.clone() />

        // 使用變數中的屬性，並覆寫特定值
        <MyComponentWithProps user_last_name="Elm" ..props />
    </>
};"#,
        ),
        h2(vec![text("嵌套")]),
        p(vec![
            text("如果組件在其 "),
            code("Properties"),
            text(" 中有一個 "),
            code("children"),
            text(" 字段，它可以接受子組件/元素"),
        ]),
        code_block_title(
            "rust",
            "parent.rs",
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Html,
}

#[component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

html! {
    <Container id="container">
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
};"#,
        ),
        p(vec![
            code("html!"),
            text(" 巨集允許您使用"),
            code("..props"),
            text(" 語法傳遞一個基本表達式，而不是單獨指定每個屬性，類似於Rust 的"),
            link(
                "https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax",
                vec![text("函數式更新語法")],
            ),
            text("。 這個基本表達式必須出現在傳遞任何單獨的 props 之後。 當傳遞一個帶有 "),
            code("children"),
            text(" 欄位的基本 props 表達式時，"),
            code("html!"),
            text(" 巨集中傳遞的子元素將覆蓋已經存在於 props 中的子元素。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Html,
}

#[component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

let props = yew::props!(Props {
    id: "container-2",
    children: Html::default(),
});

html! {
    <Container ..props>
        // 子元素將覆蓋 props.children
        <span>{ "I am a child, as you can see" }</span>
    </Container>
};"#,
        ),
        h2(vec![text("相關範例")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/function_todomvc",
                vec![text("函數化 Todo MVC")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/function_router",
                vec![text("函數化路由")],
            )]),
        ]),
    ])
}

crate::doc_page!(
    "組件",
    "/zh-Hant/docs/concepts/html/components",
    page_content()
);
