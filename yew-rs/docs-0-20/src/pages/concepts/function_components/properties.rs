crate::doc_page!(
    "Properties",
    "/docs/concepts/function-components/properties",
    Content::new(vec![
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text(
                "Properties are often shortened as \"Props\"."
            )]),]
        ),
        p(vec![text(
            "Properties are essentially component arguments that Yew can keep watch on."
        )]),
        p(vec![
            text("A type has to implement the "),
            code("Properties"),
            text(" trait before it can be used as the properties of a component."),
        ]),
        h2(vec![text("Reactivity")]),
        p(vec![text(
            "Yew checks if props have changed when reconciling the vdom during rerendering, to \
             know if nested components needs to be rerendered. This way Yew can be considered a \
             very reactive framework as changes from the parent will always be propagated \
             downwards and the view will never be out of sync from the data coming from \
             props/state."
        ),]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("If you have not yet completed the "),
                link("/docs/tutorial", vec![text("tutorial")]),
                text(", try it out and test this reactivity yourself!"),
            ]),]
        ),
        h2(vec![text("Derive macro")]),
        p(vec![
            text("Yew provides a derive macro to easily implement the "),
            code("Properties"),
            text(" trait on structs."),
        ]),
        p(vec![
            text("Types for which you derive "),
            code("Properties"),
            text(" must also implement "),
            code("PartialEq"),
            text(" so Yew can do data comparison."),
        ]),
        code_block(
            "rust",
            r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#
        ),
        h2(vec![text("Use in function components")]),
        p(vec![
            text("The attribute "),
            code("#[function_component]"),
            text(
                " allows to optionally receive Props in the function arguments. To supply them, \
                 they are assigned via attributes in the "
            ),
            code("html!"),
            text(" macro."),
        ]),
        h3(vec![text("With Props")]),
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// Then supply the prop
#[function_component]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"##
        ),
        h3(vec![text("No Props")]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html};





#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// No props to supply
#[function_component]
fn App() -> Html {
    html! {<HelloWorld />}
}
"#
        ),
        h2(vec![text("Derive macro field attributes")]),
        p(vec![
            text("When deriving "),
            code("Properties"),
            text(
                " all fields are required by default. The following attributes allow you to give \
                 your props default values which will be used when parent has not set them."
            ),
        ]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![text(
                "Attributes aren't visible in Rustdoc generated documentation. The doc strings of \
                 your properties should mention whether a prop is optional and if it has a \
                 special default value."
            ),]),]
        ),
        h3(vec![text("#[prop_or_default]")]),
        p(vec![
            text("Initialize the prop value with the default value of the field's type using the "),
            code("Default"),
            text(" trait."),
        ]),
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    // highlight-start
    #[prop_or_default]
    // highlight-end
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading.clone() {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}

// Then use like this with default
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"##
        ),
        h3(vec![text("#[prop_or(value)]")]),
        p(vec![
            text("Use "),
            code("value"),
            text(" to initialize the prop value. "),
            code("value"),
            text(
                " can be any expression that returns the field's type. For example, to default a \
                 boolean prop to "
            ),
            code("true"),
            text(", use the attribute "),
            code("#[prop_or(true)]"),
            text(
                ". The expression is evaluated when the properties are constructed and no \
                 explicit value has been given."
            ),
        ]),
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    // highlight-start
    #[prop_or("Bob".to_string())]
    // highlight-end
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// Then use like this with default
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"##
        ),
        h3(vec![text("#[prop_or_else(function)]")]),
        p(vec![
            text("Call "),
            code("function"),
            text(" to initialize the prop value. "),
            code("function"),
            text(" should have the signature "),
            code("FnMut() -> T"),
            text(" where "),
            code("T"),
            text(
                " is the field type. The function is called when no explicit value has been given \
                 for that attribute."
            ),
        ]),
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties};

