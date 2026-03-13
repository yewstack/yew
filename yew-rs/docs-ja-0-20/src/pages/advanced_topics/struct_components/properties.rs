crate::doc_page!(
    "Properties",
    "/ja/docs/advanced-topics/struct-components/properties",
    Content::new(vec![
        p(vec![
            text(
                "Properties enable child and parent components to communicate with each other. \
                 Every component has an associated properties type which describes what is passed \
                 down from the parent. In theory, this can be any type that implements the "
            ),
            code("Properties"),
            text(
                " trait, but in practice, there is no reason for it to be anything but a struct \
                 where each field represents a property."
            ),
        ]),
        h2(vec![text("Derive macro")]),
        p(vec![
            text("Instead of implementing the "),
            code("Properties"),
            text(" trait yourself, you should use "),
            code("#[derive(Properties)]"),
            text(
                " to automatically generate the implementation instead. Types for which you \
                 derive "
            ),
            code("Properties"),
            text(" must also implement "),
            code("PartialEq"),
            text("."),
        ]),
        h3(vec![text("Field attributes")]),
        p(vec![
            text("When deriving "),
            code("Properties"),
            text(
                ", all fields are required by default. The following attributes allow you to give \
                 your props initial values which will be used unless they are set to another \
                 value."
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
        h4(vec![text("#[prop_or_default]")]),
        p(vec![
            text("Initialize the prop value with the default value of the field's type using the "),
            code("Default"),
            text(" trait."),
        ]),
        h4(vec![text("#[prop_or(value)]")]),
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
            text("."),
        ]),
        h4(vec![text("#[prop_or_else(function)]")]),
        p(vec![
            text("Call "),
            code("function"),
            text(" to initialize the prop value. "),
            code("function"),
            text(" should have the signature "),
            code("FnMut() -> T"),
            text(" where "),
            code("T"),
            text(" is the field type."),
        ]),
        h2(vec![text("PartialEq")]),
        p(vec![
            code("Properties"),
            text(" require "),
            code("PartialEq"),
            text(" to be implemented. This is so that they can be compared by Yew to call the "),
            code("changed"),
            text(" method only when they change."),
        ]),
        h2(vec![text("Memory/speed overhead of using Properties")]),
        p(vec![text(
            "Internally properties are reference counted. This means that only a pointer is \
             passed down the component tree for props. It saves us from the cost of having to \
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
        h2(vec![text("Example")]),
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
                "The macro uses the same syntax as a struct expression except that you cannot use \
                 attributes or a base expression ("
            ),
            code("Foo { ..base }"),
            text("). The type path can either point to the props directly ("),
            code("path::to::Props"),
            text(") or the associated properties of a component ("),
            code("MyComp::Properties"),
            text(")."),
        ]),
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
);
