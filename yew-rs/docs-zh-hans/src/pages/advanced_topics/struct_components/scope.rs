pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["组件的 ", code("Scope<_>"), " 接口"],
        p![
            code("Scope"),
            " 是通过消息创建回调并更新自身的机制。我们通过在传递给组件的上下文对象上调用 ",
            code("link()"),
            " 来获得对它的引用。",
        ],
        h3![code("send_message")],
        p![
            "这个函数可以向组件发送消息。消息由 ",
            code("update"),
            " 方法处理，该方法确定组件是否应重新渲染。",
        ],
        h3![code("send_message_batch")],
        p![
            "这个函数可以同时向组件发送多个消息。这类似于 ",
            code("send_message"),
            "，但是如果任何消息导致 ",
            code("update"),
            " 方法返回 ",
            code("true"),
            "，则组件将在处理完批处理中的所有消息后重新渲染。",
        ],
        p!["如果给定的参数向量为空，则此函数不执行任何操作。"],
        h3![code("callback")],
        p![
            "创建一个回调，当执行时将向组件发送消息。在内部，它将使用提供的闭包返回的消息调用 ",
            code("send_message"),
            "。",
        ],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html};

enum Msg {
    Text(String),
}

struct Comp;

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // 创建一个接受一些文本并将其作为 `Msg::Text` 消息变体发送到组件的回调。
        // highlight-next-line
        let cb = ctx.link().callback(|text: String| Msg::Text(text));

        // 上一行是多余的冗长，为了更清晰，可以简化为这样：
        // highlight-next-line
        let cb = ctx.link().callback(Msg::Text);

        // 将 `Msg::Text("Hello World!")` 发送到组件。
        // highlight-next-line
        cb.emit("Hello World!".to_owned());

        html! {
            // 在这里放置 HTML
        }
    }
}"#,
        ),
        h3![code("batch_callback")],
        p![
            "创建一个回调，当执行时将向组件发送一批消息。与 ",
            code("callback"),
            " 的区别在于，传递给此方法的闭包不必返回消息。相反，闭包可以返回 ",
            code("Vec<Msg>"),
            " 或 ",
            code("Option<Msg>"),
            "，其中 ",
            code("Msg"),
            " 是组件的消息类型。",
        ],
        p![
            code("Vec<Msg>"),
            " 被视为一批消息，并在内部使用 ",
            code("send_message_batch"),
            "。",
        ],
        p![
            code("Option<Msg>"),
            " 在值为 ",
            code("Some"),
            " 时调用 ",
            code("send_message"),
            "。如果值为 ",
            code("None"),
            "，则不执行任何操作。这可以用于根据情况，不需要更新的情况。",
        ],
        p![
            "这是通过使用仅为这些类型实现的 ",
            code("SendAsMessage"),
            " trait 实现的。您可以为自己的类型实现 ",
            code("SendAsMessage"),
            "，这样可以在 ",
            code("batch_callback"),
            " 中使用它们。",
        ],
    ])
}

crate::doc_page!(
    "作用域",
    "/zh-Hans/docs/advanced-topics/struct-components/scope",
    page_content()
);
