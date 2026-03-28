crate::doc_page!("Agents", "/ja/docs/concepts/agents",
    Content::new(vec![
        p!["Agents are a way to offload tasks to web workers."],
        p![
            "In order for agents to run concurrently, Yew uses ",
            link!["https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers", "web-workers"],
            ".",
        ],
        h2!["Lifecycle"],
        img("/img/agent-lifecycle-light.svg", "agent lifecycle diagram"),
        h2!["Types of Agents"],
        h3!["Reaches"],
        ul![
            li![
                "Public - There will exist at most one instance of a Public Agent at any given time. Bridges will \
                  spawn or connect to an already spawned agent in a web worker. \
                  When no bridges are connected to this agent, the agent will disappear.",
            ],
            li![
                "Private - Spawn a new agent in a web worker for every new bridge. This is good for moving shared but \
                  independent behavior that communicates with the browser out of components. When \
                  the connected bridge is dropped, the agent will disappear.",
            ],
            li!["Global (WIP)"],
        ],
        h2!["Communication between Agents and Components"],
        h3!["Bridges"],
        p![
            "A bridge allows bi-directional communication between an agent and a component. \
              Bridges also allow agents to communicate with one another.",
        ],
        p![
            "A ",
            code("use_bridge"),
            " hook is also provided to create bridges in a function component.",
        ],
        h3!["Dispatchers"],
        p![
            "A dispatcher allows uni-directional communication between a component and an agent. \
              A dispatcher allows a component to send messages to an agent.",
        ],
        h2!["Overhead"],
        p![
            "Agents use web workers (i.e. Private and Public). They incur a serialization overhead on the \
              messages they send and receive. Agents use ",
            link!["https://github.com/servo/bincode", "bincode"],
            " to communicate with other threads, so the cost is substantially higher than just calling a function.",
        ],
        h2!["Further reading"],
        ul![
            li![
                "The ",
                link!["https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/web_worker_fib", "web_worker_fib"],
                " example shows how components can send message to and receive message from agents.",
            ],
        ],
    ])
    .with_description("Yew のエージェントシステム")
);
