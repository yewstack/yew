crate::blog_page!(
    &crate::BLOG_POSTS[3],
    Content::new(vec![
        p![
            "The Yew team is happy to announce a new, long overdue, version of Yew: v0.20. Yew is \
             a framework for creating reliable and efficient web applications.",
        ],
        h2!["What's new"],
        p![
            "This release comes with new features aimed at improving the user experience, such as \
             server-rendering and render-as-you-fetch approach for data-fetching.",
        ],
        h3!["SSR"],
        p![
            "Yew now fully supports rendering on the server. Rendering on the server means users \
             will get a rendered HTML and will not have to wait to be able to see anything until \
             the entire WebAssembly bundle is downloaded and initial render has completed. With \
             SSR, the page will be visible instantly, and interactable as soon as hydration \
             finishes.",
        ],
        p![
            "Learn more at ",
            link![
                "/docs/advanced-topics/server-side-rendering",
                "Server-side rendering"
            ],
        ],
        h3!["Data fetching"],
        p![
            "With SSR comes new ways of data-fetching. The newly added ",
            link![
                "https://api.yew.rs/next/yew/functional/macro.use_prepared_state.html",
                code("use_prepared_state!"),
            ],
            " hook can be used to fetch data while rendering on the server and seamlessly use it \
             in the component.",
        ],
        p![
            "For client-side fetching, Yew now supports render-as-you-fetch approach with ",
            link!["/docs/concepts/suspense", "Suspense"],
            ".",
        ],
        h2!["How to upgrade"],
        p![
            "There have been breaking changes in this release. Our ",
            link![
                "/docs/migration-guides/yew/from-0_19_0-to-0_20_0",
                "migration guides"
            ],
            " go over how to upgrade each over of the new crates.",
        ],
        h2!["Thanks!"],
        p![
            "Many people came together to help make this release happen. We couldn't have done it \
             without all of you. Thanks!",
        ],
    ])
);
