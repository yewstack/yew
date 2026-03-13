pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text("屬性 (Properties) 通常被簡寫為 \"Props\"。")])],
        ),
        p(vec![text(
            "屬性 (Properties) 是元件的參數，Yew 可以監視這些參數。",
        )]),
        p(vec![
            text("在元件的屬性中使用一個類型之前，它必須實作 "),
            code("Properties"),
            text(" trait。"),
        ]),
        h2(vec![text("響應性")]),
        p(vec![text(
            "在重新渲染時，Yew 在協調虛擬 DOM \
             時檢查屬性是否已更改，以了解是否需要重新渲染巢狀元件。這樣，Yew \
             可以被認為是一個非常具有響應性的框架，因為來自父組件的變更總是會向下傳播，\
             視圖永遠不會與來自屬性/狀態的資料不同步。",
        )]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("如果您尚未完成 "),
                link("/zh-Hant/docs/tutorial", vec![text("教學")]),
                text("，請嘗試並自行測試這種回應性！"),
            ])],
        ),
        h2(vec![text("派生宏")]),
        p(vec![
            text("Yew 提供了一個衍生宏，可以輕鬆地在結構體上實作 "),
            code("Properties"),
            text(" trait。"),
        ]),
        p(vec![
            text("您衍生 "),
            code("Properties"),
            text(" 的型別也必須實作 "),
            code("PartialEq"),
            text("，以便 Yew 可以進行資料比較。"),
        ]),
        code_block(
            "rust",
            r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#,
        ),
        h2(vec![text("在函數元件中使用")]),
        p(vec![
            text("屬性 "),
            code("#[component]"),
            text(" 允許在函式參數中選擇性地接收 Props。要提供它們，可以透過 "),
            code("html!"),
            text(" 巨集中的屬性進行賦值。"),
        ]),
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

// 然後提供屬性
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

