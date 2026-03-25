crate::doc_page!(
    "Agents",
    "/zh-Hans/docs/concepts/agents",
    Content::new(vec![
        p![
            text("Agents 和 Angular 的 "),
            link!["https://angular.io/guide/architecture-services", text("Services")],
            text(" 相似（但没有依赖注入），给 Yew 提供了 "),
            link!["https://en.wikipedia.org/wiki/Actor_model", text("Actor 模型")],
            text("。Agents 可以用于在组件之间路由消息，而与它们在组件层次结构中的位置无关，或者可以用于协调全局状态，或者可以用于从主 UI 线程上卸载计算密集型任务，或者在不同的标签页间通信（在未来）。"),
        ],
        p![
            text("Agents 使用 "),
            link!["https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers", text("web-workers")],
            text(" 同时运行来实现并发。"),
        ],
        h2![text("生命周期")],
        img(
            "https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png",
            "Agent lifecycle",
        ),
        h2![text("Agents 的类型")],
        h4![text("Reaches")],
        ul![
            li![text("Job - 在 UI 线程上为每个新的 Bridge 生成一个新的 Agent。这对于将与浏览器通信的共享但独立的行为移出组件是很有用的。（待验证）任务完成后，Agent 将消失。")],
            li![text("Context - Bridges 将生成或连接到 UI 线程上的 agent。这可用于在组件和其它 Agents 之间协调状态。当没有 Bridge 连接到该 Agent 时，Agent 将消失。")],
            li![text("Private - 与 Job 相同，但运行在自己的 web worker 中。")],
            li![text("Public - 与 Context 相同，但运行在自己的 web worker 中。")],
            li![text("Global \\(WIP\\)")],
        ],
        h2![text("Agent 通信")],
        h3![text("Bridges")],
        p![text("Bridges 将连接到一个 Agent 并且允许双向通信。")],
        h3![text("Dispatchers")],
        p![text("Dispatchers 和 Bridges 类似，但是他们只能发送消息给 Agents。")],
        h2![text("开销")],
        p![text("Agents 通过使用二进制码 bincode 序列化其消息来进行通信。因此，存在比仅调用函数相比更高的性能消耗。除非计算成本或者在任意组件间协调的需求超过消息传递的成本，否则你应该尽可能地在函数中包含你的应用逻辑。")],
        h2![text("Further reading")],
        ul![
            li![
                text("The "),
                link!["https://github.com/yewstack/yew/tree/master/examples/web_worker_fib", text("web_worker_fib")],
                text(" example shows how components can use agents to communicate with each other."),
            ],
        ],
    ])
);
