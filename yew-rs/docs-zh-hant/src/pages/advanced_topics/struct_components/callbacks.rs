pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["回呼函數 (Callbacks)"],
        p![
            "回調函數是用於在 Yew 中與服務、代理和父元件進行通訊的。在內部，它們的類型只是 ",
            code("Fn"),
            " 包裝在 ",
            code("Rc"),
            " 中，以允許它們被克隆。",
        ],
        p![
            "它們有一個 ",
            code("emit"),
            " 函數，該函數以其 ",
            code("<IN>"),
            " 類型作為參數，並將其轉換為其目標期望的訊息。如果父元件中的回呼函數作為 props \
             提供給子元件，子元件可以在其 ",
            code("update"),
            " 生命週期鉤子中呼叫回呼函數的 ",
            code("emit"),
            " 函數，以將訊息傳回其父元件。在 ",
            code("html!"),
            " 巨集中作為 props 提供的閉包或函數會自動轉換為回呼函數。",
        ],
        p!["一個簡單的回呼函數的使用可能如下所示："],
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
            "這個函數傳遞給 ",
            code("callback"),
            " 必須永遠帶有一個參數。例如，",
            code("onclick"),
            " 處理程序需要一個接受 ",
            code("MouseEvent"),
            " 類型參數的函數。然後處理程序可以決定應該發送什麼類型的消息給組件。\
             這個訊息無條件地被安排在下一個更新循環中。",
        ],
        p![
            "如果你需要一個回呼函數，它可能不需要引起更新，請使用 ",
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
        h2!["相關範例"],
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
    "回呼函數 (Callbacks)",
    "/zh-Hant/docs/advanced-topics/struct-components/callbacks",
    page_content()
);
