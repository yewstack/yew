use yew_site_lib::{migration_guides_sidebar, SidebarCategory, SidebarEntry, SidebarItem};

pub fn docs_sidebar() -> Vec<SidebarEntry> {
    vec![
        SidebarEntry::Category(SidebarCategory {
            label: "Getting Started",
            link: Some("/zh-Hant/docs/getting-started"),
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Build a Sample App",
                    href: "/zh-Hant/docs/getting-started/build-a-sample-app",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Examples",
                    href: "/zh-Hant/docs/getting-started/examples",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Editor Setup",
                    href: "/zh-Hant/docs/getting-started/editor-setup",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "Concepts",
            link: None,
            items: vec![
                SidebarEntry::Category(SidebarCategory {
                    label: "Basic Web Technologies",
                    link: None,
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "HTML",
                            href: "/zh-Hant/docs/concepts/basic-web-technologies/html",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "CSS",
                            href: "/zh-Hant/docs/concepts/basic-web-technologies/css",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "JavaScript",
                            href: "/zh-Hant/docs/concepts/basic-web-technologies/js",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "wasm-bindgen",
                            href: "/zh-Hant/docs/concepts/basic-web-technologies/wasm-bindgen",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "web-sys",
                            href: "/zh-Hant/docs/concepts/basic-web-technologies/web-sys",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Components",
                    link: Some("/zh-Hant/docs/concepts/function-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/zh-Hant/docs/concepts/function-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/zh-Hant/docs/concepts/function-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Children",
                            href: "/zh-Hant/docs/concepts/function-components/children",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Pure Components",
                            href: "/zh-Hant/docs/concepts/function-components/pure-components",
                        }),
                        SidebarEntry::Category(SidebarCategory {
                            label: "Hooks",
                            link: Some("/zh-Hant/docs/concepts/function-components/hooks"),
                            items: vec![SidebarEntry::Item(SidebarItem {
                                label: "Custom Hooks",
                                href: "/zh-Hant/docs/concepts/function-components/hooks/\
                                       custom-hooks",
                            })],
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Node Refs",
                            href: "/zh-Hant/docs/concepts/function-components/node-refs",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "State",
                            href: "/zh-Hant/docs/concepts/function-components/state",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Communication",
                            href: "/zh-Hant/docs/concepts/function-components/communication",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Generics",
                            href: "/zh-Hant/docs/concepts/function-components/generics",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "HTML",
                    link: Some("/zh-Hant/docs/concepts/html"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Components",
                            href: "/zh-Hant/docs/concepts/html/components",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Elements",
                            href: "/zh-Hant/docs/concepts/html/elements",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Events",
                            href: "/zh-Hant/docs/concepts/html/events",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Classes",
                            href: "/zh-Hant/docs/concepts/html/classes",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Fragments",
                            href: "/zh-Hant/docs/concepts/html/fragments",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lists",
                            href: "/zh-Hant/docs/concepts/html/lists",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Literals and Expressions",
                            href: "/zh-Hant/docs/concepts/html/literals-and-expressions",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Conditional Rendering",
                            href: "/zh-Hant/docs/concepts/html/conditional-rendering",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Agents",
                    href: "/zh-Hant/docs/concepts/agents",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Contexts",
                    href: "/zh-Hant/docs/concepts/contexts",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Router",
                    href: "/zh-Hant/docs/concepts/router",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Suspense",
                    href: "/zh-Hant/docs/concepts/suspense",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "Advanced Topics",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "How It Works",
                    href: "/zh-Hant/docs/advanced-topics/how-it-works",
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Struct Components",
                    link: Some("/zh-Hant/docs/advanced-topics/struct-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Higher Order Components",
                            href: "/zh-Hant/docs/advanced-topics/struct-components/hoc",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lifecycle",
                            href: "/zh-Hant/docs/advanced-topics/struct-components/lifecycle",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Scope",
                            href: "/zh-Hant/docs/advanced-topics/struct-components/scope",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/zh-Hant/docs/advanced-topics/struct-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/zh-Hant/docs/advanced-topics/struct-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Refs",
                            href: "/zh-Hant/docs/advanced-topics/struct-components/refs",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Children",
                    href: "/zh-Hant/docs/advanced-topics/children",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Optimizations",
                    href: "/zh-Hant/docs/advanced-topics/optimizations",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Portals",
                    href: "/zh-Hant/docs/advanced-topics/portals",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Server-Side Rendering",
                    href: "/zh-Hant/docs/advanced-topics/server-side-rendering",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Immutable",
                    href: "/zh-Hant/docs/advanced-topics/immutable",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "More",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Debugging",
                    href: "/zh-Hant/docs/more/debugging",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Deployment",
                    href: "/zh-Hant/docs/more/deployment",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "CSS",
                    href: "/zh-Hant/docs/more/css",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Testing",
                    href: "/zh-Hant/docs/more/testing",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Roadmap",
                    href: "/zh-Hant/docs/more/roadmap",
                }),
            ],
        }),
        migration_guides_sidebar(),
    ]
}
