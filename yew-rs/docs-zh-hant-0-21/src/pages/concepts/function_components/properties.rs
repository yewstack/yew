crate::doc_page!(
    "屬性 (Properties)",
    "/zh-Hant/docs/concepts/function-components/properties",
    Content::new(vec![
        admonition!(
            AdmonitionType::Note,
            None,
            p!["屬性 (Properties) 通常被簡稱為 \"Props\"。"],
        ),
        p!["屬性 (Properties) 本質上是 Yew 可以監視的元件參數。"],
        p![
            "一個型別必須先實作 ",
            code("Properties"),
            " 特徵才能被用作元件的屬性。",
        ],
        h2!["響應性"],
        p!["Yew 在重新渲染時會在協調虛擬 DOM 時檢查 props \
            是否已更改，以了解是否需要重新渲染嵌套元件。這樣，Yew \
            可以被認為是一個非常響應式的框架，因為來自父元件的更改總是會向下傳播，\
            視圖永遠不會與來自 props/狀態的資料不同步。"],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "如果您還沒有完成 ",
                link!("../../tutorial", "教程"),
                "，請嘗試一下並自己測試這種響應性！",
            ],
        ),
        h2!["派生巨集"],
        p![
            "Yew 提供了一個派生巨集來輕鬆地在結構體上實作 ",
            code("Properties"),
            " 特徵。",
        ],
        p![
            "派生 ",
            code("Properties"),
            " 的型別還必須實作 ",
            code("PartialEq"),
            "，以便 Yew 可以進行資料比較。",
        ],
        code_block(
            "rust",
            r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#,
        ),
        h2!["在函數元件中使用"],
        p![
            "屬性 ",
            code("#[function_component]"),
            " 允許可選地在函數參數中接收 Props。要提供它們，它們透過 ",
            code("html!"),
            " 巨集中的屬性分配。",
        ],
        tabs!(
            "with-props",
            tab!(
                "With Props",
                "with-props",
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// 然後提供屬性
#[function_component]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"#,
                ),
            ),
            tab!(
                "No Props",
                "no-props",
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// 沒有要提供的屬性
#[function_component]
fn App() -> Html {
    html! {<HelloWorld />}
}"#,
                ),
            ),
        ),
        h2!["派生巨集欄位屬性"],
        p![
            "派生 ",
            code("Properties"),
            " 時，預設情況下所有欄位都是必需的。",
        ],
        p!["以下屬性允許您為屬性提供預設值，當父元件沒有設定它們時將使用這些預設值。"],
        admonition!(
            AdmonitionType::Tip,
            None,
            p!["屬性在 Rustdoc 生成的文件中不可見。\
                 您的屬性的文件字串應該提及屬性是否是可選的以及是否有特殊的預設值。"],
        ),
        tabs!(
            "prop_or_default",
            tab!(
                "#[prop_or_default]",
                "prop_or_default",
                p![
                    "使用 ",
                    code("Default"),
                    " 特徵用欄位型別的預設值初始化屬性值。",
                ],
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading.clone() {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}

// 然後像這樣使用預設值
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// 或者不覆蓋預設值
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"#,
                ),
            ),
            tab!(
                "#[prop_or(value)]",
                "prop_or_value",
                p![
                    "使用 ",
                    code("value"),
                    " 來初始化屬性值。",
                    code("value"),
                    " 可以是任何返回欄位型別的表達式。例如，要將布林屬性預設為 ",
                    code("true"),
                    "，請使用屬性 ",
                    code("#[prop_or(true)]"),
                    "。表達式在建構屬性時被評估，並且沒有給出明確值時應用。",
                ],
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("Bob".to_string())]
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// 然後像這樣使用預設值
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// 或者不覆蓋預設值
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"#,
                ),
            ),
            tab!(
                "#[prop_or_else(function)]",
                "prop_or_else_function",
                p![
                    "呼叫 ",
                    code("function"),
                    " 來初始化屬性值。",
                    code("function"),
                    " 應該有簽名 ",
                    code("FnMut() -> T"),
                    "，其中 ",
                    code("T"),
                    " 是欄位型別。當沒有為該屬性給出明確值時，該函數被呼叫。",
                ],
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html, Properties};

