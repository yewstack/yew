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
            text(" 宏中："),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[function_component]
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

#[function_component]
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
        // No properties
        <MyComponent />

        // With Properties
        <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

        // With the whole set of props provided at once
        <MyComponentWithProps ..props.clone() />

        // With Properties from a variable and specific values overridden
        <MyComponentWithProps user_last_name="Elm" ..props />
    </>
};"#
        ),
        h2(vec![text("嵌套")]),
        p(vec![
            text("如果组件的 "),
            code("Properties"),
            text(" 中有 "),
            code("children"),
            text(
                " 字段，则可以被传递\\
                 u{5b50}组件。"
            ),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

#[function_component]
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
};"#
        ),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

#[function_component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

let props = yew::props!(Props {
    id: "container-2",
    children: Children::default(),
});

html! {
    <Container ..props>
        // props.children will be overwritten with this
        <span>{ "I am a child, as you can see" }</span>
    </Container>
};"#
        ),
        h2(vec![text("拥有 Props 的嵌套子组件")]),
        p(vec![
            text(
                "如果包含组件标注了 children 的类型，则可以访\\
                 u{95ee}和更改嵌套组件的属\\
                 u{6027}。在下面的示例中，"
            ),
            code("List"),
            text(" 组件可以包含 "),
            code("ListItem"),
            text(
                " 组件。有关此模式的\\
                 u{771f}实示例，请查看 "
            ),
            code("yew-router"),
            text(
                " 的源码。有关更高级\\
                 u{7684}示例，请在 yew 主仓库中查看 "
            ),
            code("nested-list"),
            text(" 示例代码。"),
        ]),
        code_block(
            "rust",
            r#"use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
    value: String,
}

#[function_component]
fn ListItem(props: &ListItemProps) -> Html {
    let ListItemProps { value } = props.clone();
    html! {
        <span>
            {value}
        </span>
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

#[function_component]
fn List(props: &Props) -> Html {
    let modified_children = props.children.iter().map(|mut item| {
            let mut props = Rc::make_mut(&mut item.props);
            props.value = format!("item-{}", props.value);
            item
    });
    html! { for modified_children }
}

html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
};"#
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
        ),
    ])
);
