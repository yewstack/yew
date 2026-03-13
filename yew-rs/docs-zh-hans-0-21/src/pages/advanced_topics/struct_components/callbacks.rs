crate::doc_page!(
    "回调（Callbacks）",
    "/zh-Hans/docs/advanced-topics/struct-components/callbacks",
    Content::new(vec![
        h2(vec![text("Callbacks")]),
        p(vec![
            text("Callbacks 用于与 Yew 中的 services，agents 和父组件进行通信。它们仅仅是个 "),
            code("Fn"),
            text("，并由 "),
            code("Rc"),
            text(" 包裹以允许被克隆。")
        ]),
        p(vec![
            text("它们有一个 "),
            code("emit"),
            text(" 函数，该函数将它的 "),
            code("<IN>"),
            text(
                " 类型作为参数并将其转换为目标所期望的消息。如果一个回调从父组件中通过 props \
                 提供给子组件，则子组件可以在其 "
            ),
            code("update"),
            text(" 生命周期钩子中对该回调调用 "),
            code("emit"),
            text("，以将消息发送回父组件。在 "),
            code("html!"),
            text(" 宏内被提供作为 props 的闭包或函数会自动转换为 Callbacks。")
        ]),
        p(vec![text(
            "A simple use of a callback might look something like this:"
        )]),
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
}"#
        ),
        p(vec![
            text("The function passed to "),
            code("callback"),
            text(" must always take a parameter. For example, the "),
            code("onclick"),
            text(" handler requires a function that takes a parameter of type "),
            code("MouseEvent"),
            text(
                ". The handler can then decide what kind of message should be sent to the \
                 component. This message is scheduled for the next update loop unconditionally."
            )
        ]),
        p(vec![
            text("If you need a callback that might not need to cause an update, use "),
            code("batch_callback"),
            text(".")
        ]),
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
}"#
        ),
        h2(vec![text("Relevant examples")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/counter",
                vec![text("Counter")]
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/timer",
                vec![text("Timer")]
            )])
        ])
    ])
);
