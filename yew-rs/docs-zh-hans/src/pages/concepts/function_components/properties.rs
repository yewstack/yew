pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text("属性 (Properties) 通常被简写为 \"Props\"。")])],
        ),
        p(vec![text(
            "属性 (Properties) 是组件的参数，Yew 可以监视这些参数。",
        )]),
        p(vec![
            text("在组件的属性中使用一个类型之前，它必须实现 "),
            code("Properties"),
            text(" trait。"),
        ]),
        h2(vec![text("响应性")]),
        p(vec![text(
            "在重新渲染时，Yew 在协调虚拟 DOM \
             时检查属性是否已更改，以了解是否需要重新渲染嵌套组件。这样，Yew \
             可以被认为是一个非常具有响应性的框架，因为来自父组件的更改总是会向下传播，\
             视图永远不会与来自属性/状态的数据不同步。",
        )]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("如果您尚未完成 "),
                link("/zh-Hans/docs/tutorial", vec![text("教程")]),
                text("，请尝试并自行测试这种响应性！"),
            ])],
        ),
        h2(vec![text("派生宏")]),
        p(vec![
            text("Yew 提供了一个派生宏，可以轻松地在结构体上实现 "),
            code("Properties"),
            text(" trait。"),
        ]),
        p(vec![
            text("您派生 "),
            code("Properties"),
            text(" 的类型也必须实现 "),
            code("PartialEq"),
            text("，以便 Yew 可以进行数据比较。"),
        ]),
        code_block(
            "rust",
            r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#,
        ),
        h2(vec![text("在函数组件中使用")]),
        p(vec![
            text("属性 "),
            code("#[component]"),
            text(" 允许在函数参数中选择性地接收 Props。要提供它们，可以通过 "),
            code("html!"),
            text(" 宏中的属性进行赋值。"),
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

// 然后提供属性
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

// 没有属性需要提供
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
                    )],
                ),
            ],
        ),
        h2(vec![text("派生宏字段属性")]),
        p(vec![
            text("在派生 "),
            code("Properties"),
            text(" 时，默认情况下所有字段都是必需的。"),
        ]),
        p(vec![text(
            "以下属性允许您为属性提供默认值，当父组件没有设置它们时将使用这些默认值。",
        )]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![text(
                "属性在 Rustdoc \
                 生成的文档中是不可见的。您的属性的文档字符串应该提到一个属性是否是可选的，\
                 以及它是否有一个特殊的默认值。",
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
                            text(" trait 的字段类型的默认值初始化属性值。"),
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

// 这样使用默认值
#[component]
fn Case1() -> Html {
    html! { <HelloWorld /> }
}
// 或者不覆盖默认值
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
                            text(" 来初始化属性值。"),
                            code("value"),
                            text(" 可以是返回字段类型的任何表达式。例如，要将布尔属性默认为 "),
                            code("true"),
                            text("，请使用属性 "),
                            code("#[prop_or(true)]"),
                            text("。当属性被构造时，表达式会被评估，且没有给出明确的值。"),
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

// 这样使用默认值
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// 或者不覆盖默认值
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
                            text("调用 "),
                            code("function"),
                            text(" 来初始化属性值。"),
                            code("function"),
                            text(" 应该具有 "),
                            code("FnMut() -> T"),
                            text(" 签名，其中 "),
                            code("T"),
                            text(
                                " 是字段类型。当没有为该属性给出明确的值时，将调用该函数。\
                                 这个函数在属性被构造时被调用。",
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

// 使用默认值
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// 或者不覆盖默认值
#[component]
fn Case2() -> Html {
    html! { <Hello name="Sam" /> }
}"#,
                        ),
                    ],
                ),
            ],
        ),
        h2(vec![text("使用 Properties 的性能开销")]),
        p(vec![text(
            "内部属性是以引用计数的智能指针传递的。\
             这意味着只有一个共享指针被传递到组件树中的属性，这样就能节约克隆整个属性的高昂成本。",
        )]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                code("AttrValue"),
                text(
                    " 是我们用于属性值的自定义类型，这样就不用将它们定义为 String \
                     或其他类似克隆成本高昂的类型了。",
                ),
            ])],
        ),
        h2(vec![text("Props 宏")]),
        p(vec![
            code("yew::props!"),
            text(" 宏允许您以与 "),
            code("html!"),
            text(" 宏相同的方式构建属性。"),
        ]),
        p(vec![
            text("该宏使用与结构表达式相同的语法，只是您不能使用属性或基本表达式 ("),
            code("Foo { ..base }"),
            text(")。类型路径可以直接指向属性 ("),
            code("path::to::Props"),
            text(")，也可以指向组件的关联属性 ("),
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
        Props {} // 注意我们不需要指定 name 属性
    };
    // highlight-end
    html! { <Hello ..pre_made_props /> }
}"#,
        ),
        h2(vec![text("自动生成属性 (yew-autoprops)")]),
        p(vec![
            text("为了简化您的开发流程，您还可以使用宏 "),
            code("#[autoprops]"),
            text("（来自 "),
            code("yew-autoprops"),
            text(" 包）自动生成 "),
            code("Properties"),
            text(" 结构体。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew_autoprops::autoprops;

// #[autoprops] 宏必须出现在 #[component] 之前，顺序很重要
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

// 结构体 "GreetingsProps" 将会被自动生成。
//
// `is_loading` 将作为值传递给组件，而 `message` 和 `name` 将使用引用，因为定义中有一个前导的 `&`。"#,
        ),
        h2(vec![text("评估顺序")]),
        p(vec![text("属性按照指定的顺序进行评估，如下例所示：")]),
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
            "虽然几乎任何 Rust 类型都可以作为属性传递，但有一些反模式应该避免。这些包括但不限于：",
        )]),
        ol(vec![
            li_blocks(vec![
                p(vec![
                    text("使用 "),
                    code("String"),
                    text(" 类型而不是 "),
                    code("AttrValue"),
                    text("。"),
                ]),
                p(vec![
                    bold(vec![text("为什么不好？")]),
                    text(" "),
                    code("String"),
                    text(" 克隆成本高昂。当属性值与钩子和回调一起使用时，通常需要克隆。"),
                    code("AttrValue"),
                    text(" 是一个引用计数的字符串 ("),
                    code("Rc<str>"),
                    text(") 或一个 "),
                    code("&'static str"),
                    text("，因此非常便宜克隆。"),
                ]),
                p(vec![
                    bold(vec![text("注意")]),
                    text("："),
                    code("AttrValue"),
                    text(" 在内部是来自 "),
                    link(
                        "https://crates.io/crates/implicit-clone",
                        vec![text("implicit-clone")],
                    ),
                    text(" 的 "),
                    code("IString"),
                    text("。查看该包以了解更多信息。"),
                ]),
            ]),
            li_blocks(vec![
                p(vec![text("使用内部可变性。")]),
                p(vec![
                    bold(vec![text("为什么不好？")]),
                    text(" 内部可变性（例如 "),
                    code("RefCell"),
                    text("、"),
                    code("Mutex"),
                    text(
                        " 等）应该 _通常_ 避免使用。它可能会导致重新渲染问题（Yew \
                         不知道状态何时发生了变化），因此您可能需要手动强制重新渲染。\
                         就像所有事物一样，它有其用武之地。请谨慎使用。",
                    ),
                ]),
            ]),
            li_blocks(vec![
                p(vec![
                    text("使用 "),
                    code("Vec<T>"),
                    text(" 类型而不是 "),
                    code("IArray<T>"),
                    text("。"),
                ]),
                p(vec![
                    bold(vec![text("为什么不好？")]),
                    text(" "),
                    code("Vec<T>"),
                    text("，就像 "),
                    code("String"),
                    text(" 一样，克隆成本也很高。"),
                    code("IArray<T>"),
                    text(" 是一个引用计数的切片 ("),
                    code("Rc<[T]>"),
                    text(") 或一个 "),
                    code("&'static [T]"),
                    text("，因此非常便宜克隆。"),
                ]),
                p(vec![
                    bold(vec![text("注意")]),
                    text("："),
                    code("IArray"),
                    text(" 可以从 "),
                    link(
                        "https://crates.io/crates/implicit-clone",
                        vec![text("implicit-clone")],
                    ),
                    text(" 导入。查看该包以了解更多信息。"),
                ]),
            ]),
            li_blocks(vec![p(vec![text(
                "您发觉可能的新内容。您是否遇到了一个希望早点了解清楚的边缘情况？\
                 请随时创建一个问题或向本文档提供修复的 PR。",
            )])]),
        ]),
        h2(vec![text("yew-autoprops")]),
        p(vec![
            link(
                "https://crates.io/crates/yew-autoprops",
                vec![text("yew-autoprops")],
            ),
            text(
                " 是一个实验性包，允许您根据函数的参数动态创建 Props \
                 结构体。如果属性结构体永远不会被重用，这可能会很有用。",
            ),
        ]),
    ])
}

crate::doc_page!(
    "属性 (Properties)",
    "/zh-Hans/docs/concepts/function-components/properties",
    page_content()
);