fn create_default_name() -> String {
    "Bob".to_string()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    // highlight-start
    #[prop_or_else(create_default_name)]
    // highlight-end
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// Then use like this with default
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"##
        ),
        h2(vec![text("Memory/speed overhead of using Properties")]),
        p(vec![text(
            "Internally properties are reference counted. This means that only a shared pointer \
             is passed down the component tree for props. It saves us from the cost of having to \
             clone the entire props, which might be expensive."
        ),]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("Make use of "),
                code("AttrValue"),
                text(
                    " which is our custom type for attribute values instead of defining them as \
                     String or another similar type."
                ),
            ]),]
        ),
        h2(vec![text("Props macro")]),
        p(vec![
            text("The "),
            code("yew::props!"),
            text(" macro allows you to build properties the same way the "),
            code("html!"),
            text(" macro does it."),
        ]),
        p(vec![
            text(
                "The macro uses the same syntax as a struct expression except that you can't use \
                 attributes or a base expression ("
            ),
            code("Foo {{ ..base }}"),
            text("). The type path can either point to the props directly ("),
            code("path::to::Props"),
            text(") or the associated properties of a component ("),
            code("MyComp::Properties"),
            text(")."),
        ]),
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties, props, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Bob"))]
    pub name: AttrValue,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

#[function_component]
fn App() -> Html {
    // highlight-start
    let pre_made_props = props! {
        Props {} // Notice we did not need to specify name prop
    };
    // highlight-end
    html! {<HelloWorld ..pre_made_props />}
}"##
        ),
        h2(vec![text("Evaluation Order")]),
        p(vec![text(
            "Props are evaluated in the order they're specified, as shown by the following \
             example:"
        )]),
        code_block(
            "rust",
            r#"#[derive(yew::Properties, PartialEq)]
struct Props { first: usize, second: usize, last: usize }

fn main() {
    let mut g = 1..=3;
    let props = yew::props!(Props { first: g.next().unwrap(), second: g.next().unwrap(), last: g.next().unwrap() });

    assert_eq!(props.first, 1);
    assert_eq!(props.second, 2);
    assert_eq!(props.last, 3);
}"#
        ),
        h2(vec![text("Anti Patterns")]),
        p(vec![text(
            "While almost any Rust type can be passed as properties, there are some anti-patterns \
             that should be avoided. These include, but are not limited to:"
        ),]),
        ol(vec![
            li_blocks(vec![
                p(vec![
                    text("Using "),
                    code("String"),
                    text(" type instead of "),
                    code("AttrValue"),
                    text("."),
                ]),
                p(vec![
                    bold(vec![text("Why is this bad?")]),
                    text(" "),
                    code("String"),
                    text(
                        " can be expensive to clone. Cloning is often needed when the prop value \
                         is used with hooks and callbacks. "
                    ),
                    code("AttrValue"),
                    text(" is either a reference-counted string ("),
                    code("Rc<str>"),
                    text(") or a "),
                    code("&'static str"),
                    text(", thus very cheap to clone."),
                ]),
                p(vec![
                    bold(vec![text("Note")]),
                    text(": "),
                    code("AttrValue"),
                    text(" internally is "),
                    code("IString"),
                    text(" from "),
                    link(
                        "https://crates.io/crates/implicit-clone",
                        vec![text("implicit-clone")]
                    ),
                    text(". See that crate to learn more."),
                ]),
            ]),
            li_blocks(vec![
                p(vec![text("Using interior mutability.")]),
                p(vec![
                    bold(vec![text("Why is this bad?")]),
                    text(" Interior mutability (such as with "),
                    code("RefCell"),
                    text(", "),
                    code("Mutex"),
                    text(
                        ", etc.) should generally be avoided. It can cause problems with \
                         re-renders (Yew doesn't know when state has changed) so you may have to \
                         manually force a render. Like all things, it has its place. Use it with \
                         caution."
                    ),
                ]),
            ]),
            li_blocks(vec![p(vec![text(
                "You tell us. Did you run into an edge-case you wish you knew about earlier? Feel \
                 free to create an issue or PR a fix to this documentation."
            ),]),]),
        ]),
    ])
);
