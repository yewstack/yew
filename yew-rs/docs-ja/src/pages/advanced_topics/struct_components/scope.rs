pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("コンポーネントの `Scope<_>` インターフェース")],
        p![
            code("Scope"),
            text(" は、メッセージを介してコールバックを作成し、自身を更新するメカニズムです。コンポーネントに渡されるコンテキストオブジェクトで "),
            code("link()"),
            text(" を呼び出すことで、その参照を取得します。"),
        ],
        h3![text("send_message")],
        p![
            text("この関数は、コンポーネントにメッセージを送信できます。メッセージは "),
            code("update"),
            text(" メソッドによって処理され、コンポーネントが再レンダリングするかどうかを決定します。"),
        ],
        h3![text("send_message_batch")],
        p![
            text("この関数は、コンポーネントに複数のメッセージを同時に送信できます。これは "),
            code("send_message"),
            text(" に似ていますが、任意のメッセージが "),
            code("update"),
            text(" メソッドで "),
            code("true"),
            text(" を返す場合、バッチ内のすべてのメッセージの処理が完了した後にコンポーネントが再レンダリングされます。"),
        ],
        p![text("指定された引数ベクターが空の場合、この関数は何も実行しません。")],
        h3![text("callback")],
        p![
            text("コールバックを作成し、実行時にコンポーネントにメッセージを送信します。内部的には、提供されたクロージャが返すメッセージを使用して "),
            code("send_message"),
            text(" を呼び出します。"),
        ],
        code_block("rust", r#"use yew::{html, Component, Context, Html};

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
        // テキストを受け取り、それを `Msg::Text` メッセージバリアントとしてコンポーネントに送信するコールバックを作成します。
        // highlight-next-line
        let cb = ctx.link().callback(|text: String| Msg::Text(text));

        // 上の行は冗長であり、より明確にするために次のように簡略化できます：
        // highlight-next-line
        let cb = ctx.link().callback(Msg::Text);

        // `Msg::Text("Hello World!")` をコンポーネントに送信します。
        // highlight-next-line
        cb.emit("Hello World!".to_owned());

        html! {
            // ここに HTML を配置
        }
    }
}"#),
        h3![text("batch_callback")],
        p![
            text("バッチメッセージを送信するコールバックを作成します。このメソッドに渡されるクロージャはメッセージを返す必要はありません。代わりに、クロージャは "),
            code("Vec<Msg>"),
            text(" または "),
            code("Option<Msg>"),
            text(" を返すことができます。ここで、"),
            code("Msg"),
            text(" はコンポーネントのメッセージタイプです。"),
        ],
        p![
            code("Vec<Msg>"),
            text(" はバッチメッセージとして扱われ、内部的に "),
            code("send_message_batch"),
            text(" を使用します。"),
        ],
        p![
            code("Option<Msg>"),
            text(" は値が "),
            code("Some"),
            text(" の場合に "),
            code("send_message"),
            text(" を呼び出します。値が "),
            code("None"),
            text(" の場合は何も実行しません。これは、更新が不要な場合に使用できます。"),
        ],
        p![
            text("これは、これらの型に対してのみ実装された "),
            code("SendAsMessage"),
            text(" トレイトを使用して実現されています。独自の型に対して "),
            code("SendAsMessage"),
            text(" を実装することで、"),
            code("batch_callback"),
            text(" でそれらを使用できるようになります。"),
        ],
    ])
}

crate::doc_page!(
    "スコープ",
    "/ja/docs/advanced-topics/struct-components/scope",
    page_content()
);
