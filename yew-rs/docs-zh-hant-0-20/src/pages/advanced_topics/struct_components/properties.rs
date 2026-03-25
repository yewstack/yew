crate::doc_page!(
    "Properties",
    "/zh-Hant/docs/advanced-topics/struct-components/properties",
    Content::new(vec![
        p![text("屬性讓子元件與父元件可以互相溝通。")],
        h2![text("Derive macro")],
        p![
            text("不要嘗試自己實作 "),
            code("Properties"),
            text("，而是用"),
            code("#[derive(Properties)]"),
            text("derive 他。"),
        ],
        h3![text("必填的欄位")],
        p![
            text("預設所有在 "),
            code("Properties"),
            text(" struct 裡的欄位都是必填的。當必填的欄位沒有值，而元件在 "),
            code("html!"),
            text(" 巨集中又被建立，編譯器就會報錯。如果希望欄位是可選的，可以使用 "),
            code("#[prop_or_default]"),
            text(" 來讓該欄位有預設值。如果希望欄位預設特定值，可以使用 "),
            code("#[prop_or_else(value)]"),
            text(" ，裡面的 value 就會是這個欄位的預設值。舉例來說，希望預設值是 "),
            code("true"),
            text("可以在欄位宣告上面這樣寫 "),
            code("#[prop_or_else(true)]"),
            text(". 通常可選的屬性，會用 "),
            code("Option"),
            text(" ，且預設值為"),
            code("None"),
            text("。"),
        ],
        h3![text("PartialEq")],
        p![
            text("如果可以，最好在你的屬性上面 derive "),
            code("PartialEq"),
            text(" 。他可以避免畫面多餘的渲染，更細節的內容請參考，"),
            bold![text("優化與最佳實例")],
            text("的區塊。"),
        ],
        h2![text("屬性的記憶體與速度的開銷")],
        p![
            text("在 "),
            code("Component::view"),
            text(",裡，你可以拿到元件狀態的參考，且用他來建立 "),
            code("Html"),
            text(
                " 。但是屬性是有所有權的。這代表著為了建立屬性，並且將他們傳遞給子元件，\
                 我們必須取得被 ",
            ),
            code("view"),
            text(" 方法拿走的所有權。 當將參考傳給元件時，可以透過隱式的複製來做到得到所有權。",),
        ],
        p![text(
            "這意味著，每個元件，都有從父元件傳遞下來的獨有的狀態複本，且每當你重新渲染一次元件，\
             被重新渲染的元件的所有的子元件的屬性就會被重新複製一次。",
        )],
        p![
            text(
                "代表如果你要在屬性中傳遞*大量*的資料（大於 10 KB \
                 的字串之類的），你可能需要考慮將你的子元件變成一個回傳 ",
            ),
            code("Html"),
            text(" 的方法，讓父元件呼叫，以避免資料被複製。",),
        ],
        p![
            text("如果你不需要改變傳下去的資料，你可以用 "),
            code("Rc"),
            text(" 將資料包裝起來，這樣就會只複製參考的指針，而不是資料本身。"),
        ],
        h2![text("範例")],
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
}"#,
        ),
    ])
);
