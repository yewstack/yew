crate::doc_page!(
    "",
    "/zh-Hans/docs/advanced-topics/struct-components/lifecycle",
    Content::new(vec![
        h1!["组件（Components）"],
        h2!["组件是什么？"],
        p![
            "组件是 Yew 的基石。它们管理自己的状态，并可以渲染为 \
             DOM。组件是通过实现描述组件生命周期的 ",
            code("Component"),
            " trait 来创建的。",
        ],
        h2!["生命周期"],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("为我们的文档做出贡献："),
                link![
                    "https://github.com/yewstack/docs/issues/22",
                    "添加组件的生命周期图示",
                ],
            ],
        ],
        h2!["生命周期方法"],
        h3!["Create"],
        p![
            "当一个组件被创建时，它会从其父组件以及一个 ",
            code("ComponentLink"),
            " 接收属性（properties）。属性（properties）可用于初始化组件的状态，\"link\"\
             可用于注册回调或向组件发送消息。",
        ],
        p!["通常将 props 和 link 存储在组件的结构体中，如下所示："],
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
            "组件在 ",
            code("view()"),
            " 方法中声明它的布局。Yew 提供了 ",
            code("html!"),
            " 宏来声明 HTML 和 SVG 节点和它们的监听器及其子组件。这个宏的行为很像 React 中的 \
             JSX，但是使用的是 Rust 表达式而不是 JavaScript。",
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
            link!["concepts/html/introduction.mdx", "html! 宏指南"],
            "]",
        ],
        h3!["Mounted"],
        p![
            code("mounted()"),
            " 组件生命周期方法调用是在 ",
            code("view()"),
            " 被处理并且 Yew 已经把组件挂载到 DOM \
             上之后，浏览器刷新页面之前。\
             组件通常希望实现此方法以执行只能在组件渲染元素之后才能执行的操作。\
             如果你想在做出一些更改后重新渲染组件，返回 ",
            code("true"),
            " 就可以了。",
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
        admonition![
            AdmonitionType::Note,
            None,
            p!["请注意，此生命周期方法不要求必须实现，默认情况下不会执行任何操作。"],
        ],
        h3!["Update"],
        p![
            "组件是动态的，可以注册以接收异步信息。",
            code("update()"),
            " 生命周期方法对于每个消息都会被调用。这使得组件可以根据消息的内容来更新自身，\
             并决定是否需要重新渲染自己。消息可以由 HTML \
             元素监听器触发，或者由子组件，Agents，Services 或 Futures 发送。",
        ],
        p![code("update()"), " 可能看起来像下面这个例子："],
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
        h3!["Change"],
        p![
            "组件可能被其父节点重新渲染。发生这种情况时，\
             它们可以接收新的属性（properties）并选择重新渲染。\
             这种设计通过更改属性（properties）来促进父子组件之间的通信。你不是必须实现 ",
            code("change()"),
            "，但是如果想在组件被创建后通过 props 来更新组件，则可能要这么做。",
        ],
        p!["一个原始的实现可能看起来像："],
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
        h3!["Destroy"],
        p![
            "组件从 DOM 上被卸载后，Yew 调用 ",
            code("destroy()"),
            " 生命周期方法来支持任何必要的清理操作。这个方法是可选的，默认情况下不执行任何操作。",
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
        code_block(
            "rust",
            r#"impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}"#
        ),
        p![
            code("Message"),
            " 表示组件可以处理以触发某些副作用的各种消息。例如，你可能有一条 ",
            code("Click"),
            " 消息，该消息触发 API 请求或者切换 UI \
             组件的外观。通常的做法是在组件模块中创建一个叫做 ",
            code("Msg"),
            " 的枚举并将其用作组件中的消息类型。通常将\"message\"缩写为\"msg\"。",
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
            " 表示从父级传递到组件的信息。此类型必须实现 ",
            code("Properties"),
            " trait（通常通过派生），并且可以指定某些属性（properties）是必需的还是可选的。\
             创建和更新组件时使用此类型。通常的做法是在组件模块中创建一个叫做 ",
            code("Props"),
            " 的结构体并将其用作组件的 ",
            code("Properties"),
            " 类型。通常将\"properties\"缩写为\"props\"。由于 props \
             是从父组件传递下来的，因此应用程序的根组件通常有一个类型为 ",
            code("()"),
            " 的 ",
            code("Properties"),
            "。如果你希望为根组件指定属性（properties），请使用 ",
            code("App::mount_with_props"),
            " 方法。",
        ],
    ])
);
