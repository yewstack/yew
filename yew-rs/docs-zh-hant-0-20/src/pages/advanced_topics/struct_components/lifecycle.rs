crate::doc_page!(
    "元件",
    "/zh-Hant/docs/advanced-topics/struct-components/lifecycle",
    Content::new(vec![
        h2![text("什麼是元件？")],
        p![
            text(
                "元件是 Yew 的基石。他們管理自己的狀態，可以渲染自己成為 \
                 DOM。元件可以透過實作，描述元件生命周期的 ",
            ),
            code("Component"),
            text(" trait 來建立。"),
        ],
        h2![text("生命周期")],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("歡迎來貢獻我們的文件："),
                text(" "),
                link![
                    "https://github.com/yewstack/docs/issues/22",
                    text("Add a diagram of the component lifecycle"),
                ],
            ],
        ],
        h2![text("生命周期的方法")],
        h3![text("Create")],
        p![
            text("當一個元件被建立，他會接收從父元件，也就是 "),
            code("ComponentLink"),
            text(
                " ，傳下來的屬性。 這些屬性用來初始化元件的狀態，此外，\
                 「link」可以用來註冊回調函式或傳訊息給元件。"
            ),
        ],
        p![text(
            "通常，你的元件 struct 會儲存 props 與 link，就像下面的例子：",
        )],
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
}"#,
        ),
        h3![text("View")],
        p![
            text("元件會在 "),
            code("view()"),
            text(" 方法中宣告佈局。Yew 提供 "),
            code("html!"),
            text(
                " 巨集來宣告 HTML 合 SVG 的結點，包含他們的監聽事件與子結點。這個巨集扮演像是 \
                 React 的 JSX 的角色，但是是使用 Rust 的表達式，而不是 JavaScript 的。",
            ),
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
}"#,
        ),
        p![
            text("更多使用細節，請參考 "),
            link!["/zh-Hant/docs/concepts/html", text("html! 教學"),],
            text("。"),
        ],
        h3![text("Rendered")],
        p![
            code("rendered()"),
            text(" 生命周期的方法會，在 "),
            code("view()"),
            text(
                " 處理完並且 Yew \
                 渲染完你的元件之後，與瀏覽器刷新頁面之前，被呼叫。一個元件可能希望實作這個方法，\
                 去執行只能在元件被渲染完元素才能做的事情。 你可以透過 ",
            ),
            code("first_render"),
            text(" 變數來確認這個元件是不是第一次被渲染。"),
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
}"#,
        ),
        admonition![
            AdmonitionType::Note,
            None,
            p![text(
                "注意，這個生命周期方法，不是一定要被實作，預設的行為是不做任何事情。",
            )],
        ],
        h3![text("Update")],
        p![
            text("元件是可動態更新且可以註冊接收非同步的訊息。 "),
            code("update()"),
            text(
                " 生命周期方法會被每個訊息呼叫。他基於訊息是什麼，來允許元件更新自己，\
                 且會決定是否需要重新渲染。 訊息可以被 HTML \
                 元素的監聽器觸發，或被子元件、Agents、Services 或 Futures 傳送。",
            ),
        ],
        p![code("update()"), text(" 應用範例："),],
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

}"#,
        ),
        h3![text("Change")],
        p![text(
            "元件可能會被他的父元件重新渲染。當他被父元件重新渲染時，他會收到新的屬性，\
             然後決定要不要再渲染一次。 這設計是讓父元件透過便於跟子元件溝通。",
        )],
        p![text("一個簡單的實作方式像：")],
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
}"#,
        ),
        h3![text("Destroy")],
        p![
            text("當元件從 DOM 上被解除掛載，Yew 會呼叫 "),
            code("destroy()"),
            text(
                " 生命周期方法以提供任何需要清理的操作。這個方法是不一定要被實作的，\
                 預設不會做設任何事。",
            ),
        ],
        h2![text("相關的型別")],
        p![
            code("Component"),
            text(" trait 有兩個相關的型別："),
            code("Message"),
            text(" 與 "),
            code("Properties"),
            text("。"),
        ],
        code_block(
            "rust",
            r#"impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}"#,
        ),
        p![
            code("Message"),
            text(" 負責各式各樣的訊息，他可能被元件處理去觸發各種影響。舉例來說，你可能有一個 ",),
            code("Click"),
            text(
                " 的訊息，他會觸發 API 請求，或是切換 UI \
                 元件的樣貌。下面是一個常見的實作，在你的元件模組中，創建一個叫作 ",
            ),
            code("Msg"),
            text(" 的 enum，然後把他當作元件裡的 Message 型別。通常 message 會縮寫成 msg。",),
        ],
        code_block(
            "rust",
            r#"enum Msg {
    Click,
    FormInput(String)
}"#,
        ),
        p![
            code("Properties"),
            text(" 代表要從父員件傳遞到子元件的資訊。這個型別必須實作 ",),
            code("Properties"),
            text(
                " trait （通常會 deriving \
                 他）並且可以決定某個屬性是必要的屬性，或是可選的屬性。\
                 這個型別會在創建元件的時候，或是更新元件的時候被使用到。\
                 常見的實作會在你的元件模組中，建立一個叫作 ",
            ),
            code("Props"),
            text(" 的 struct，然後把他當作元件的"),
            code("Properties"),
            text(
                " 型別。通常 properties 或縮寫成 \
                 props。因為屬性是從父原件被傳下來的，所以應用程式中的根元件的 ",
            ),
            code("Properties"),
            text(" 原則上都是 "),
            code("()"),
            text("。如果你希望你的根元件有特定的屬性，可以使用 "),
            code("App::mount_with_props"),
            text(" 的方法。"),
        ],
    ])
);
