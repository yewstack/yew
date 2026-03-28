pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("Component"),
            " trait 有许多方法需要实现；Yew 会在组件的生命周期的不同阶段调用这些方法。",
        ],
        h2!["生命周期"],
        admonition![
            AdmonitionType::Important,
            Some("改进文档"),
            p![
                code("为文档做贡献："),
                " ",
                link!(
                    "https://github.com/yewstack/yew/issues/1915",
                    "添加定制生命周期的组件示例",
                ),
            ],
        ],
        h2!["生命周期方法"],
        h3!["Create"],
        p![
            "当组件被创建时，它会从其父组件接收属性，并存储在传递给 ",
            code("create"),
            " 方法的 ",
            code("Context<Self>"),
            " 中。这些属性可以用来初始化组件的状态，而 \"link\" 可以用来注册回调或向组件发送消息。",
        ],
        code_block(
            "rust",
            r#"use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    // highlight-start
    fn create(ctx: &Context<Self>) -> Self {
        MyComponent
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // 具体实现
        }
    }
}"#,
        ),
        h3!["View"],
        p![
            code("view"),
            " 方法允许您描述组件应该如何呈现到 DOM 中。使用 Rust 函数编写类似 HTML \
             的代码可能会变得非常混乱，因此 Yew 提供了一个名为 ",
            code("html!"),
            " 的宏，用于声明 HTML 和 SVG \
             节点（以及将属性和事件监听器附加到它们）以及一种方便的方法来渲染子组件。\
             这个宏在某种程度上类似于 React 的 JSX（除了编程语言的差异）。一个不同之处是 Yew \
             提供了一种类似 Svelte 的属性的简写语法，其中您可以只写 ",
            code("{onclick}"),
            "，而不用写 ",
            code("onclick={onclick}"),
            "。",
        ],
        code_block(
            "rust",
            r#"use yew::{Component, Context, html, Html, Properties};

enum Msg {
    Click,
}

#[derive(PartialEq, Properties)]
struct Props {
    button_text: String,
}

struct MyComponent;

impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // highlight-start
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Click);
        html! {
            <button {onclick}>{ &ctx.props().button_text }</button>
        }
    }
    // highlight-end
}"#,
        ),
        p![
            "就使用上的说明，请查看 ",
            link!("concepts/html/introduction.mdx", "html! 指南"),
            "。",
        ],
        h3!["Rendered"],
        p![
            code("rendered"),
            " 组件生命周期方法在 ",
            code("view"),
            " 被调用并且 Yew 已经将结果渲染到 DOM \
             中后调用，但在浏览器刷新页面之前。当您想要执行只能在组件渲染元素后完成的操作时，\
             此方法非常有用。还有一个名为 ",
            code("first_render"),
            " 的参数，可以用来确定此函数是在第一次渲染时调用，还是在后续渲染时调用。",
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::{
    Component, Context, html, Html, NodeRef,
};

pub struct MyComponent {
    node_ref: NodeRef,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input ref={self.node_ref.clone()} type="text" />
        }
    }

    // highlight-start
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }
    // highlight-end
}"#,
        ),
        admonition![
            AdmonitionType::Tip,
            Some("note"),
            p!["请注意，此生命周期方法不需要实现，并且默认情况下不会执行任何操作。"],
        ],
        h3!["Update"],
        p![
            "与组件的通信主要通过消息进行，这些消息由 ",
            code("update"),
            " 生命周期方法处理。这允许组件根据消息更新自身，并确定是否需要重新渲染自身。\
             消息可以由事件监听器、子组件、Agents、Services 或 Futures 发送。",
        ],
        p!["下面是 ", code("update"), " 的一个实现示例：",],
        code_block(
            "rust",
            r#"use yew::{Component, Context, html, Html};

// highlight-start
pub enum Msg {
    SetInputEnabled(bool)
}
// highlight-end

struct MyComponent {
    input_enabled: bool,
}

impl Component for MyComponent {
    // highlight-next-line
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_enabled: false,
        }
    }

    // highlight-start
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInputEnabled(enabled) => {
                if self.input_enabled != enabled {
                    self.input_enabled = enabled;
                    true // 重新渲染
                } else {
                    false
                }
            }
        }
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // 具体实现
        }
    }
}"#,
        ),
        h3!["Changed"],
        p!["组件可能会被其父组件重新渲染。当这种情况发生时，\
            它们可能会接收新的属性并需要重新渲染。\
            这种设计通过仅更改属性的值来促进父子组件之间的通信。当属性更改时，\
            有一个默认实现会重新渲染组件。"],
        h3!["Destroy"],
        p![
            "组件从 DOM 中卸载后，Yew 会调用 ",
            code("destroy"),
            " 生命周期方法；如果您需要在组件被销毁之前执行清理操作，这是必要的。此方法是可选的，\
             默认情况下不执行任何操作。",
        ],
        h3!["无限循环"],
        p![
            "无限循环在 Yew 的生命周期方法中是可能的，但只有在尝试在每次渲染后更新相同的组件时，\
             当该更新还要求重新渲染组件时才会发生。"
        ],
        p!["下面是一个简单的示例："],
        code_block(
            "rust",
            r#"use yew::{Context, Component, Html};

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // 我们总是请求在任何消息上重新渲染
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // 无论渲染什么都不重要
        Html::default()
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // 请求使用此新消息更新组件
        ctx.link().send_message(());
    }
}"#,
        ),
        p!["让我们看看这里发生了什么："],
        ol![
            li!["使用 ", code("create"), " 函数创建组件。"],
            li![
                "调用 ",
                code("view"),
                " 方法，以便 Yew 知道要渲染到浏览器 DOM 中的内容。",
            ],
            li![
                "调用 ",
                code("rendered"),
                " 方法，使用 ",
                code("Context"),
                " 链接安排更新消息。",
            ],
            li!["Yew 完成后渲染阶段。"],
            li!["Yew 检查已安排的事件，并看到更新消息队列不为空，因此处理消息。"],
            li![
                "调用 ",
                code("update"),
                " 方法，返回 ",
                code("true"),
                " 表示发生了变化，组件需要重新渲染。",
            ],
            li!["跳回到第 2 步。"],
        ],
        p![
            "您仍然可以在 ",
            code("rendered"),
            " 方法中安排更新，这通常是有用的，但是在这样做时，请考虑您的组件将如何终止此循环。",
        ],
        h2!["关联类型"],
        p![
            code("Component"),
            " trait 有两个关联类型：",
            code("Message"),
            " 和 ",
            code("Properties"),
            "。",
        ],
        code_block_ignore(
            "rust",
            r#"impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}"#,
        ),
        p![
            code("Message"),
            " 类型用于在事件发生后向组件发送消息；例如，\
             您可能希望在用户单击按钮或向下滚动页面时执行某些操作。因为组件通常需要响应多个事件，\
             所以 ",
            code("Message"),
            " 类型通常是一个枚举，其中每个变体都是要处理的事件。",
        ],
        p![
            "在组织代码库时，将 ",
            code("Message"),
            " 类型的定义包含在定义组件的同一模块中是明智的。\
             您可能会发现采用一致的命名约定来命名消息类型很有帮助。\
             一个选项（尽管不是唯一的选项）是将类型命名为 ",
            code("ComponentNameMsg"),
            "，例如，如果您的组件名为 ",
            code("Homepage"),
            "，则可以将类型命名为 ",
            code("HomepageMsg"),
            "。",
        ],
        code_block(
            "rust",
            r#"enum Msg {
    Click,
    FormInput(String)
}"#,
        ),
        p![
            code("Properties"),
            " 表示从其父组件传递给组件的信息。此类型必须实现 ",
            code("Properties"),
            " trait（通常通过派生它）并可以指定某些属性是必需的还是可选的。\
             在创建和更新组件时使用此类型。在组件的模块中创建一个名为 ",
            code("Props"),
            " 的结构体，并将其用作组件的 ",
            code("Properties"),
            " 类型是一种常见做法。通常将 \"properties\" 缩写为 \"props\"。由于 props \
             是从父组件传递下来的，因此应用程序的根组件通常具有 ",
            code("Properties"),
            " 类型为 ",
            code("()"),
            "。如果要为根组件指定属性，请使用 ",
            code("App::mount_with_props"),
            " 方法。",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![link!(
                "/zh-Hans/docs/advanced-topics/struct-components/properties",
                "了解更多关于属性的信息",
            )],
        ],
        h2!["生命周期上下文"],
        p![
            "所有组件生命周期方法都接受一个上下文对象。此对象提供了对组件作用域的引用，\
             允许向组件发送消息并传递给组件的 props。"
        ],
    ])
    .with_description("Components and their lifecycle hooks")
}

crate::doc_page!(
    "生命周期",
    "/zh-Hans/docs/advanced-topics/struct-components/lifecycle",
    page_content()
);