fn create_default_name() -> String {
    "Bob".to_string()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(create_default_name)]
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// 然後像這樣使用預設值
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// 或者不覆蓋預設值
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"#,
                ),
            ),
        ),
        h2!["使用 Properties 的記憶體/速度開銷"],
        p![
            "內部屬性是引用計數的。這意味著只有一個共享指標會沿著元件樹向下傳遞給 \
             props。這節省了我們不得不複製整個 props 的成本，這可能很昂貴。"
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "使用 ",
                code("AttrValue"),
                "，這是我們用於屬性值的自訂型別，而不是將它們定義為 String 或其他類似型別。",
            ],
        ),
        h2!["Props 巨集"],
        p![
            code("yew::props!"),
            " 巨集允許您以與 ",
            code("html!"),
            " 巨集相同的方式建構屬性。",
        ],
        p![
            "巨集使用與結構體表達式相同的語法，除了您不能使用屬性或基本表達式（",
            code("Foo { ..base }"),
            "）。型別路徑可以直接指向 props（",
            code("path::to::Props"),
            "）或指向元件的關聯屬性（",
            code("MyComp::Properties"),
            "）。",
        ],
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html, Properties, props, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Bob"))]
    pub name: AttrValue,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

#[function_component]
fn App() -> Html {
    let pre_made_props = props! {
        Props {}
    };
    html! {<HelloWorld ..pre_made_props />}
}"#,
        ),
        h2!["評估順序"],
        p!["Props 按指定的順序進行評估，如以下示例所示："],
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
}"#,
        ),
        h2!["反模式"],
        p!["雖然幾乎任何 Rust \
            型別都可以作為屬性傳遞，但有一些應該避免的反模式。這些包括但不限於："],
        ol![
            li![
                "使用 ",
                code("String"),
                " 型別而不是 ",
                code("AttrValue"),
                "。",
            ],
            li!["使用內部可變性。"],
            li![
                "使用 ",
                code("Vec<T>"),
                " 型別而不是 ",
                code("IArray<T>"),
                "。",
            ],
            li![
                "您發覺可能的新內容。您是否遇到了一個希望早點了解清楚的邊緣情況？\
                 請隨時建立一個問題或向本文檔提供修復的 PR。"
            ],
        ],
        p![
            bold!["為什麼不好？"],
            " ",
            code("String"),
            " 複製成本很高。當屬性值與鉤子和回呼一起使用時，通常需要複製。",
            code("AttrValue"),
            " 是一個引用計數的字串 (",
            code("Rc<str>"),
            ") 或一個 ",
            code("&'static str"),
            "，因此非常便宜複製。",
        ],
        p![
            bold!["注意"],
            "：",
            code("AttrValue"),
            " 內部是來自 ",
            link!("https://crates.io/crates/implicit-clone", "implicit-clone",),
            " 的 ",
            code("IString"),
            "。查看該包以了解更多資訊。",
        ],
        p![
            bold!["為什麼不好？"],
            " 內部可變性（例如 ",
            code("RefCell"),
            "、",
            code("Mutex"),
            " 等）應該通常避免使用。它可能會導致重新渲染問題（Yew \
             不知道狀態何時發生了變化），因此您可能需要手動強制重新渲染。就像所有事物一樣，\
             它有其用武之地。請謹慎使用。",
        ],
        p![
            bold!["為什麼不好？"],
            " ",
            code("Vec<T>"),
            "，就像 ",
            code("String"),
            " 一樣，複製成本也很高。",
            code("IArray<T>"),
            " 是一個引用計數的切片 (",
            code("Rc<[T]>"),
            ") 或一個 ",
            code("&'static [T]"),
            "，因此非常便宜複製。",
        ],
        p![
            bold!["注意"],
            "：",
            code("IArray<T>"),
            " 可以從 ",
            link!("https://crates.io/crates/implicit-clone", "implicit-clone",),
            " 匯入。查看該包以了解更多資訊。",
        ],
        h2!["yew-autoprops"],
        p![
            link!("https://crates.io/crates/yew-autoprops", "yew-autoprops",),
            " 是一個實驗性包，可讓您根據函數的參數動態建立 Props \
             結構體。如果屬性結構體永遠不會被重複使用，這可能會很有用。",
        ],
    ])
);
