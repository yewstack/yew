crate::doc_page!(
    "Scope",
    "/zh-Hant/docs/advanced-topics/struct-components/scope",
    Content::new(vec![
        h2(vec![text("Component's Scope<_> API")]),
        p(vec![
            text(
                "The component \"Scope\" is the mechanism through which components can create \
                 callbacks and update themselves using messages. We obtain a reference to this by \
                 calling "
            ),
            code("link()"),
            text(" on the context object passed to the component."),
        ]),
        h3(vec![text("send_message")]),
        p(vec![
            text("Sends a message to the component. Messages are handled by the "),
            code("update"),
            text(" method which determines whether the component should re-render."),
        ]),
        h3(vec![text("send_message_batch")]),
        p(vec![
            text("Sends multiple messages to the component at the same time. This is similar to "),
            code("send_message"),
            text(" but if any of the messages cause the "),
            code("update"),
            text(" method to return "),
            code("true"),
            text(
                ", the component will re-render after all messages in the batch have been \
                 processed."
            ),
        ]),
        p(vec![text(
            "If the given vector is empty, this function does nothing."
        )]),
        h3(vec![text("callback")]),
        p(vec![
            text(
                "Create a callback that will send a message to the component when it is executed. \
                 Under the hood, it will call "
            ),
            code("send_message"),
            text(" with the message returned by the provided closure."),
        ]),
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
        // Create a callback that accepts some text and sends it
        // to the component as the `Msg::Text` message variant.
        // highlight-next-line
        let cb = ctx.link().callback(|text: String| Msg::Text(text));

        // The previous line is needlessly verbose to make it clearer.
        // It can be simplified it to this:
        // highlight-next-line
        let cb = ctx.link().callback(Msg::Text);

        // Will send `Msg::Text("Hello World!")` to the component.
        // highlight-next-line
        cb.emit("Hello World!".to_owned());

        html! {
            // html here
        }
    }
}"#
        ),
        h3(vec![text("batch_callback")]),
        p(vec![
            text(
                "Create a callback that will send a batch of messages to the component when it is \
                 executed. The difference to "
            ),
            code("callback"),
            text(
                " is that the closure passed to this method doesn't have to return a message. \
                 Instead, the closure can return either "
            ),
            code("Vec<Msg>"),
            text(" or "),
            code("Option<Msg>"),
            text(" where "),
            code("Msg"),
            text(" is the component's message type."),
        ]),
        p(vec![
            code("Vec<Msg>"),
            text(" is treated as a batch of messages and uses "),
            code("send_message_batch"),
            text(" under the hood."),
        ]),
        p(vec![
            code("Option<Msg>"),
            text(" calls "),
            code("send_message"),
            text(" if it is "),
            code("Some"),
            text(". If the value is "),
            code("None"),
            text(
                ", nothing happens. This can be used in cases where, depending on the situation, \
                 an update isn't required."
            ),
        ]),
        p(vec![
            text("This is achieved using the "),
            code("SendAsMessage"),
            text(" trait which is only implemented for these types. You can implement "),
            code("SendAsMessage"),
            text(" for your own types which allows you to use them in "),
            code("batch_callback"),
            text("."),
        ]),
    ])
);
