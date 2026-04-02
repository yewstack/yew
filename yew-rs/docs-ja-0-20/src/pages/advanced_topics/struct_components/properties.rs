crate::doc_page!(
    "Properties",
    "/ja/docs/advanced-topics/struct-components/properties",
    Content::new(vec![
        p![
            "Properties enable child and parent components to communicate with each other. Every \
             component has an associated properties type which describes what is passed down from \
             the parent. In theory, this can be any type that implements the ",
            code("Properties"),
            " trait, but in practice, there is no reason for it to be anything but a struct where \
             each field represents a property.",
        ],
        h2!["Derive macro"],
        p![
            "Instead of implementing the ",
            code("Properties"),
            " trait yourself, you should use ",
            code("#[derive(Properties)]"),
            " to automatically generate the implementation instead. Types for which you derive ",
            code("Properties"),
            " must also implement ",
            code("PartialEq"),
            ".",
        ],
        h3!["Field attributes"],
        p![
            "When deriving ",
            code("Properties"),
            ", all fields are required by default. The following attributes allow you to give \
             your props initial values which will be used unless they are set to another value.",
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "Attributes aren't visible in Rustdoc generated documentation. The doc strings of \
                 your properties should mention whether a prop is optional and if it has a \
                 special default value.",
            ],
        ],
        h4!["#[prop_or_default]"],
        p![
            "Initialize the prop value with the default value of the field's type using the ",
            code("Default"),
            " trait.",
        ],
        h4!["#[prop_or(value)]"],
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
            ".",
        ],
        h4!["#[prop_or_else(function)]"],
        p![
            "Call ",
            code("function"),
            " to initialize the prop value. ",
            code("function"),
            " should have the signature ",
            code("FnMut() -> T"),
            " where ",
            code("T"),
            " is the field type.",
        ],
        h2!["PartialEq"],
        p![
            code("Properties"),
            " require ",
            code("PartialEq"),
            " to be implemented. This is so that they can be compared by Yew to call the ",
            code("changed"),
            " method only when they change.",
        ],
        h2!["Memory/speed overhead of using Properties"],
        p![
            "Internally properties are reference counted. This means that only a pointer is \
             passed down the component tree for props. It saves us from the cost of having to \
             clone the entire props, which might be expensive.",
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "Make use of ",
                code("AttrValue"),
                " which is our custom type for attribute values instead of defining them as \
                 String or another similar type.",
            ],
        ],
        h2!["Example"],
        code_block(
            "rust",
            r##"use yew::Properties;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

fn create_default_link_color() -> LinkColor {
    LinkColor::Blue
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    href: AttrValue,
    text: AttrValue,
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    #[prop_or_default]
    size: Option<u32>,
    #[prop_or(true)]
    active: bool,
}"##
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
            "The macro uses the same syntax as a struct expression except that you cannot use \
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
            r##"use yew::{props, Properties, virtual_dom::AttrValue};

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

fn create_default_link_color() -> LinkColor {
    LinkColor::Blue
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    href: AttrValue,
    text: AttrValue,
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    #[prop_or_default]
    size: Option<u32>,
    #[prop_or(true)]
    active: bool,
}

impl LinkProps {
    pub fn new_link_with_size(href: String, text: String, size: u32) -> Self {
        // highlight-start
        props! {LinkProps {
            href: AttrValue::from(href),
            text: AttrValue::from(text),
            size,
        }}
        // highlight-end
    }
}"##
        ),
    ])
    .with_description("Parent to child communication")
);
