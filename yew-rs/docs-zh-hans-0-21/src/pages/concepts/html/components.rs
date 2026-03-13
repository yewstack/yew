crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/components",
    Content::new(vec![
        h1(vec![text("组件")]),
        h2(vec![text("基础")]),
        p(vec![
            text("任何实现了 "),
            code("Component"),
            text(" trait 的类型都可被用在 "),
            code("html!"),
            text(" 宏中：")
        ]),
        code_block(
            "rust",
            r#"html!{
    <>
        // 没有属性
        <MyComponent />

        // 具有属性
        <MyComponent prop1="lorem" prop2="ipsum" />

        // 同时提供全套的 props
        <MyComponent ..props />
    </>
}"#
        ),
        h2(vec![text("嵌套")]),
        p(vec![
            text("如果组件的 "),
            code("Properties"),
            text(" 中有 "),
            code("children"),
            text(" 字段，则可以被传递子组件。")
        ]),
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
}"#
        ),
        h2(vec![text("拥有 Props 的嵌套子组件")]),
        p(vec![
            text(
                "如果包含组件标注了 children \
                 的类型，则可以访问和更改嵌套组件的属性。在下面的示例中，"
            ),
            code("List"),
            text(" 组件可以包含 "),
            code("ListItem"),
            text(" 组件。有关此模式的真实示例，请查看 "),
            code("yew-router"),
            text(" 的源码。有关更高级的示例，请在 yew 主仓库中查看 "),
            code("nested-list"),
            text(" 示例代码。")
        ]),
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
}"#
        )
    ])
);
