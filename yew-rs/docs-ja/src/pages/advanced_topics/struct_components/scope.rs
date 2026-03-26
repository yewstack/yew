pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["コンポーネントの `Scope<_>` インターフェース"],
        p![
            code("Scope"),
            " は、メッセージを介してコールバックを作成し、自身を更新するメカニズムです。コンポーネントに渡されるコンテキストオブジェクトで ",
            code("link()"),
            " を呼び出すことで、その参照を取得します。",
        ],
        h3!["send_message"],
        p![
            "この関数は、コンポーネントにメッセージを送信できます。メッセージは ",
            code("update"),
            " メソッドによって処理され、コンポーネントが再レンダリングするかどうかを決定します。",
        ],
        h3!["send_message_batch"],
        p![
            "この関数は、コンポーネントに複数のメッセージを同時に送信できます。これは ",
            code("send_message"),
            " に似ていますが、任意のメッセージが ",
            code("update"),
            " メソッドで ",
            code("true"),
            " を返す場合、バッチ内のすべてのメッセージの処理が完了した後にコンポーネントが再レンダリングされます。",
        ],
        p!["指定された引数ベクターが空の場合、この関数は何も実行しません。"],
        h3!["callback"],
        p![
            "コールバックを作成し、実行時にコンポーネントにメッセージを送信します。内部的には、提供されたクロージャが返すメッセージを使用して ",
            code("send_message"),
            " を呼び出します。",
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
        h3!["batch_callback"],
        p![
            "バッチメッセージを送信するコールバックを作成します。このメソッドに渡されるクロージャはメッセージを返す必要はありません。代わりに、クロージャは ",
            code("Vec<Msg>"),
            " または ",
            code("Option<Msg>"),
            " を返すことができます。ここで、",
            code("Msg"),
            " はコンポーネントのメッセージタイプです。",
        ],
        p![
            code("Vec<Msg>"),
            " はバッチメッセージとして扱われ、内部的に ",
            code("send_message_batch"),
            " を使用します。",
        ],
        p![
            code("Option<Msg>"),
            " は値が ",
            code("Some"),
            " の場合に ",
            code("send_message"),
            " を呼び出します。値が ",
            code("None"),
            " の場合は何も実行しません。これは、更新が不要な場合に使用できます。",
        ],
        p![
            "これは、これらの型に対してのみ実装された ",
            code("SendAsMessage"),
            " トレイトを使用して実現されています。独自の型に対して ",
            code("SendAsMessage"),
            " を実装することで、",
            code("batch_callback"),
            " でそれらを使用できるようになります。",
        ],
    ])
}

crate::doc_page!(
    "スコープ",
    "/ja/docs/advanced-topics/struct-components/scope",
    page_content()
);
