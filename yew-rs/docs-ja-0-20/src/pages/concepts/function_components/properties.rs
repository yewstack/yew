crate::doc_page!(
    "Properties",
    "/ja/docs/concepts/function-components/properties",
    Content::new(vec![
        admonition![
            AdmonitionType::Note,
            None,
            p!["Properties are often shortened as \"Props\"."]
        ],
        p!["Properties are essentially component arguments that Yew can keep watch on."],
        p![
            "A type has to implement the ",
            code("Properties"),
            " trait before it can be used as the properties of a component.",
        ],
        h2!["Reactivity"],
        p![
            "Yew checks if props have changed when reconciling the vdom during rerendering, to \
             know if nested components needs to be rerendered. This way Yew can be considered a \
             very reactive framework as changes from the parent will always be propagated \
             downwards and the view will never be out of sync from the data coming from \
             props/state."
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "If you have not yet completed the ",
                doc_link!(@ "/tutorial", "tutorial"),
                ", try it out and test this reactivity yourself!",
            ]
        ],
        h2!["Derive macro"],
        p![
            "Yew provides a derive macro to easily implement the ",
            code("Properties"),
            " trait on structs.",
        ],
        p![
            "Types for which you derive ",
            code("Properties"),
            " must also implement ",
            code("PartialEq"),
            " so Yew can do data comparison.",
        ],
        code_block(
            "rust",
            r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#
        ),
        h2!["Use in function components"],
        p![
            "The attribute ",
            code("#[function_component]"),
            " allows to optionally receive Props in the function arguments. To supply them, they \
             are assigned via attributes in the ",
            code("html!"),
            " macro.",
        ],
        h3!["With Props"],
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component(HelloWorld)]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// Then supply the prop
#[function_component(App)]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"##
        ),
        h3!["No Props"],
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html};





#[function_component(HelloWorld)]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// No props to supply
#[function_component(App)]
fn App() -> Html {
    html! {<HelloWorld />}
}
"#
        ),
        h2!["Derive macro field attributes"],
        p![
            "When deriving ",
            code("Properties"),
            " all fields are required by default. The following attributes allow you to give your \
             props default values which will be used when parent has not set them.",
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "Attributes aren't visible in Rustdoc generated documentation. The doc strings of \
                 your properties should mention whether a prop is optional and if it has a \
                 special default value."
            ]
        ],
        h3!["#[prop_or_default]"],
        p![
            "Initialize the prop value with the default value of the field's type using the ",
            code("Default"),
            " trait.",
        ],
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

#[function_component(HelloWorld)]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading.clone() {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}

// Then use like this with default
#[function_component(Case1)]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component(Case2)]
fn Case2() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"##
        ),
        h3!["#[prop_or(value)]"],
        p![
            "Use ",
            code("value"),
            " to initialize the prop value. ",
            code("value"),
            " can be any expression that returns the field's type. For example, to default a \
             boolean prop to ",
            code("true"),
            ", use the attribute ",
            code("#[prop_or(true)]"),
            ". The expression is evaluated when the properties are constructed and no explicit \
             value has been given.",
        ],
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

#[function_component(HelloWorld)]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// Then use like this with default
#[function_component(Case1)]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component(Case2)]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"##
        ),
        h3!["#[prop_or_else(function)]"],
        p![
            "Call ",
            code("function"),
            " to initialize the prop value. ",
            code("function"),
            " should have the signature ",
            code("FnMut() -> T"),
            " where ",
            code("T"),
            " is the field type. The function is called when no explicit value has been given for \
             that attribute.",
        ],
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

#[function_component(HelloWorld)]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// Then use like this with default
#[function_component(Case1)]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component(Case2)]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"##
        ),
        h2!["Memory/speed overhead of using Properties"],
        p![
            "Internally properties are reference counted. This means that only a shared pointer \
             is passed down the component tree for props. It saves us from the cost of having to \
             clone the entire props, which might be expensive."
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "Make use of ",
                code("AttrValue"),
                " which is our custom type for attribute values instead of defining them as \
                 String or another similar type.",
            ]
        ],
        h2!["Props macro"],
        p![
            "The ",
            code("yew::props!"),
            " macro allows you to build properties the same way the ",
            code("html!"),
            " macro does it.",
        ],
        p![
            "The macro uses the same syntax as a struct expression except that you can't use \
             attributes or a base expression (",
            code("Foo {{ ..base }}"),
            "). The type path can either point to the props directly (",
            code("path::to::Props"),
            ") or the associated properties of a component (",
            code("MyComp::Properties"),
            ").",
        ],
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties, props, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Bob"))]
    pub name: AttrValue,
}

#[function_component(HelloWorld)]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

#[function_component(App)]
fn App() -> Html {
    // highlight-start
    let pre_made_props = props! {
        Props {} // Notice we did not need to specify name prop
    };
    // highlight-end
    html! {<HelloWorld ..pre_made_props />}
}"##
        ),
        h2!["Evaluation Order"],
        p![
            "Props are evaluated in the order they're specified, as shown by the following \
             example:"
        ],
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
        h2!["Anti Patterns"],
        p![
            "While almost any Rust type can be passed as properties, there are some anti-patterns \
             that should be avoided. These include, but are not limited to:"
        ],
        ol![
            li_blocks![
                p![
                    "Using ",
                    code("String"),
                    " type instead of ",
                    code("AttrValue"),
                    ".",
                ],
                p![
                    bold!["Why is this bad?"],
                    " ",
                    code("String"),
                    " can be expensive to clone. Cloning is often needed when the prop value is \
                     used with hooks and callbacks. ",
                    code("AttrValue"),
                    " is either a reference-counted string (",
                    code("Rc<str>"),
                    ") or a ",
                    code("&'static str"),
                    ", thus very cheap to clone.",
                ],
                p![
                    bold!["Note"],
                    ": ",
                    code("AttrValue"),
                    " internally is ",
                    code("IString"),
                    " from ",
                    link!["https://crates.io/crates/implicit-clone", "implicit-clone"],
                    ". See that crate to learn more.",
                ],
            ],
            li_blocks![
                p!["Using interior mutability."],
                p![
                    bold!["Why is this bad?"],
                    " Interior mutability (such as with ",
                    code("RefCell"),
                    ", ",
                    code("Mutex"),
                    ", etc.) should generally be avoided. It can cause problems with re-renders \
                     (Yew doesn't know when state has changed) so you may have to manually force \
                     a render. Like all things, it has its place. Use it with caution.",
                ],
            ],
            li_blocks![p!["You tell us. Did you run into an edge-case you wish \
                           you knew about earlier? Feel free to create an issue \
                           or PR a fix to this documentation."]],
        ],
    ])
);
