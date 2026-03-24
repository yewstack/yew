pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text("Agents are a way to offload tasks to web workers.")],
        p![
            text("In order for agents to run concurrently, Yew uses "),
            link!("https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers", text("web-workers")),
            text("."),
        ],
        h2![text("Lifecycle")],
        themed_img("/img/agent-lifecycle-light.svg", "/img/agent-lifecycle-dark.svg", "agent lifecycle diagram"),
        h2![text("Types of Agents")],
        h3![text("Reaches")],
        ul![
            li![text("Public - There will exist at most one instance of a Public Agent at any given time. Bridges will spawn or connect to an already spawned agent in a web worker. When no bridges are connected to this agent, the agent will disappear.")],
            li![text("Private - Spawn a new agent in a web worker for every new bridge. This is good for moving shared but independent behavior that communicates with the browser out of components. When the connected bridge is dropped, the agent will disappear.")],
            li![text("Global (WIP)")],
        ],
        h2![text("Communication between Agents and Components")],
        h3![text("Bridges")],
        p![text("A bridge allows bi-directional communication between an agent and a component. Bridges also allow agents to communicate with one another.")],
        p![
            text("A "),
            code("use_bridge"),
            text(" hook is also provided to create bridges in a function component."),
        ],
        h3![text("Dispatchers")],
        p![text("A dispatcher allows uni-directional communication between a component and an agent. A dispatcher allows a component to send messages to an agent.")],
        h2![text("Overhead")],
        p![
            text("Agents use web workers (i.e. Private and Public). They incur a serialization overhead on the messages they send and receive. Agents use "),
            link!("https://github.com/bincode-org/bincode", text("bincode")),
            text(" to communicate with other threads, so the cost is substantially higher than just calling a function."),
        ],
        h2![text("Further reading")],
        ul![
            li![
                text("The "),
                link!("https://github.com/yewstack/yew/tree/master/examples/web_worker_fib", text("web_worker_fib")),
                text(" example shows how components can send messages to and receive messages from agents."),
            ],
        ],
    ])
}

crate::doc_page!("Agents", "/docs/concepts/agents", page_content());
