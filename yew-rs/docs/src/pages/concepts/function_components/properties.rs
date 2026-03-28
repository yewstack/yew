pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(
            AdmonitionType::Note,
            None,
            p!["Properties are often shortened as \"Props\"."],
        ),
        p!["Properties are essentially component arguments that Yew can keep watch on."],
        p![
            "A type has to implement the ",
            code("Properties"),
            " trait before it can be used as the properties of a component.",
        ],
        h2!["Reactivity"],
        p![
            "Yew checks if props have changed when reconciling the Virtual DOM during \
             re-rendering, to know if nested components need to be re-rendered. This way Yew can \
             be considered a very reactive framework, as changes from the parent will always be \
             propagated downward, and the view will never be out of sync with the data coming \
             from props/state."
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "If you have not yet completed the ",
                doc_link!(@ "/tutorial", "tutorial"),
                ", try it out and test this reactivity yourself!",
            ],
        ),
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
}"#,
        ),
        h2!["Use in function components"],
        p![
            "The attribute ",
            code("#[component]"),
            " allows to optionally receive Props in the function arguments. To supply them, they \
             are assigned via attributes in the ",
            code("html!"),
            " macro.",
        ],
        tabs(
            "with-props",
            vec![
                tab(
                    "with-props",
                    "With Props",
                    vec![code_block(
                        "rust",
                        r#"use yew::{component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[component]
fn HelloWorld(&Props { is_loading }: &Props) -> Html {
    html! { <>{"Am I loading? - "}{is_loading}</> }
}

// Then supply the prop
#[component]
fn App() -> Html {
    html! { <HelloWorld is_loading=true /> }
}"#,
                    )],
                ),
                tab(
                    "no-props",
                    "No Props",
                    vec![code_block(
                        "rust",
                        r#"use yew::{component, html, Html};

#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// No props to supply
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
                    )],
                ),
            ],
        ),
        h2!["Derive macro field attributes"],
        p![
            "When deriving ",
            code("Properties"),
            " all fields are required by default. The following attributes allow you to give your \
             props default values which will be used when the parent has not set them.",
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "Attributes aren't visible in Rustdoc generated documentation. The doc strings of \
                 your properties should mention whether a prop is optional and if it has a \
                 special default value."
            ],
        ),
        tabs(
            "prop_or_default",
            vec![
                tab(
                    "prop_or_default",
                    "#[prop_or_default]",
                    vec![
                        p![
                            "Initialize the prop value with the default value of the field's type \
                             using the ",
                            code("Default"),
                            " trait.",
                        ],
                        code_block(
                            "rust",
                            r#"use yew::{component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[component]
fn HelloWorld(&Props { is_loading }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}

// Then use like this with default
#[component]
fn Case1() -> Html {
    html! { <HelloWorld /> }
}
// Or no override the default
#[component]
fn Case2() -> Html {
    html! { <HelloWorld is_loading=true /> }
}"#,
                        ),
                    ],
                ),
                tab(
                    "prop_or_value",
                    "#[prop_or(value)]",
                    vec![
                        p![
                            "Use ",
                            code("value"),
                            " to initialize the prop value. ",
                            code("value"),
                            " can be any expression that returns the field's type. For example, \
                             to default a boolean prop to ",
                            code("true"),
                            ", use the attribute ",
                            code("#[prop_or(true)]"),
                            ". The expression is evaluated when the properties are constructed \
                             and no explicit value has been given.",
                        ],
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::Static("Bob"))]
    pub name: AttrValue,
}

#[component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name} </>}
    }
}

// Then use like this with default
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// Or no override the default
#[component]
fn Case2() -> Html {
    html! { <Hello name="Sam" /> }
}"#,
                        ),
                    ],
                ),
                tab(
                    "prop_or_else_function",
                    "#[prop_or_else(function)]",
                    vec![
                        p![
                            "Call ",
                            code("function"),
                            " to initialize the prop value. ",
                            code("function"),
                            " should have the signature ",
                            code("FnMut() -> T"),
                            " where ",
                            code("T"),
                            " is the field type. The function is called when no explicit value \
                             has been given for that attribute.",
                        ],
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

fn create_default_name() -> AttrValue {
    AttrValue::Static("Bob")
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or_else(create_default_name)]
    pub name: AttrValue,
}

#[component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name}</> }
    }
}

