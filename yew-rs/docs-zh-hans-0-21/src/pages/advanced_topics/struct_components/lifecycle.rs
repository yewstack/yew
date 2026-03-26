crate::doc_page!(
    "生命周期",
    "/zh-Hans/docs/advanced-topics/struct-components/lifecycle",
    Content::new(vec![
        p![
            code("Component"),
            " trait 有许多需要实现的方法；Yew 会在组件生命周期的不同阶段调用这些方法。"
        ],
        h2!["生命周期"],
        admonition(
            AdmonitionType::Important,
            Some("contribute"),
            vec![p![
                code("Contribute to our docs:"),
                " ",
                link!(
                    "https://github.com/yewstack/yew/issues/1915",
                    "Add a diagram of the component lifecycle"
                )
            ]]
        ),
        h2!["生命周期方法"],
        h3!["Create"],
        p![
            "当一个组件被创建时，它会从其父组件接收属性（properties）并存储在传递给 ",
            code("create"),
            " 方法的 ",
            code("Context<Self>"),
            " 中。 属性（properties）可用于初始化组件的状态，\"link\"\
             可用于注册回调或向组件发送消息。"
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
            // impl
        }
    }
}"#
        ),
        h3!["View"],
        p![
            code("view"),
            " 方法允许你描述组件应该如何渲染到 DOM。使用 Rust 函数编写类似 HTML \
             的代码可能会变得相当混乱， 因此 Yew 提供了一个叫做 ",
            code("html!"),
            " 的宏用于声明 HTML 和 SVG 节点（以及附加属性和事件监听器）， \
             并提供了一种方便的方式来渲染子组件。这个宏在某种程度上类似于 React 的 \
             JSX（撇开编程语言的差异不谈）。 一个区别是 Yew 提供了类似 Svelte \
             的属性简写语法，你可以直接写 ",
            code("{{onclick}}"),
            "，而不是写 ",
            code("onclick={{onclick}}"),
            "。"
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
}"#
        ),
        p![
            "有关用法的详细信息，请查看 ",
            link!("concepts/html/introduction.mdx", "html! 宏指南"),
            "。"
        ],
        h3!["Rendered"],
        p![
            code("rendered"),
            " 组件生命周期方法在 ",
            code("view"),
            " 被调用并且 Yew 已将结果渲染到 DOM 之后，但在浏览器刷新页面之前被调用。 \
             当你想要执行只能在组件渲染元素之后才能完成的操作时，这个方法非常有用。 还有一个名为 ",
            code("first_render"),
            " 的参数，可用于确定此函数是在第一次渲染时被调用，还是在后续渲染时被调用。"
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
}"#
        ),
        admonition(
            AdmonitionType::Tip,
            Some("note"),
            vec![p![
                "注意，此生命周期方法不需要实现，默认情况下不会执行任何操作。"
            ]]
        ),
        h3!["Update"],
        p![
            "与组件的通信主要通过消息进行，这些消息由 ",
            code("update"),
            " 生命周期方法处理。 这允许组件根据消息的内容更新自身，并决定是否需要重新渲染。 \
             消息可以由事件监听器、子组件、Agents、Services 或 Futures 发送。"
        ],
        p!["以下是 ", code("update"), " 实现的示例："],
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
                    true // Re-render
                } else {
                    false
                }
            }
        }
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl
        }
    }

}"#
        ),
        h3!["Changed"],
        p!["组件可能会被其父组件重新渲染。当这种情况发生时，\
            它们可能会收到新的属性并需要重新渲染。 \
            这种设计通过简单地改变属性值来促进父组件到子组件的通信。 \
            有一个默认实现会在属性发生变化时重新渲染组件。"],
        h3!["Destroy"],
        p![
            "组件从 DOM 卸载后，Yew 会调用 ",
            code("destroy"),
            " 生命周期方法； 如果你需要在组件销毁之前清理组件先前操作的内容，这是必要的。 \
             此方法是可选的，默认情况下不执行任何操作。"
        ],
        h3!["无限循环"],
        p![
            "使用 Yew 的生命周期方法可能会出现无限循环，但只有在尝试在每次渲染后更新同一组件， \
             并且该更新也请求组件重新渲染时才会发生。"
        ],
        p!["一个简单的示例如下："],
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
        // We are going to always request to re-render on any msg
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // For this example it doesn't matter what is rendered
        Html::default()
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // Request that the component is updated with this new msg
        ctx.link().send_message(());
    }
}"#
        ),
        p!["让我们来看看这里发生了什么："],
        ol![
            li!["使用 ", code("create"), " 函数创建组件。"],
            li![
                "调用 ",
                code("view"),
                " 方法，以便 Yew 知道要渲染什么到浏览器 DOM。"
            ],
            li![
                "调用 ",
                code("rendered"),
                " 方法，它使用 ",
                code("Context"),
                " 链接安排了一个更新消息。"
            ],
            li!["Yew 完成后渲染阶段。"],
            li!["Yew 检查预定的事件，发现更新消息队列不为空，因此处理这些消息。"],
            li![
                "调用 ",
                code("update"),
                " 方法，它返回 ",
                code("true"),
                " 表示有内容已更改，组件需要重新渲染。"
            ],
            li!["跳回步骤 2。"]
        ],
        p![
            "你仍然可以在 ",
            code("rendered"),
            " 方法中安排更新，这通常很有用， 但在这样做时要考虑你的组件将如何终止这个循环。"
        ],
        h2!["关联类型"],
        p![
            code("Component"),
            " trait 有两个关联类型：",
            code("Message"),
            " 和 ",
            code("Properties"),
            "。"
        ],
        code_block_ignore(
            "rust",
            r#"impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}"#
        ),
        p![
            code("Message"),
            " 类型用于在事件发生后向组件发送消息； \
             例如，你可能希望在用户点击按钮或向下滚动页面时执行某些操作。 \
             由于组件通常需要响应多个事件，",
            code("Message"),
            " 类型通常是一个枚举， 其中每个变体都是要处理的事件。"
        ],
        p![
            "在组织代码库时，明智的做法是将 ",
            code("Message"),
            " 类型的定义包含在定义组件的同一模块中。 \
             你可能会发现采用一致的消息类型命名约定很有帮助。 \
             一种选择（尽管不是唯一的）是将类型命名为 ",
            code("ComponentNameMsg"),
            "， 例如，如果你的组件叫 ",
            code("Homepage"),
            "，那么你可能会将类型命名为 ",
            code("HomepageMsg"),
            "。"
        ],
        code_block(
            "rust",
            r#"enum Msg {
    Click,
    FormInput(String)
}"#
        ),
        p![
            code("Properties"),
            " 表示从父组件传递给组件的信息。此类型必须实现 ",
            code("Properties"),
            " trait（通常通过派生它）并可以指定某些属性是必需的还是可选的。\
             此类型在创建和更新组件时使用。常见的做法是在组件模块中创建一个名为 ",
            code("Props"),
            " 的结构体，并将其用作组件的 ",
            code("Properties"),
            " 类型。通常将\"properties\"缩写为\"props\"。由于 props \
             是从父组件传递下来的，应用程序的根组件通常具有 ",
            code("()"),
            " 类型的 ",
            code("Properties"),
            "。如果你希望为根组件指定属性，请使用 ",
            code("App::mount_with_props"),
            " 方法。"
        ],
        admonition(
            AdmonitionType::Info,
            None,
            vec![p![link!("", "了解更多关于属性的信息")]]
        ),
        h2!["生命周期上下文"],
        p![
            "所有组件生命周期方法都接受一个上下文对象。此对象提供了对组件作用域的引用， \
             允许向组件发送消息以及访问传递给组件的 props。"
        ]
    ])
);
