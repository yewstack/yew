crate::doc_page!(
    "",
    "/zh-Hans/docs/advanced-topics/struct-components/properties",
    Content::new(vec![
        h1!["属性（Properties）"],
        p!["如\"组件（Components）\"页面所述，Properties 用于父级到子组件的通信。"],
        h2!["派生宏"],
        p![
            "不要尝试自己去实现 ",
            code("Properties"),
            "，而是通过使用 ",
            code("#[derive(Properties)]"),
            " 来派生它。",
        ],
        h3!["必需属性"],
        p![
            "默认情况下，实现了 ",
            code("Properties"),
            " 的结构体中的字段是必需的。当缺少了该字段并且在 ",
            code("html!"),
            " 宏中创建了组件时，将返回编译错误。对于具有可选属性的字段，使用 ",
            code("#[prop_or_default]"),
            " 来使用该类型的默认值。要指定一个值，请使用 ",
            code("#[prop_or_else(value)]"),
            "，其中 value 是该属性的默认值。例如，要将一个布尔值的默认值设置为 ",
            code("true"),
            "，请使用属性 ",
            code("#[prop_or_else(true)]"),
            "。可选属性通常使用 ",
            code("Option"),
            "，其默认值为 ",
            code("None"),
            "。",
        ],
        h3!["PartialEq"],
        p![
            "如果可以的话，在你的 props 上派生 ",
            code("PartialEq"),
            " 通常是很有意义的。这使用了一个",
            bold!["性能优化与最佳实践"],
            "部分解释了的技巧，可以更轻松地避免重新渲染。",
        ],
        h2!["Properties 的内存/速度开销"],
        p!["记住组件的 ", code("view"), " 函数签名："],
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
}"#
        ),
        p![
            "你对组件的状态取了一个引用，并用来创建 ",
            code("Html"),
            "。但是 properties 是有所有权的值（owned \
             values）。这意味着为了创造它们并且将它们传递给子组件，我们需要获取 ",
            code("view"),
            " 函数里提供的引用的所有权。这是在将引用传递给组件时隐式克隆引用完成的，以获得构成其 \
             props 的有所有权的值。",
        ],
        p![
            "这意味着每个组件都有从其父级传递来的状态的独特副本，而且，每当你重新渲染一个组件时，\
             该重新渲染组件的所有子组件的 props 都将被克隆。"
        ],
        p![
            "这意味着如果你将 _大量_ 数据作为 props（大小为 10 KB \
             的字符串）向下传递，则可能需要考虑将子组件转换为在父级运行返回 ",
            code("Html"),
            " 的函数，因为这样就不会被强制克隆你的数据。",
        ],
        p![
            "另外，如果你不需要修改作为 props 传递的大数据，而只需要显示它，则可以将其包装在 ",
            code("Rc"),
            " 中，以便仅克隆一个引用计数的指针，而不是数据本身。",
        ],
        h2!["示例"],
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
        ),
    ])
);