// Then use like this with default
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// Or no override the default
#[component]
fn Case2() -> Html {
    html! { <Hello name="Sam" /> }
}"#,
                        ),
                    ],
                ),
            ],
        ),
        h2!["Memory/speed overhead of using Properties"],
        p![
            "Internally properties are reference counted. This means that only a shared pointer \
             is passed down the component tree for props. It saves us from the cost of having to \
             clone the entire props, which might be expensive."
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "Make use of ",
                code("AttrValue"),
                " which is our custom type for attribute values instead of defining them as \
                 String or another similar type.",
            ],
        ),
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
            code("Foo { ..base }"),
            "). The type path can either point to the props directly (",
            code("path::to::Props"),
            ") or the associated properties of a component (",
            code("MyComp::Properties"),
            ").",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::Static("Bob"))]
    pub name: AttrValue,
}

#[component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name}</> }
    }
}

#[component]
fn App() -> Html {
    // highlight-start
    let pre_made_props = yew::props! {
        Props {} // Notice we did not need to specify name prop
    };
    // highlight-end
    html! { <Hello ..pre_made_props /> }
}"#,
        ),
        h2!["Automatically generate properties (yew-autoprops)"],
        p![
            "In order to streamline your development process, you can also use the macro ",
            code("#[autoprops]"),
            " (from the crate ",
            code("yew-autoprops"),
            ") that will automatically generate the ",
            code("Properties"),
            " struct for you.",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew_autoprops::autoprops;

// the #[autoprops] macro must appear BEFORE #[component], the order matters
#[autoprops]
#[component]
fn Greetings(
    #[prop_or_default]
    is_loading: bool,
    #[prop_or(AttrValue::Static("Hello"))]
    message: &AttrValue,
    #[prop_or(AttrValue::Static("World"))]
    name: &AttrValue,
) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{message}{" "}{name}</> }
    }
}

// The properties struct "GreetingsProps" will be generated automatically.
//
// `is_loading` will be passed as value to the components while `message` and
// `name` will use references because of the leading `&` in the definition."#,
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

let mut g = 1..=3;
let props = yew::props!(Props { first: g.next().unwrap(), second: g.next().unwrap(), last: g.next().unwrap() });

assert_eq!(props.first, 1);
assert_eq!(props.second, 2);
assert_eq!(props.last, 3);"#,
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
                    link!("https://crates.io/crates/implicit-clone", "implicit-clone",),
                    "\nSee that crate to learn more.",
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
                    ", etc.) should ",
                    italic!["generally"],
                    " be avoided. It can cause problems with re-renders (Yew doesn't know when \
                     the state has changed) so you may have to manually force a render. Like all \
                     things, it has its place. Use it with caution.",
                ],
            ],
            li_blocks![
                p![
                    "Using ",
                    code("Vec<T>"),
                    " type instead of ",
                    code("IArray<T>"),
                    ".",
                ],
                p![
                    bold!["Why is this bad?"],
                    " ",
                    code("Vec<T>"),
                    ", just like ",
                    code("String"),
                    ", can also be expensive to clone. ",
                    code("IArray<T>"),
                    " is either a reference-counted slice (",
                    code("Rc<[T]>"),
                    ") or a ",
                    code("&'static [T]"),
                    ", thus very cheap to clone.",
                ],
                p![
                    bold!["Note"],
                    ": ",
                    code("IArray"),
                    " can be imported from ",
                    link!("https://crates.io/crates/implicit-clone", "implicit-clone",),
                    "\nSee that crate to learn more.",
                ],
            ],
            li_blocks![p!["You tell us. Did you run into an edge-case you wish \
                           you knew about earlier? Feel free to create an issue \
                           or PR a fix to this documentation."]],
        ],
        h2!["yew-autoprops"],
        p![
            link!("https://crates.io/crates/yew-autoprops", "yew-autoprops",),
            " is an experimental package that allows one to create the Props struct on the fly \
             out of the arguments of your function. Might be useful, if the properties struct is \
             never reused.",
        ],
    ])
    .with_description("Parent to child communication")
}

crate::doc_page!(
    "Properties",
    "/docs/concepts/function-components/properties",
    page_content()
);
