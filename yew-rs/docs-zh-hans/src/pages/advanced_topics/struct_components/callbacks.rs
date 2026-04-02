pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["回调函数 (Callbacks)"],
        p![
            "回调函数是用于在 Yew 中与服务、代理和父组件进行通信的。在内部，它们的类型只是 ",
            code("Fn"),
            " 包装在 ",
            code("Rc"),
            " 中，以允许它们被克隆。",
        ],
        p![
            "它们有一个 ",
            code("emit"),
            " 函数，该函数以其 ",
            code("<IN>"),
            " 类型作为参数，并将其转换为其目标期望的消息。如果父组件中的回调函数作为 props \
             提供给子组件，子组件可以在其 ",
            code("update"),
            " 生命周期钩子中调用回调函数的 ",
            code("emit"),
            " 函数，以将消息发送回其父组件。在 ",
            code("html!"),
            " 宏中作为 props 提供的闭包或函数会自动转换为回调函数。",
        ],
        p!["一个简单的回调函数的使用可能如下所示："],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html};

enum Msg {
    Clicked,
}

struct Comp;

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // highlight-next-line
        let onclick = ctx.link().callback(|_| Msg::Clicked);
        html! {
            // highlight-next-line
            <button {onclick}>{ "Click" }</button>
        }
    }
}"#,
        ),
        p![
            "这个函数传递给 ",
            code("callback"),
            " 必须始终带有一个参数。例如，",
            code("onclick"),
            " 处理程序需要一个接受 ",
            code("MouseEvent"),
            " 类型参数的函数。然后处理程序可以决定应该发送什么类型的消息给组件。\
             这个消息无条件地被安排在下一个更新循环中。",
        ],
        p![
            "如果你需要一个回调函数，它可能不需要引起更新，请使用 ",
            code("batch_callback"),
            "。",
        ],
        code_block(
            "rust",
            r#"use yew::{events::KeyboardEvent, html, Component, Context, Html};

enum Msg {
    Submit,
}

struct Comp;

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // highlight-start
        let onkeypress = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(Msg::Submit)
            } else {
                None
            }
        });

        html! {
            <input type="text" {onkeypress} />
        }
        // highlight-end
    }
}"#,
        ),
        h2!["相关示例"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/counter",
                "Counter",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/timer",
                "Timer",
            )],
        ],
    ])
}

crate::doc_page!(
    "回调函数 (Callbacks)",
    "/zh-Hans/docs/advanced-topics/struct-components/callbacks",
    page_content()
);
