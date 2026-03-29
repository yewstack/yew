crate::doc_page!("Components", "/ja/docs/concepts/html/components",
    Content::new(vec![
        h2!["Basic"],
        p![
            "Components can be used in the ",
            code("html!"),
            " macro:",
        ],
        code_block("rust", r#"use yew::prelude::*;

#[function_component(MyComponent)]
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

#[function_component(MyComponentWithProps)]
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
};"#),
        h2!["Nested"],
        p![
            "Components can be passed children if they have a ",
            code("children"),
            " field in their ",
            code("Properties"),
            ".",
        ],
        code_block_title("rust", "parent.rs", r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
id: String,
children: Children,
}

#[function_component(Container)]
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
            "The ",
            code("html!"),
            " macro allows you to pass a base expression with the ",
            code("..props"),
            " syntax instead of specifying each property individually, \
              similar to Rust's ",
            link!["https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax", "Functional Update Syntax"],
            ". This base expression must occur after any individual props are passed. \
              When passing a base props expression with a ",
            code("children"),
            " field, the children passed in the ",
            code("html!"),
            " macro overwrite the ones already present in the props.",
        ],
        code_block("rust", r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
id: String,
children: Children,
}

#[function_component(Container)]
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
};"#),
        h2!["Nested Children with Props"],
        p![
            "Nested component properties can be accessed and mutated if the containing component types its children. \
              In the following example, the ",
            code("List"),
            " component can wrap ",
            code("ListItem"),
            " components. For a real world example of this pattern, check out the ",
            code("yew-router"),
            " source code. For a more advanced example, check out the ",
            code("nested-list"),
            " example in the main yew repository.",
        ],
        code_block("rust", r#"use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
value: String,
}

#[function_component(ListItem)]
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

#[function_component(List)]
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
};"#),
        h2!["Relevant examples"],
        ul![
            li![link!["https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/function_todomvc", "Function Todo MVC"]],
            li![link!["https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/function_router", "Function Router"]],
        ],
    ])
    .with_description("Create complex layouts with component hierarchies")
);
