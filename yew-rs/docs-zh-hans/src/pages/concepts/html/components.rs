pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["基础"],
        p![
            "组件可以在 ",
            code("html!"),
            " 宏中使用：",
        ],
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
        // 没有属性
        <MyComponent />

        // 使用属性
        <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

        // 一次性提供所有属性
        <MyComponentWithProps ..props.clone() />

        // 使用变量中的属性，并覆盖特定值
        <MyComponentWithProps user_last_name="Elm" ..props />
    </>
};"#,
        ),
        h2!["嵌套"],
        p![
            "如果组件在其 ",
            code("Properties"),
            " 中有一个 ",
            code("children"),
            " 字段，它可以接受子组件/元素",
        ],
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
        p![
            code("html!"),
            " 宏允许您使用 ",
            code("..props"),
            " 语法传递一个基本表达式，而不是单独指定每个属性，类似于 Rust 的",
            link!(
                "https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax",
                "函数式更新语法",
            ),
            "。 这个基本表达式必须出现在传递任何单独的 props 之后。 当传递一个带有 ",
            code("children"),
            " 字段的基本 props 表达式时，",
            code("html!"),
            " 宏中传递的子元素将覆盖已经存在于 props 中的子元素。",
        ],
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
        // 子元素将覆盖 props.children
        <span>{ "I am a child, as you can see" }</span>
    </Container>
};"#,
        ),
        h2!["相关示例"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/function_todomvc",
                "函数化 Todo MVC",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/function_router",
                "函数化路由",
            )],
        ],
    ])
}

crate::doc_page!(
    "组件",
    "/zh-Hans/docs/concepts/html/components",
    page_content()
);
