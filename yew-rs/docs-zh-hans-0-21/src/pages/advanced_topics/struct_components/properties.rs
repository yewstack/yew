crate::doc_page!(
    "属性（Properties）",
    "/zh-Hans/docs/advanced-topics/struct-components/properties",
    Content::new(vec![
        p![
            text(
                "Properties 用于父级到子组件的通信。 \
                 每个组件都有一个关联的属性类型，描述了从父级传递下来的内容。 \
                 理论上，这可以是任何实现了 "
            ),
            code("Properties"),
            text(
                " trait 的类型，但实际上， 没有理由不使用一个结构体，其中每个字段都代表一个属性。"
            )
        ],
        h2![text("派生宏")],
        p![
            text("不要尝试自己去实现 "),
            code("Properties"),
            text("，而是通过使用 "),
            code("#[derive(Properties)]"),
            text(" 来派生它。 派生 "),
            code("Properties"),
            text(" 的类型还必须实现 "),
            code("PartialEq"),
            text("。")
        ],
        h3![text("Field attributes")],
        p![
            text("When deriving "),
            code("Properties"),
            text(
                ", all fields are required by default. The following attributes allow you to give \
                 your props initial values which will be used unless they are set to another \
                 value."
            )
        ],
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p![text(
                "Attributes aren't visible in Rustdoc generated documentation.
The doc strings of your properties should mention whether a prop is optional and if it has a \
                 special default value."
            )]]
        ),
        h4![text("`#[prop_or_default]`")],
        p![
            text("Initialize the prop value with the default value of the field's type using the "),
            code("Default"),
            text(" trait.")
        ],
        h4![text("`#[prop_or(value)]`")],
        p![
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
            text(".")
        ],
        h4![text("`#[prop_or_else(function)]`")],
        p![
            text("Call "),
            code("function"),
            text(" to initialize the prop value. "),
            code("function"),
            text(" should have the signature "),
            code("FnMut() -> T"),
            text(" where "),
            code("T"),
            text(" is the field type.")
        ],
        h2![text("`PartialEq`")],
        p![
            code("Properties"),
            text(" require "),
            code("PartialEq"),
            text(" to be implemented. This is so that they can be compared by Yew to call the "),
            code("changed"),
            text(" method only when they change.")
        ],
        h2![text("Memory/speed overhead of using Properties")],
        p![text(
            "Internally properties are reference counted. This means that only a pointer is \
             passed down the component tree for props. It saves us from the cost of having to \
             clone the entire props, which might be expensive."
        )],
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p![
                text("Make use of "),
                code("AttrValue"),
                text(
                    " which is our custom type for attribute values instead of defining them as \
                     String or another similar type."
                )
            ]]
        ),
        h2![text("Example")],
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
        h2![text("Props macro")],
        p![
            text("The "),
            code("yew::props!"),
            text(" macro allows you to build properties the same way the "),
            code("html!"),
            text(" macro does it.")
        ],
        p![
            text(
                "The macro uses the same syntax as a struct expression except that you cannot use \
                 attributes or a base expression ("
            ),
            code("Foo {{ ..base }}"),
            text("). The type path can either point to the props directly ("),
            code("path::to::Props"),
            text(") or the associated properties of a component ("),
            code("MyComp::Properties"),
            text(").")
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
);
