pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("コールバック関数 (Callbacks)")],
        p![
            text(
                "コールバック関数は、Yew \
                 でサービス、エージェント、および親コンポーネントと通信するために使用されます。\
                 内部的には、それらの型は ",
            ),
            code("Rc"),
            text(" に包まれた "),
            code("Fn"),
            text(" に過ぎず、クローンを許可します。"),
        ],
        p![
            text("それらには "),
            code("emit"),
            text(" 関数があり、その "),
            code("<IN>"),
            text(
                " 型を引数として取り、それをターゲットが期待するメッセージに変換します。\
                 親コンポーネントのコールバック関数が子コンポーネントに props \
                 として提供される場合、子コンポーネントはその ",
            ),
            code("update"),
            text(" ライフサイクルフックでコールバック関数の "),
            code("emit"),
            text(" 関数を呼び出して、メッセージを親コンポーネントに送信できます。"),
            code("html!"),
            text(
                " マクロで props \
                 として提供されるクロージャまたは関数は、自動的にコールバック関数に変換されます。",
            ),
        ],
        p![text(
            "シンプルなコールバック関数の使用例は次のようになります：",
        )],
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
            text("この関数を "),
            code("callback"),
            text(" に渡す場合、常に1つの引数を持つ必要があります。例えば、"),
            code("onclick"),
            text(" ハンドラは "),
            code("MouseEvent"),
            text(
                " 型の引数を受け取る関数である必要があります。その後、\
                 ハンドラはコンポーネントにどのタイプのメッセージを送信するかを決定できます。\
                 このメッセージは無条件に次の更新サイクルにスケジュールされます。",
            ),
        ],
        p![
            text("更新を引き起こす必要がないコールバック関数が必要な場合は、"),
            code("batch_callback"),
            text(" を使用してください。"),
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
        h2![text("関連例")],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/counter",
                text("Counter"),
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/timer",
                text("Timer"),
            )],
        ],
    ])
}

crate::doc_page!(
    "コールバック関数 (Callbacks)",
    "/ja/docs/advanced-topics/struct-components/callbacks",
    page_content()
);
