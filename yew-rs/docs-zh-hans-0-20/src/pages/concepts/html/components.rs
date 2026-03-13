crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/components",
    Content::new(vec![
        h1(vec![text("\u{7ec4}\u{4ef6}")]),
        h2(vec![text("\u{57fa}\u{7840}")]),
        p(vec![
            text("\u{4efb}\u{4f55}\u{5b9e}\u{73b0}\u{4e86} "),
            code("Component"),
            text(" trait \u{7684}\u{7c7b}\u{578b}\u{90fd}\u{53ef}\u{88ab}\u{7528}\u{5728} "),
            code("html!"),
            text(" \u{5b8f}\u{4e2d}\u{ff1a}"),
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
        h2(vec![text("\u{5d4c}\u{5957}")]),
        p(vec![
            text("\u{5982}\u{679c}\u{7ec4}\u{4ef6}\u{7684} "),
            code("Properties"),
            text(" \u{4e2d}\u{6709} "),
            code("children"),
            text(
                " \u{5b57}\u{6bb5}\u{ff0c}\u{5219}\u{53ef}\u{4ee5}\u{88ab}\u{4f20}\u{9012}\\
                 u{5b50}\u{7ec4}\u{4ef6}\u{3002}"
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
        h2(vec![text(
            "\u{62e5}\u{6709} Props \u{7684}\u{5d4c}\u{5957}\u{5b50}\u{7ec4}\u{4ef6}"
        )]),
        p(vec![
            text(
                "\u{5982}\u{679c}\u{5305}\u{542b}\u{7ec4}\u{4ef6}\u{6807}\u{6ce8}\u{4e86} \
                 children \u{7684}\u{7c7b}\u{578b}\u{ff0c}\u{5219}\u{53ef}\u{4ee5}\u{8bbf}\\
                 u{95ee}\u{548c}\u{66f4}\u{6539}\u{5d4c}\u{5957}\u{7ec4}\u{4ef6}\u{7684}\u{5c5e}\\
                 u{6027}\u{3002}\u{5728}\u{4e0b}\u{9762}\u{7684}\u{793a}\u{4f8b}\u{4e2d}\u{ff0c}"
            ),
            code("List"),
            text(" \u{7ec4}\u{4ef6}\u{53ef}\u{4ee5}\u{5305}\u{542b} "),
            code("ListItem"),
            text(
                " \u{7ec4}\u{4ef6}\u{3002}\u{6709}\u{5173}\u{6b64}\u{6a21}\u{5f0f}\u{7684}\\
                 u{771f}\u{5b9e}\u{793a}\u{4f8b}\u{ff0c}\u{8bf7}\u{67e5}\u{770b} "
            ),
            code("yew-router"),
            text(
                " \u{7684}\u{6e90}\u{7801}\u{3002}\u{6709}\u{5173}\u{66f4}\u{9ad8}\u{7ea7}\\
                 u{7684}\u{793a}\u{4f8b}\u{ff0c}\u{8bf7}\u{5728} yew \
                 \u{4e3b}\u{4ed3}\u{5e93}\u{4e2d}\u{67e5}\u{770b} "
            ),
            code("nested-list"),
            text(" \u{793a}\u{4f8b}\u{4ee3}\u{7801}\u{3002}"),
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
