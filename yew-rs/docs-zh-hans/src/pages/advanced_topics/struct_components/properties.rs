pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "属性 (Properties) \
             使子组件和父组件之间能够进行通信。每个组件都有一个关联的属性类型，\
             用于描述从父组件传递下来的内容。理论上，这可以是任何实现了 ",
            code("Properties"),
            " 特性的类型，但实际上，它应该是一个结构体，其中每个字段代表一个属性。",
        ],
        h2!["派生宏"],
        p![
            "无需自己实现 ",
            code("Properties"),
            " 特性，我们可以用 ",
            code("#[derive(Properties)]"),
            " 来自动生成实现。派生 ",
            code("Properties"),
            " 的类型也必须实现 ",
            code("PartialEq"),
            "。",
        ],
        h3!["字段属性"],
        p![
            "在派生 ",
            code("Properties"),
            " 时，默认情况下所有字段都是必需的。以下属性允许您为属性提供初始值，\
             除非它们被设置为另一个值。",
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p!["属性不会在 Rustdoc \
                生成的文档中显示。您的属性的文档字符串应该说明一个属性是否是可选的，\
                以及它是否有一个特殊的默认值。"],
        ],
        h4![code("#[prop_or_default]")],
        p![
            "使用字段类型的默认值使用 ",
            code("Default"),
            " 特性来初始化属性值。",
        ],
        h4![code("#[prop_or(value)]")],
        p![
            "使用 ",
            code("value"),
            " 来初始化属性值。",
            code("value"),
            " 可以是返回字段类型的任何表达式。例如，要将布尔属性默认为 ",
            code("true"),
            "，请使用属性 ",
            code("#[prop_or(true)]"),
            "。",
        ],
        h4![code("#[prop_or_else(function)]")],
        p![
            "调用 ",
            code("function"),
            " 来初始化属性值。",
            code("function"),
            " 应该具有签名 ",
            code("FnMut() -> T"),
            "，其中 ",
            code("T"),
            " 是字段类型。",
        ],
        h2![code("PartialEq")],
        p![
            code("Properties"),
            " 需要实现 ",
            code("PartialEq"),
            "。这样，Yew 才能比较它们，以便在它们发生变化时调用 ",
            code("changed"),
            " 方法。",
        ],
        h2!["使用 Properties 的性能开销"],
        p![
            "内部属性是基于引用计数的指针存储的。这意味着只有一个指针被传递到组件树中的属性，\
             以避免克隆整个属性所带来的昂贵性能开销。"
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "使用 ",
                code("AttrValue"),
                "，这是我们提供的自定义属性值类型，这样就可以不用 String \
                 或其他类似的需要克隆的类型。",
            ],
        ],
        h2!["示例"],
        code_block(
            "rust",
            r#"use yew::Properties;
/// 从 virtual_dom 中导入 AttrValue
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
    /// 链接必须有一个目标
    href: AttrValue,
    /// 还要注意我们使用的是 AttrValue 而不是 String
    text: AttrValue,
    /// 链接的颜色，默认为 `Blue`
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// 如果值为 None，则视图函数不会指定大小
    #[prop_or_default]
    size: Option<u32>,
    /// 当视图函数没有指定活动时，默认为 true
    #[prop_or(true)]
    active: bool,
}"#,
        ),
        h2!["Props 宏"],
        p![
            code("yew::props!"),
            " 宏允许您以与 ",
            code("html!"),
            " 宏相同的方式构建属性。",
        ],
        p![
            "该宏使用与结构体表达式相同的语法，只是您不能使用属性或基本表达式 (",
            code("Foo { ..base }"),
            ")。类型路径可以直接指向属性 (",
            code("path::to::Props"),
            ")，也可以指向组件的关联属性 (",
            code("MyComp::Properties"),
            ")。",
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
    /// 链接必须有一个目标
    href: AttrValue,
    /// 还要注意我们使用的是 AttrValue 而不是 String
    text: AttrValue,
    /// 链接的颜色，默认为 `Blue`
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// 如果值为 None，则视图函数不会指定大小
    #[prop_or_default]
    size: Option<u32>,
    /// 当视图函数没有指定活动时，默认为 true
    #[prop_or(true)]
    active: bool,
}

impl LinkProps {
    /// 注意此函数接收 href 和 text 作为 String
    /// 我们可以使用 `AttrValue::from` 将其转换为 `AttrValue`
    pub fn new_link_with_size(href: String, text: String, size: u32) -> Self {
        // highlight-start
        props! {LinkProps {
            href: AttrValue::from(href),
            text: AttrValue::from(text),
            size,
        }}
        // highlight-end
    }
}"#,
        ),
    ])
}

crate::doc_page!(
    "属性 (Props)",
    "/zh-Hans/docs/advanced-topics/struct-components/properties",
    page_content()
);