// 沒有屬性需要提供
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
                    )],
                ),
            ],
        ),
        h2(vec![text("派生巨集欄位屬性")]),
        p(vec![
            text("在派生 "),
            code("Properties"),
            text(" 時，預設情況下所有欄位都是必要的。"),
        ]),
        p(vec![text(
            "以下屬性可讓您為屬性提供預設值，當父元件沒有設定它們時將使用這些預設值。",
        )]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![text(
                "屬性在 Rustdoc \
                 產生的文檔中是不可見的。您的屬性的文檔字串應該提到一個屬性是否是可選的，\
                 以及它是否有一個特殊的預設值。",
            )])],
        ),
        tabs(
            "prop_or_default",
            vec![
                tab(
                    "prop_or_default",
                    "#[prop_or_default]",
                    vec![
                        p(vec![
                            text("使用 "),
                            code("Default"),
                            text(" trait 的欄位類型的預設值初始化屬性值。"),
                        ]),
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

// 這樣使用預設值
#[component]
fn Case1() -> Html {
    html! { <HelloWorld /> }
}
// 或不覆蓋預設值
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
                        p(vec![
                            text("使用 "),
                            code("value"),
                            text(" 來初始化屬性值。 "),
                            code("value"),
                            text(" 可以是傳回欄位類型的任何表達式。例如，要將布林屬性預設為 "),
                            code("true"),
                            text("，請使用屬性 "),
                            code("#[prop_or(true)]"),
                            text("。當屬性被建構時，表達式會被評估，且沒有給出明確的值。"),
                        ]),
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

// 這樣使用預設值
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// 或不覆蓋預設值
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
                        p(vec![
                            text("呼叫 "),
                            code("function"),
                            text(" 來初始化屬性值。 "),
                            code("function"),
                            text(" 應該有 "),
                            code("FnMut() -> T"),
                            text(" 簽名，其中 "),
                            code("T"),
                            text(
                                " 是欄位類型。當沒有為該屬性給出明確的值時，將呼叫該函數。\
                                 這個函數在屬性被建構時被呼叫。",
                            ),
                        ]),
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

// 使用預設值
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// 或不覆蓋預設值
#[component]
fn Case2() -> Html {
    html! { <Hello name="Sam" /> }
}"#,
                        ),
                    ],
                ),
            ],
        ),
        h2(vec![text("使用 Properties 的效能開銷")]),
        p(vec![text(
            "內部屬性是以引用計數的智慧型指標傳遞的。\
             這意味著只有一個共享指標被傳遞到元件樹中的屬性，這樣就能節約克隆整個屬性的高昂成本。",
        )]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                code("AttrValue"),
                text(
                    " 是我們用於屬性值的自訂類型，這樣就不用將它們定義為 String \
                     或其他類似克隆成本高昂的類型了。",
                ),
            ])],
        ),
        h2(vec![text("Props 巨集")]),
        p(vec![
            code("yew::props!"),
            text(" 巨集允許您以與 "),
            code("html!"),
            text(" 巨集相同的方式建立屬性。"),
        ]),
        p(vec![
            text("這個巨集使用與結構表達式相同的語法，只是您不能使用屬性或基本表達式 ("),
            code("Foo { ..base }"),
            text(")。類型路徑可以直接指向屬性 ("),
            code("path::to::Props"),
            text(")，也可以指向元件的關聯屬性 ("),
            code("MyComp::Properties"),
            text(")。"),
        ]),
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
        Props {} // 注意我們不需要指定 name 屬性
    };
    // highlight-end
    html! { <Hello ..pre_made_props /> }
}"#,
        ),
        h2(vec![text("自動產生屬性 (yew-autoprops)")]),
        p(vec![
            text("為了簡化您的開發流程，您也可以使用巨集 "),
            code("#[autoprops]"),
            text("（來自 "),
            code("yew-autoprops"),
            text(" 套件）自動產生 "),
            code("Properties"),
            text(" 結構體。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew_autoprops::autoprops;

// #[autoprops] 巨集必須出現在 #[component] 之前，順序很重要
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

// 結構體 "GreetingsProps" 將會自動產生。
//
// `is_loading` 將作為值傳遞給元件，而 `message` 和 `name` 將使用引用，因為定義中有一個前導的 `&`。"#,
        ),
        h2(vec![text("評估順序")]),
        p(vec![text("屬性依照指定的順序進行評估，如下例所示：")]),
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
        h2(vec![text("反模式")]),
        p(vec![text(
            "雖然幾乎任何 Rust 類型都可以作為屬性傳遞，但有一些反模式應該避免。這些包括但不限於：",
        )]),
        ol(vec![
            li(vec![
                text("使用 "),
                code("String"),
                text(" 類型而不是 "),
                code("AttrValue"),
                text("。 "),
                br(),
                bold(vec![text("為什麼不好？ ")]),
                code("String"),
                text(" 克隆成本高。當屬性值與鉤子和回調一起使用時，通常需要克隆。 "),
                code("AttrValue"),
                text(" 是一個引用計數的字串 ("),
                code("Rc<str>"),
                text(") 或一個 "),
                code("&'static str"),
                text("，因此非常便宜克隆。 "),
                br(),
                bold(vec![text("注意")]),
                text("："),
                code("AttrValue"),
                text(" 在內部是來自 "),
                link(
                    "https://crates.io/crates/implicit-clone",
                    vec![text("implicit-clone")],
                ),
                text(" 的 "),
                code("IString"),
                text("。查看該包以了解更多資訊。"),
            ]),
            li(vec![
                text("使用內部可變性。 "),
                br(),
                bold(vec![text("為什麼不好？ ")]),
                text("內部可變性（例如 "),
                code("RefCell"),
                text("、"),
                code("Mutex"),
                text(
                    " 等）應該 _通常_ 避免使用。它可能會導致重新渲染問題（Yew \
                     不知道狀態何時發生了變化），因此您可能需要手動強制重新渲染。\
                     就像所有事物一樣，它有其用武之地。請謹慎使用。",
                ),
            ]),
            li(vec![
                text("使用 "),
                code("Vec<T>"),
                text(" 型別而不是 "),
                code("IArray<T>"),
                text("。 "),
                br(),
                bold(vec![text("為什麼不好？ ")]),
                code("Vec<T>"),
                text("，就像 "),
                code("String"),
                text(" 一樣，克隆成本也很高。 "),
                code("IArray<T>"),
                text(" 是一個引用計數的切片 ("),
                code("Rc<[T]>"),
                text(") 或一個 "),
                code("&'static [T]"),
                text("，因此非常便宜克隆。 "),
                br(),
                bold(vec![text("注意")]),
                text("："),
                code("IArray"),
                text(" 可以從 "),
                link(
                    "https://crates.io/crates/implicit-clone",
                    vec![text("implicit-clone")],
                ),
                text(" 匯入。查看該包以了解更多資訊。"),
            ]),
            li(vec![text(
                "您發覺可能的新內容。您是否遇到了一個希望早點了解清楚的邊緣情況？\
                 請隨時建立一個問題或向本文檔提供修復的 PR。",
            )]),
        ]),
        h2(vec![text("yew-autoprops")]),
        p(vec![
            link(
                "https://crates.io/crates/yew-autoprops",
                vec![text("yew-autoprops")],
            ),
            text(
                " 是一個實驗性包，可讓您根據函數的參數動態建立 Props \
                 結構體。如果屬性結構體永遠不會被重複使用，這可能會很有用。",
            ),
        ]),
    ])
}

crate::doc_page!(
    "屬性 (Properties)",
    "/zh-Hant/docs/concepts/function-components/properties",
    page_content()
);
