crate::doc_page!(
    "作用域",
    "/zh-Hant/docs/advanced-topics/struct-components/scope",
    Content::new(vec![
        h2![text("元件的 `Scope<_>` 接口")],
        p![
            code("Scope"),
            text(" 是透過訊息建立回呼並更新自己的機制。我們透過在傳遞給元件的上下文物件上呼叫 "),
            code("link()"),
            text(" 來獲得對它的參考。"),
        ],
        h3![text("`send_message`")],
        p![
            text("這個函數可以向元件發送訊息。訊息由 "),
            code("update"),
            text(" 方法處理，該方法決定元件是否應重新渲染。"),
        ],
        h3![text("`send_message_batch`")],
        p![
            text("這個函數可以同時向元件發送多個訊息。這類似於 "),
            code("send_message"),
            text("，但是如果任何訊息導致 "),
            code("update"),
            text(" 方法傳回 "),
            code("true"),
            text("，則元件將在處理批次中的所有訊息後重新渲染。"),
        ],
        p![text("如果給定的參數向量為空，則此函數不執行任何操作。"),],
        h3![text("`callback`")],
        p![
            text(
                "建立一個回調，當執行時將向元件發送訊息。在內部，它將使用提供的閉包返回的訊息呼叫 "
            ),
            code("send_message"),
            text("。"),
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
        // 建立一個接受一些文本並將其作為 `Msg::Text` 訊息變體發送到元件的回調。
        // highlight-next-line
        let cb = ctx.link().callback(|text: String| Msg::Text(text));

        // 上一行是多餘的冗長，為了更清晰，可以簡化為這樣：
        // highlight-next-line
        let cb = ctx.link().callback(Msg::Text);

        // 將 `Msg::Text("Hello World!")` 發送到元件。
        // highlight-next-line
        cb.emit("Hello World!".to_owned());

        html! {
            // 在這裡放置 HTML
        }
    }
}"#,
        ),
        h3![text("`batch_callback`")],
        p![
            text("建立一個回調，執行時將向元件發送一批訊息。與 "),
            code("callback"),
            text(" 的區別在於，傳遞給此方法的閉包不必傳回訊息。相反，閉包可以傳回 "),
            code("Vec<Msg>"),
            text(" 或 "),
            code("Option<Msg>"),
            text("，其中 "),
            code("Msg"),
            text(" 是元件的訊息類型。"),
        ],
        p![
            code("Vec<Msg>"),
            text(" 被視為一批訊息，並在內部使用 "),
            code("send_message_batch"),
            text("。"),
        ],
        p![
            code("Option<Msg>"),
            text(" 在值為 "),
            code("Some"),
            text(" 時呼叫 "),
            code("send_message"),
            text("。如果值為 "),
            code("None"),
            text("，則不執行任何操作。這可以用於根據情況，不需要更新的情況。"),
        ],
        p![
            text("這是透過使用僅為這些類型實現的 "),
            code("SendAsMessage"),
            text(" trait 來實現的。您可以為自己的類型實作 "),
            code("SendAsMessage"),
            text("，這樣可以在 "),
            code("batch_callback"),
            text(" 中使用它們。"),
        ],
    ])
);
