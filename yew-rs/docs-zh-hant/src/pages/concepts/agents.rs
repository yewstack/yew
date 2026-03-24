pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("代理程式 (Agents) 是一種將任務卸載到 Web Workers 的方式。"),
        ],
        p![
            text("為了使代理程式能夠並發運行，Yew 使用了 "),
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers",
                text("Web Workers"),
            ),
            text("。"),
        ],
        h2![text("生命週期")],
        themed_img("/img/agent-lifecycle-light.svg", "/img/agent-lifecycle-dark.svg", "agent lifecycle diagram"),
        h2![text("代理程式的類型")],
        h3![text("範圍")],
        ul![
            li![text(
                "公開 - 在任何給定時間，公共代理的實例最多只有一個。橋樑將在 Web Worker \
                 中產生或連接到已經產生的代理程式。當沒有橋樑連接到此代理時，代理將消失。",
            )],
            li![text(
                "私有 - 為每個新的橋樑在 Web Worker \
                 中產生一個新的代理程式。這對於將與瀏覽器通訊的共享但獨立的行為從元件中移出是很好的。當連接的橋樑被丟棄時，代理將消失。",
            )],
            li![text("全域 (WIP)")],
        ],
        h2![text("代理與元件之間的通信")],
        h3![text("通信橋 (Bridges)")],
        p![text(
            "通訊橋 (bridge) \
             是一個元件和代理程式之間的通訊通道。它允許元件向代理發送訊息，並接收來自代理的訊息。",
        )],
        p![
            code("use_bridge"),
            text(" 鉤子也提供了在函數元件中建立橋樑的功能。"),
        ],
        h3![text("派發器 (Dispatchers)")],
        p![text(
            "派發器 (Dispatchers) \
             允許元件和代理程式之間進行單向通信，元件以此方式向代理程式發送訊息。",
        )],
        h2![text("開銷")],
        p![
            text(
                "代理程式使用 Web Workers（即私有和公開）。它們在發送和接收訊息時會產生序列化開銷。代理程式使用 ",
            ),
            link!(
                "https://github.com/bincode-org/bincode",
                text("bincode"),
            ),
            text(" 與其他執行緒通信，因此成本比僅呼叫函數要高得多。"),
        ],
        h2![text("進一步閱讀")],
        ul![li![
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/web_worker_fib",
                text("web_worker_fib"),
            ),
            text(" 範例展示了元件如何向代理程式傳送訊息並接收來自代理程式的訊息。"),
        ]],
    ])
}

crate::doc_page!(
    "代理 (Agents)",
    "/zh-Hant/docs/concepts/agents",
    page_content()
);
