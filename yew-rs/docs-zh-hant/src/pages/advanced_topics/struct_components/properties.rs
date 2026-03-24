pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text(
                "屬性 (Properties) \
                 使子組件和父組件之間能夠進行通訊。每個元件都有一個關聯的屬性類型，\
                 用來描述從父元件傳遞下來的內容。理論上，這可以是任何實現了 ",
            ),
            code("Properties"),
            text(" 特性的類型，但實際上，它應該是一個結構體，其中每個字段代表一個屬性。"),
        ],
        h2![text("派生宏")],
        p![
            text("無需自己實作 "),
            code("Properties"),
            text(" 特性，我們可以用 "),
            code("#[derive(Properties)]"),
            text(" 來自動生成實作。派生 "),
            code("Properties"),
            text(" 的型別也必須實作 "),
            code("PartialEq"),
            text("。"),
        ],
        h3![text("欄位屬性")],
        p![
            text("在派生 "),
            code("Properties"),
            text(
                " 時，預設情況下所有欄位都是必要的。以下屬性可讓您為屬性提供初始值，\
                 除非它們被設定為另一個值。",
            ),
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![text(
                "屬性不會在 Rustdoc \
                 產生的文件中顯示。您的屬性的文檔字串應該說明一個屬性是否是可選的，\
                 以及它是否有一個特殊的預設值。",
            )],
        ),
        h4![code("#[prop_or_default]")],
        p![
            text("使用欄位類型的預設值使用 "),
            code("Default"),
            text(" 特性來初始化屬性值。"),
        ],
        h4![code("#[prop_or(value)]")],
        p![
            text("使用 "),
            code("value"),
            text(" 來初始化屬性值。 "),
            code("value"),
            text(" 可以是傳回欄位類型的任何表達式。例如，要將布林屬性預設為 "),
            code("true"),
            text("，請使用屬性 "),
            code("#[prop_or(true)]"),
            text("。"),
        ],
        h4![code("#[prop_or_else(function)]")],
        p![
            text("呼叫 "),
            code("function"),
            text(" 來初始化屬性值。 "),
            code("function"),
            text(" 應該有簽章 "),
            code("FnMut() -> T"),
            text("，其中 "),
            code("T"),
            text(" 是欄位類型。"),
        ],
        h2![code("PartialEq")],
        p![
            code("Properties"),
            text(" 需要實作 "),
            code("PartialEq"),
            text("。這樣，Yew 才能比較它們，以便在它們發生變化時呼叫 "),
            code("changed"),
            text(" 方法。"),
        ],
        h2![text("使用 Properties 的效能開銷")],
        p![text(
            "內部屬性是基於引用計數的指標儲存的。這意味著只有一個指標被傳遞到元件樹中的屬性，\
             以避免複製整個屬性所帶來的昂貴效能開銷。",
        )],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                text("使用 "),
                code("AttrValue"),
                text(
                    "，這是我們提供的自訂屬性值類型，這樣就可以不用 String \
                     或其他類似的需要克隆的類型。",
                ),
            ],
        ),
        h2![text("範例")],
        code_block(
            "rust",
            r#"use yew::Properties;
/// 從 virtual_dom 中導入 AttrValue
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
    /// 鏈接必須有一個目標
    href: AttrValue,
    /// 還要注意我們使用的是 AttrValue 而不是 String
    text: AttrValue,
    /// 鏈接的顏色，默認為 `Blue`
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// 如果值為 None，則視圖函數不會指定大小
    #[prop_or_default]
    size: Option<u32>,
    /// 當視圖函數沒有指定活動時，默認為 true
    #[prop_or(true)]
    active: bool,
}"#,
        ),
        h2![text("Props 巨集")],
        p![
            code("yew::props!"),
            text(" 巨集允許您以與 "),
            code("html!"),
            text(" 巨集相同的方式建立屬性。"),
        ],
        p![
            text("這個巨集使用與結構體表達式相同的語法，只是您不能使用屬性或基本表達式 ("),
            code("Foo { ..base }"),
            text(")。類型路徑可以直接指向屬性 ("),
            code("path::to::Props"),
            text(")，也可以指向元件的關聯屬性 ("),
            code("MyComp::Properties"),
            text(")。"),
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
    /// 鏈接必須有一個目標
    href: AttrValue,
    /// 還要注意我們使用的是 AttrValue 而不是 String
    text: AttrValue,
    /// 鏈接的顏色，默認為 `Blue`
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// 如果值為 None，則視圖函數不會指定大小
    #[prop_or_default]
    size: Option<u32>,
    /// 當視圖函數沒有指定活動時，默認為 true
    #[prop_or(true)]
    active: bool,
}

impl LinkProps {
    /// 注意此函數接收 href 和 text 作為 String
    /// 我們可以使用 `AttrValue::from` 將其轉換為 `AttrValue`
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
    "屬性 (Props)",
    "/zh-Hant/docs/advanced-topics/struct-components/properties",
    page_content()
);
