crate::blog_page!(
    &crate::BLOG_POSTS[1],
    Content::new(vec![
        h2!["What's new"],
        h3!["SSR on WASI"],
        p![
            "Before Yew 0.22, server-side rendering (SSR) was only possible on the native target. \
             With Yew 0.22, you can now render your Yew application on the WebAssembly System \
             Interface (WASI) target.",
        ],
        p![
            "Since the old SSR implementation will create new tasks directly in the asynchronous \
             context directly (based on ",
            code("prokio"),
            "). It only allowed in a multi-threaded environment that it is not compatible with \
             WASI. So the new version added a dedicated one for a single-threaded environment \
             that rendering function to support single-threaded scenes.",
        ],
        p![
            "Learn more at ",
            link![
                "https://github.com/yewstack/yew/tree/master/examples/wasi_ssr_module/src/main.rs",
                "Server-side rendering example on WASI environment",
            ],
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
            "Many people came together to create Yew 0.22. We couldn't have done it without all \
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
