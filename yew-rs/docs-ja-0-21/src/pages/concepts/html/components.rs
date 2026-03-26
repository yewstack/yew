crate::doc_page!(
    "Components",
    "/ja/docs/concepts/html/components",
    Content::new(vec![
        h2!["基本"],
        p![
            code("Component"),
            "を実装しているあらゆる型は",
            code("html!"),
            "マクロの中で使えます:",
        ],
        code_block(
            "rust",
            r#"html!{
    <>
        // No properties
        <MyComponent />

        // With Properties
        <MyComponent prop1="lorem" prop2="ipsum" />

        // With the whole set of props provided at once
        <MyComponent ..props />

        // With Properties from a variable and specific values overridden
        <MyComponent prop2="lorem" ..props />
    </>
}"#
        ),
        h2!["ネスト"],
        p![
            code("children"),
            "フィールドが",
            code("Properties"),
            "の中にある場合はコンポーネントは子に渡されます。",
        ],
        code_block_title(
            "rust",
            "parent.rs",
            r#"html! {
    <Container>
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}"#
        ),
        code_block_title(
            "rust",
            "container.rs",
            r#"pub struct Container(Props);

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Container {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
       html! {
           <div id="container">
               { self.0.children.clone() }
           </div>
       }
    }
}"#
        ),
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("Properties"),
                "を継承した型は",
                code("Clone"),
                "を実装していなければいけません。これは",
                code("#[derive(Properties, Clone)]"),
                "を使うか手で",
                code("Clone"),
                "を実装すれば良いです。",
            ],
        ],
        h2!["Props とネストした子コンポーネント"],
        p!["ネストしたコンポーネントのプロパティは格納しているコンポーネントの型が子である場合は\
             アクセス可能、または変更可能です。以下の例では",
        code("List"),
        "コンポーネントは",
        code("ListItem"),
        "コンポーネントをラップできています。実際の使用においてこのパターンの例については",
        code("yew-router"),
        "のソースコードを確認してみてください。より進んだ例としては Yew のメインのリポジトリにある",
        code("nested-list"),
        "を確認してみてください。",
        ],
        code_block_title(
            "rust",
            "parent.rs",
            r#"html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
}"#
        ),
        code_block_title(
            "rust",
            "list.rs",
            r#"pub struct List(Props);

#[derive(Properties, Clone)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

impl Component for List {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
        html!{{
            for self.0.children.iter().map(|mut item| {
                item.props.value = format!("item-{}", item.props.value);
                item
            })
        }}
    }
}"#
        ),
    ])
);
