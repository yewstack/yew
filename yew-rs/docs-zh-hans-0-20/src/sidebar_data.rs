use yew_site_lib::{migration_guides_sidebar, SidebarCategory, SidebarEntry, SidebarItem};

pub fn docs_sidebar() -> Vec<SidebarEntry> {
    vec![
        SidebarEntry::Category(SidebarCategory {
            label: "Getting Started",
            link: Some("/zh-Hans/docs/getting-started"),
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Build a Sample App",
                    href: "/zh-Hans/docs/getting-started/build-a-sample-app",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Examples",
                    href: "/zh-Hans/docs/getting-started/examples",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Editor Setup",
                    href: "/zh-Hans/docs/getting-started/editor-setup",
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
                            href: "/zh-Hans/docs/concepts/basic-web-technologies/html",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "CSS",
                            href: "/zh-Hans/docs/concepts/basic-web-technologies/css",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "JavaScript",
                            href: "/zh-Hans/docs/concepts/basic-web-technologies/js",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "wasm-bindgen",
                            href: "/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "web-sys",
                            href: "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Components",
                    link: Some("/zh-Hans/docs/concepts/function-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/zh-Hans/docs/concepts/function-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/zh-Hans/docs/concepts/function-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Children",
                            href: "/zh-Hans/docs/concepts/function-components/children",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Pure Components",
                            href: "/zh-Hans/docs/concepts/function-components/pure-components",
                        }),
                        SidebarEntry::Category(SidebarCategory {
                            label: "Hooks",
                            link: Some("/zh-Hans/docs/concepts/function-components/hooks"),
                            items: vec![SidebarEntry::Item(SidebarItem {
                                label: "Custom Hooks",
                                href: "/zh-Hans/docs/concepts/function-components/hooks/\
                                       custom-hooks",
                            })],
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Node Refs",
                            href: "/zh-Hans/docs/concepts/function-components/node-refs",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "State",
                            href: "/zh-Hans/docs/concepts/function-components/state",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Communication",
                            href: "/zh-Hans/docs/concepts/function-components/communication",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Generics",
                            href: "/zh-Hans/docs/concepts/function-components/generics",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "HTML",
                    link: Some("/zh-Hans/docs/concepts/html"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Components",
                            href: "/zh-Hans/docs/concepts/html/components",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Elements",
                            href: "/zh-Hans/docs/concepts/html/elements",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Events",
                            href: "/zh-Hans/docs/concepts/html/events",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Classes",
                            href: "/zh-Hans/docs/concepts/html/classes",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Fragments",
                            href: "/zh-Hans/docs/concepts/html/fragments",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lists",
                            href: "/zh-Hans/docs/concepts/html/lists",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Literals and Expressions",
                            href: "/zh-Hans/docs/concepts/html/literals-and-expressions",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Conditional Rendering",
                            href: "/zh-Hans/docs/concepts/html/conditional-rendering",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Agents",
                    href: "/zh-Hans/docs/concepts/agents",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Contexts",
                    href: "/zh-Hans/docs/concepts/contexts",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Router",
                    href: "/zh-Hans/docs/concepts/router",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Suspense",
                    href: "/zh-Hans/docs/concepts/suspense",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "Advanced Topics",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "How It Works",
                    href: "/zh-Hans/docs/advanced-topics/how-it-works",
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Struct Components",
                    link: Some("/zh-Hans/docs/advanced-topics/struct-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Higher Order Components",
                            href: "/zh-Hans/docs/advanced-topics/struct-components/hoc",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lifecycle",
                            href: "/zh-Hans/docs/advanced-topics/struct-components/lifecycle",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Scope",
                            href: "/zh-Hans/docs/advanced-topics/struct-components/scope",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/zh-Hans/docs/advanced-topics/struct-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/zh-Hans/docs/advanced-topics/struct-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Refs",
                            href: "/zh-Hans/docs/advanced-topics/struct-components/refs",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Children",
                    href: "/zh-Hans/docs/advanced-topics/children",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Optimizations",
                    href: "/zh-Hans/docs/advanced-topics/optimizations",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Portals",
                    href: "/zh-Hans/docs/advanced-topics/portals",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Server-Side Rendering",
                    href: "/zh-Hans/docs/advanced-topics/server-side-rendering",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Immutable",
                    href: "/zh-Hans/docs/advanced-topics/immutable",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "More",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Debugging",
                    href: "/zh-Hans/docs/more/debugging",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Deployment",
                    href: "/zh-Hans/docs/more/deployment",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "CSS",
                    href: "/zh-Hans/docs/more/css",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Testing",
                    href: "/zh-Hans/docs/more/testing",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Roadmap",
                    href: "/zh-Hans/docs/more/roadmap",
                }),
            ],
        }),
        migration_guides_sidebar(),
    ]
}
