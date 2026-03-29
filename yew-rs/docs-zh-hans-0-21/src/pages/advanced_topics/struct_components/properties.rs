crate::doc_page!(
    "属性（Properties）",
    "/zh-Hans/docs/advanced-topics/struct-components/properties",
    Content::new(vec![
        p![
            "Properties 用于父级到子组件的通信。 \
             每个组件都有一个关联的属性类型，描述了从父级传递下来的内容。 \
             理论上，这可以是任何实现了 ",
            code("Properties"),
            " trait 的类型，但实际上， 没有理由不使用一个结构体，其中每个字段都代表一个属性。"
        ],
        h2!["派生宏"],
        p![
            "不要尝试自己去实现 ",
            code("Properties"),
            "，而是通过使用 ",
            code("#[derive(Properties)]"),
            " 来派生它。 派生 ",
            code("Properties"),
            " 的类型还必须实现 ",
            code("PartialEq"),
            "。"
        ],
        h3!["Field attributes"],
        p![
            "When deriving ",
            code("Properties"),
            ", all fields are required by default. The following attributes allow you to give \
             your props initial values which will be used unless they are set to another value."
        ],
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p!["Attributes aren't visible in Rustdoc generated \
                     documentation.
The doc strings of your properties should mention whether a prop is optional and if it has a \
                     special default value."]]
        ),
        h4!["`#[prop_or_default]`"],
        p![
            "Initialize the prop value with the default value of the field's type using the ",
            code("Default"),
            " trait."
        ],
        h4!["`#[prop_or(value)]`"],
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
            "."
        ],
        h4!["`#[prop_or_else(function)]`"],
        p![
            "Call ",
            code("function"),
            " to initialize the prop value. ",
            code("function"),
            " should have the signature ",
            code("FnMut() -> T"),
            " where ",
            code("T"),
            " is the field type."
        ],
        h2!["`PartialEq`"],
        p![
            code("Properties"),
            " require ",
            code("PartialEq"),
            " to be implemented. This is so that they can be compared by Yew to call the ",
            code("changed"),
            " method only when they change."
        ],
        h2!["Memory/speed overhead of using Properties"],
        p![
            "Internally properties are reference counted. This means that only a pointer is \
             passed down the component tree for props. It saves us from the cost of having to \
             clone the entire props, which might be expensive."
        ],
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p![
                "Make use of ",
                code("AttrValue"),
                " which is our custom type for attribute values instead of defining them as \
                 String or another similar type."
            ]]
        ),
        h2!["Example"],
        code_block(
            "rust",
            r#"use yew::Properties;
/// Importing the AttrValue from virtual_dom
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
    /// The link must have a target.
    href: AttrValue,
    /// Also notice that we are using AttrValue instead of String
    text: AttrValue,
    /// Color of the link. Defaults to `Blue`.
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    #[prop_or_default]
    size: Option<u32>,
    /// When the view function does not specify active, it defaults to true.
    #[prop_or(true)]
    active: bool,
}"#
        ),
        h2!["Props macro"],
        p![
            "The ",
            code("yew::props!"),
            " macro allows you to build properties the same way the ",
            code("html!"),
            " macro does it."
        ],
        p![
            "The macro uses the same syntax as a struct expression except that you cannot use \
             attributes or a base expression (",
            code("Foo {{ ..base }}"),
            "). The type path can either point to the props directly (",
            code("path::to::Props"),
            ") or the associated properties of a component (",
            code("MyComp::Properties"),
            ")."
        ],
        code_block(
            "rust",
            r#"use yew::{props, Properties, virtual_dom::AttrValue};

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
    /// The link must have a target.
    href: AttrValue,
    /// Also notice that we're using AttrValue instead of String
    text: AttrValue,
    /// Color of the link. Defaults to `Blue`.
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    #[prop_or_default]
    size: Option<u32>,
    /// When the view function doesn't specify active, it defaults to true.
    #[prop_or(true)]
    active: bool,
}

impl LinkProps {
    /// Notice that this function receives href and text as String
    /// We can use `AttrValue::from` to convert it to a `AttrValue`
    pub fn new_link_with_size(href: String, text: String, size: u32) -> Self {
        // highlight-start
        props! {LinkProps {
            href: AttrValue::from(href),
            text: AttrValue::from(text),
            size,
        }}
        // highlight-end
    }
}"#
        )
    ])
    .with_description("Parent to child communication")
);
