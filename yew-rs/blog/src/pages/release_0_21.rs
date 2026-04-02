crate::blog_page!(
    &crate::BLOG_POSTS[2],
    Content::new(vec![
        p![
            "The Yew development team is thrilled to unveil Yew 0.21.0, a significant milestone \
             in the journey of empowering developers to create dependable and high-performance \
             web applications with Rust. Let's dive into the major highlights of this release.",
        ],
        h2!["What's new"],
        h3!["1. Changing Signatures: A Shift in Hook Dependencies"],
        p![
            "One of the significant changes in Yew 0.21 is the adjustment to the signature of \
             hooks that accept dependencies. Dependencies used to be passed as the second \
             argument after the closure. However, now they're passed as the first argument before \
             the closure.",
        ],
        code_block(
            "rust",
            r#"use_effect_with_deps(deps, move |deps: Vec<i32>| {
    // Do something with dependencies
});"#
        ),
        p![
            "The reason behind swapping the order of dependencies in the code snippet is to \
             address a specific use case. In situations where the same value is needed both to \
             compute a dependency and to be moved by value into the closure, the new order \
             simplifies the code and improves readability and ergonomics.",
        ],
        p![
            "This is a big breaking change so we've provided ",
            link![
                "https://yew.rs/docs/migration-guides/yew/from-0_20_0-to-0_21_0#automated-refactor",
                "a way to automate the refactor",
            ],
        ],
        h3!["2. Versatile Child Types"],
        p![
            "Yew now allows you to use any type as children within your components. This means \
             you're no longer limited to just the ",
            code("Children"),
            " type. Instead, you can use any type, even ",
            code("Html"),
            " or closures, unlocking patterns such as:",
        ],
        code_block(
            "rust",
            r##"html! {
    <Comp>
        {|p: RenderProps| html!{<>{"Hello, "}{p.name}</>}}
    </Comp>
}"##
        ),
        h3!["3. Agents: A Complete Rewrite"],
        p![
            "Yew 0.21 brings a complete rewrite of ",
            code("yew-agent"),
            ". This streamlines and simplifies the way workers operate. Here's what you need to \
             know:",
        ],
        ul![
            li![
                bold!["Introducing Providers:"],
                " Say goodbye to the old ",
                code("Worker::bridge()"),
                " method. Now, you can use the ",
                code("WorkerProvider"),
                " / ",
                code("ReactorProvider"),
                " / ",
                code("OneshotProvider"),
                " as per your need, by creating them using the hooks.",
            ],
            li![
                bold!["WorkerLink to WorkerScope:"],
                " We've removed WorkerLink in favor of WorkerScope. This change simplifies the \
                 worker architecture, making it more straightforward to manage and maintain.",
            ],
        ],
        p!["There are now 3 types of agents to be used, depending upon the situation:"],
        ul![
            li![
                bold!["Worker Agent:"],
                " The original agent that uses an actor model, designed to handle complex states.",
            ],
            li![
                bold!["Oneshot Agent:"],
                " Designed for scenarios where you expect a single input and a single output for \
                 each agent.",
            ],
            li![
                bold!["Reactor Agent:"],
                " Ideal for situations where multiple inputs and multiple outputs are needed for \
                 each agent.",
            ],
        ],
        p![
            "Learn more in ",
            link![
                "https://docs.rs/yew-agent/latest/yew_agent/",
                "documentation of yew-agent"
            ],
        ],
        h3!["4. Performance Improvements: A Faster and Smoother Experience"],
        p![
            "Yew 0.21 brings substantial performance improvements. Your web applications will run \
             faster and more efficiently, thanks to optimizations that reduce memory usage and \
             enhance rendering.",
        ],
        h2!["Call for Contributors"],
        p![
            "The Yew project thrives on community involvement, and we welcome contributors with \
             open arms. Whether you're an experienced Rust developer or just starting your \
             journey, there are plenty of ways to get involved and make a meaningful impact on \
             Yew's growth.",
        ],
        p!["Here are some areas where you can contribute:"],
        ul![
            li![
                bold!["Code Contributions:"],
                " If you're passionate about web development with Rust, consider contributing \
                 code to Yew. Whether it's fixing bugs, adding new features, or improving \
                 documentation, your code can help make Yew even better.",
            ],
            li![
                bold!["Documentation:"],
                " Clear and comprehensive documentation is vital for any project's success. You \
                 can contribute by improving documentation, writing tutorials, or creating \
                 examples that help others understand and use Yew effectively.",
            ],
            li![
                bold!["Testing and Bug Reporting:"],
                " Testing Yew and reporting bugs you encounter is a valuable contribution. Your \
                 feedback helps us identify and fix issues, ensuring a more stable framework for \
                 everyone.",
            ],
            li![
                bold!["Community Support:"],
                " Join discussions, chat rooms (we have our own Discord and Matrix!), or social \
                 media to assist other developers using Yew. Sharing your knowledge and helping \
                 others solve problems is a fantastic way to contribute.",
            ],
        ],
        p![
            "Contributing to open-source projects like Yew is not only a way to give back to the \
             community but also an excellent opportunity to learn, collaborate, and enhance your \
             skills.",
        ],
        p![
            "To get started, check out the Yew GitHub repository and the contribution guidelines. \
             Your contributions are highly appreciated and play a crucial role in shaping the \
             future of Yew. Join us in this exciting journey!",
        ],
        h2!["Thanks!"],
        p![
            "Many people came together to create Yew 0.21. We couldn't have done it without all \
             of you. Thanks!",
        ],
        p![
            "See ",
            link![
                "https://github.com/yewstack/yew/blob/master/CHANGELOG.md",
                "the full changelog"
            ],
        ],
    ])
);
