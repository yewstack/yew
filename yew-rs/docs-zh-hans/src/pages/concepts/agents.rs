pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["代理 (Agents) 是一种将任务卸载到 Web Workers 的方式。"],
        p![
            "为了使代理能够并发运行，Yew 使用了 ",
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers",
                "Web Workers",
            ),
            "。",
        ],
        h2!["生命周期"],
        themed_img("/img/agent-lifecycle-light.svg", "/img/agent-lifecycle-dark.svg", "agent lifecycle diagram"),
        h2!["代理的类型"],
        h3!["范围"],
        ul![
            li!["公开 - 在任何给定时间，公共代理的实例最多只有一个。桥梁将在 Web Worker \
                 中生成或连接到已经生成的代理。当没有桥梁连接到此代理时，代理将消失。"],
            li!["私有 - 为每个新的桥梁在 Web Worker \
                 中生成一个新的代理。这对于将与浏览器通信的共享但独立的行为从组件中移出是很好的。当连接的桥梁被丢弃时，代理将消失。"],
            li!["全局 (WIP)"],
        ],
        h2!["代理与组件之间的通信"],
        h3!["通信桥 (Bridges)"],
        p!["通信桥 (bridge) \
             是一个组件和代理之间的通信通道。它允许组件向代理发送消息，并接收来自代理的消息。"],
        p![
            code("use_bridge"),
            " 钩子也提供了在函数组件中创建桥梁的功能。",
        ],
        h3!["派发器 (Dispatchers)"],
        p!["派发器 (Dispatchers) \
             允许组件和代理之间进行单向通信，组件以此方式向代理发送消息。"],
        h2!["开销"],
        p![
            "代理使用 Web \
                 Workers（即私有和公开）。它们在发送和接收消息时会产生序列化开销。代理使用 ",
            link!(
                "https://github.com/bincode-org/bincode",
                "bincode",
            ),
            " 与其他线程通信，因此成本比仅调用函数要高得多。",
        ],
        h2!["进一步阅读"],
        ul![li![
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/web_worker_fib",
                "web_worker_fib",
            ),
            " 示例展示了组件如何向代理发送消息并接收来自代理的消息。",
        ]],
    ])
}

crate::doc_page!(
    "代理 (Agents)",
    "/zh-Hans/docs/concepts/agents",
    page_content()
);
