crate::doc_page!(
    "属性 (Properties)",
    "/zh-Hans/docs/concepts/function-components/properties",
    Content::new(vec![
        admonition(AdmonitionType::Note, None, vec![p(vec![text("属性 (Properties) 通常被简称为 \"Props\"。")])]),
        p(vec![text("属性 (Properties) 本质上是 Yew 可以监视的组件参数。")]),
        p(vec![text("一个类型必须先实现 "), code("Properties"), text(" 特征才能被用作组件的属性。")]),
        h2(vec![text("响应性")]),
        p(vec![text("Yew 在重新渲染时会在协调虚拟 DOM 时检查 props 是否已更改，以了解是否需要重新渲染嵌套组件。这样，Yew 可以被认为是一个非常响应式的框架，因为来自父组件的更改总是会向下传播，视图永远不会与来自 props/状态的数据不同步。")]),
        admonition(AdmonitionType::Tip, None, vec![p(vec![text("如果您还没有完成 "), link("", vec![text("教程")]), text("，请尝试一下并自己测试这种响应性！")])]),
        h2(vec![text("派生宏")]),
        p(vec![text("Yew 提供了一个派生宏来轻松地在结构体上实现 "), code("Properties"), text(" 特征。")]),
        p(vec![text("派生 "), code("Properties"), text(" 的类型还必须实现 "), code("PartialEq"), text("，以便 Yew 可以进行数据比较。")]),
        code_block("rust", r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#),
        h2(vec![text("在函数组件中使用")]),
        p(vec![text("属性 "), code("#[function_component]"), text(" 允许可选地在函数参数中接收 Props。要提供它们，它们通过 "), code("html!"), text(" 宏中的属性分配。")]),
        tabs("with-props", vec![
            tab("with-props", "With Props", vec![
                code_block("rust", r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// 然后提供属性
#[function_component]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}
"#),
            ]),
            tab("no-props", "No Props", vec![
                code_block("rust", r#"use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// 没有要提供的属性
#[function_component]
fn App() -> Html {
    html! {<HelloWorld />}
}
"#),
            ]),
        ]),
        h2(vec![text("派生宏字段属性")]),
        p(vec![text("派生 "), code("Properties"), text(" 时，默认情况下所有字段都是必需的。 以下属性允许您为属性提供默认值，当父组件没有设置它们时将使用这些默认值。")]),
        admonition(AdmonitionType::Tip, None, vec![p(vec![text("属性在 Rustdoc 生成的文档中不可见。您的属性的文档字符串应该提及属性是否是可选的以及是否有特殊的默认值。")])]),
        tabs("prop_or_default", vec![
            tab("prop_or_default", "#[prop_or_default]", vec![
                p(vec![text("使用 "), code("Default"), text(" 特征用字段类型的默认值初始化属性值。")]),
                code_block("rust", r#"use yew::{function_component, html, Html, Properties};

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

// 然后像这样使用默认值
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// 或者不覆盖默认值
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"#),
            ]),
            tab("prop_or_value", "#[prop_or(value)]", vec![
                p(vec![text("使用 "), code("value"), text(" 来初始化属性值。"), code("value"), text(" 可以是任何返回字段类型的表达式。 例如，要将布尔属性默认为 "), code("true"), text("，请使用属性 "), code("#[prop_or(true)]"), text("。表达式在构造属性时被评估，并且没有给出明确值时应用。")]),
                code_block("rust", r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("Bob".to_string())]
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// 然后像这样使用默认值
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// 或者不覆盖默认值
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"#),
            ]),
            tab("prop_or_else_function", "#[prop_or_else(function)]", vec![
                p(vec![text("调用 "), code("function"), text(" 来初始化属性值。"), code("function"), text(" 应该有签名 "), code("FnMut() -> T"), text("，其中 "), code("T"), text(" 是字段类型。当没有为该属性给出明确值时，该函数被调用。")]),
                code_block("rust", r#"use yew::{function_component, html, Html, Properties};

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

// 然后像这样使用默认值
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// 或者不覆盖默认值
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"#),
            ]),
        ]),
        h2(vec![text("使用 Properties 的内存/速度开销")]),
        p(vec![text("内部属性是引用计数的。这意味着只有一个共享指针会沿着组件树向下传递给 props。这节省了我们不得不克隆整个 props 的成本，这可能很昂贵。")]),
        admonition(AdmonitionType::Tip, None, vec![p(vec![text("使用 "), code("AttrValue"), text("，这是我们用于属性值的自定义类型，而不是将它们定义为 String 或其他类似类型。")])]),
        h2(vec![text("Props 宏")]),
        p(vec![code("yew::props!"), text(" 宏允许您以与 "), code("html!"), text(" 宏相同的方式构建属性。")]),
        p(vec![text("宏使用与结构体表达式相同的语法，除了您不能使用属性或基本表达式（"), code("Foo { ..base }"), text("）。类型路径可以直接指向 props（"), code("path::to::Props"), text("）或指向组件的关联属性（"), code("MyComp::Properties"), text("）。")]),
        code_block("rust", r#"use yew::{function_component, html, Html, Properties, props, virtual_dom::AttrValue};

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
    // highlight-start
    let pre_made_props = props! {
        Props {} // 注意我们不需要指定 name 属性
    };
    // highlight-end
    html! {<HelloWorld ..pre_made_props />}
}"#),
        h2(vec![text("评估顺序")]),
        p(vec![text("Props 按指定的顺序进行评估，如以下示例所示：")]),
        code_block("rust", r#"#[derive(yew::Properties, PartialEq)]
struct Props { first: usize, second: usize, last: usize }

fn main() {
    let mut g = 1..=3;
    let props = yew::props!(Props { first: g.next().unwrap(), second: g.next().unwrap(), last: g.next().unwrap() });

    assert_eq!(props.first, 1);
    assert_eq!(props.second, 2);
    assert_eq!(props.last, 3);
}"#),
        h2(vec![text("反模式")]),
        p(vec![text("虽然几乎任何 Rust 类型都可以作为属性传递，但有一些应该避免的反模式。这些包括但不限于：")]),
        ol(vec![li(vec![text("使用 "), code("String"), text(" 类型而不是 "), code("AttrValue"), text("。 <br />")])]),
        p(vec![bold(vec![text("为什么不好？")]), text(" "), code("String"), text(" 克隆成本很高。当属性值与钩子和回调一起使用时，通常需要克隆。"), code("AttrValue"), text(" 是一个引用计数的字符串 ("), code("Rc<str>"), text(") 或一个 "), code("&'static str"), text("，因此非常便宜克隆。<br /> "), bold(vec![text("注意")]), text("："), code("AttrValue"), text(" 内部是来自 "), link("https://crates.io/crates/implicit-clone", vec![text("implicit-clone")]), text(" 的 "), code("IString"), text("。查看该包以了解更多信息。")]),
        ol(vec![li(vec![text("使用内部可变性。 <br />")])]),
        p(vec![bold(vec![text("为什么不好？")]), text(" 内部可变性（例如 "), code("RefCell"), text("、"), code("Mutex"), text(" 等）应该 _通常_ 避免使用。它可能会导致重新渲染问题（Yew 不知道状态何时发生了变化），因此您可能需要手动强制重新渲染。就像所有事物一样，它有其用武之地。请谨慎使用。")]),
        ol(vec![li(vec![text("使用 "), code("Vec<T>"), text(" 类型而不是 "), code("IArray<T>"), text("。 <br />")])]),
        p(vec![bold(vec![text("为什么不好？")]), text(" "), code("Vec<T>"), text("，就像 "), code("String"), text(" 一样，克隆成本也很高。"), code("IArray<T>"), text(" 是一个引用计数的切片 ("), code("Rc<[T]>"), text(") 或一个 "), code("&'static [T]"), text("，因此非常便宜克隆。<br /> "), bold(vec![text("注意")]), text("："), code("IArray<T>"), text(" 可以从 "), link("https://crates.io/crates/implicit-clone", vec![text("implicit-clone")]), text(" 导入。查看该包以了解更多信息。")]),
        ol(vec![li(vec![text("您发觉可能的新内容。您是否遇到了一个希望早点了解清楚的边缘情况？请随时创建一个问题或向本文档提供修复的 PR。")])]),
        h2(vec![text("yew-autoprops")]),
        p(vec![link("https://crates.io/crates/yew-autoprops", vec![text("yew-autoprops")]), text(" 是一个实验性包，允许您根据函数的参数动态创建 Props 结构体。如果属性结构体永远不会被重用，这可能会很有用。")])
    ])
);
