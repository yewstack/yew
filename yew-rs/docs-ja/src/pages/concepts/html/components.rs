pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["基本"],
        p![
            "コンポーネントは ",
            code("html!"),
            " マクロで使用できます：",
        ],
        code_block("rust", r#"use yew::prelude::*;

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
        // プロパティなし
        <MyComponent />

        // プロパティを使用
        <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

        // すべてのプロパティを一度に提供
        <MyComponentWithProps ..props.clone() />

        // 変数のプロパティを使用し、特定の値を上書き
        <MyComponentWithProps user_last_name="Elm" ..props />
    </>
};"#),
        h2!["ネスト"],
        p![
            "コンポーネントの ",
            code("Properties"),
            " に ",
            code("children"),
            " フィールドがある場合、子コンポーネント/要素を受け入れることができます。",
        ],
        code_block_title("rust", "parent.rs", r#"use yew::prelude::*;

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
};"#),
        p![
            code("html!"),
            " マクロは、各プロパティを個別に指定するのではなく、基本式を ",
            code("..props"),
            " 構文で渡すことを可能にします。これは Rust の",
            link!["https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax", "関数的更新構文"],
            "に似ています。この基本式は、個別のプロパティを渡した後に現れる必要があります。",
            code("children"),
            " フィールドを持つ基本 props 式を渡す場合、",
            code("html!"),
            " マクロ内で渡された子要素は、props 内に既に存在する子要素を上書きします。",
        ],
        code_block("rust", r#"use yew::prelude::*;

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
        // 子要素は props.children を上書きします
        <span>{ "I am a child, as you can see" }</span>
    </Container>
};"#),
        h2!["参考例"],
        ul![
            li![link!["https://github.com/yewstack/yew/tree/master/examples/function_todomvc", "関数型 Todo MVC"]],
            li![link!["https://github.com/yewstack/yew/tree/master/examples/function_router", "関数型ルーティング"]],
        ],
    ])
}

crate::doc_page!(
    "コンポーネント",
    "/ja/docs/concepts/html/components",
    page_content()
);
