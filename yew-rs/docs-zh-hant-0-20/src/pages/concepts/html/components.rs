crate::doc_page!(
    "Components",
    "/zh-Hant/docs/concepts/html/components",
    Content::new(vec![
        h2![text("基本")],
        p![
            text("任何實作 "),
            code("Component"),
            text(" 的型別，都可以在 "),
            code("html!"),
            text(" 的巨集中使用："),
        ],
        code_block(
            "rust",
            r#"html!{
    <>
        // 沒有屬性
        <MyComponent />

        // 有屬性
        <MyComponent prop1="lorem" prop2="ipsum" />

        // 一次提供很多屬性
        <MyComponent ..props />
    </>
}"#,
        ),
        h2![text("巢狀")],
        p![
            text("只要元件的 "),
            code("Properties"),
            text(" 中有 "),
            code("children"),
            text("，就可以傳遞子結點給元件。"),
        ],
        code_block_title(
            "rust",
            "parent.rs",
            r#"html! {
    <Container>
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}"#,
        ),
        code_block_title(
            "rust",
            "container.rs",
            r#"pub struct Container(Props);

#[derive(Properties)]
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
}"#,
        ),
        h2![text("指定子結點的型別")],
        p![
            text("如果指定了子結點的型別，就可以使用或改變巢狀元件的屬性。下面的範例就是， "),
            code("List"),
            text(" 元件包裹 "),
            code("ListItem"),
            text(" 元件。另一個真實的範例是 "),
            code("yew-router"),
            text(" 的原始碼。還有一個更進階的範例，請參考 Yew GitHub repo 中的 "),
            code("nested-list"),
            text(" 範例。"),
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
}"#,
        ),
        code_block_title(
            "rust",
            "list.rs",
            r#"pub struct List(Props);

#[derive(Properties)]
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
}"#,
        ),
    ])
);
