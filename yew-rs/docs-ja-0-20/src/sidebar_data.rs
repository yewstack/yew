use yew_site_lib::{migration_guides_sidebar, SidebarCategory, SidebarEntry, SidebarItem};

pub fn docs_sidebar() -> Vec<SidebarEntry> {
    vec![
        SidebarEntry::Category(SidebarCategory {
            label: "Getting Started",
            link: Some("/ja/docs/getting-started"),
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Build a Sample App",
                    href: "/ja/docs/getting-started/build-a-sample-app",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Examples",
                    href: "/ja/docs/getting-started/examples",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Editor Setup",
                    href: "/ja/docs/getting-started/editor-setup",
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
                            href: "/ja/docs/concepts/basic-web-technologies/html",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "CSS",
                            href: "/ja/docs/concepts/basic-web-technologies/css",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "JavaScript",
                            href: "/ja/docs/concepts/basic-web-technologies/js",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "wasm-bindgen",
                            href: "/ja/docs/concepts/basic-web-technologies/wasm-bindgen",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "web-sys",
                            href: "/ja/docs/concepts/basic-web-technologies/web-sys",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Components",
                    link: Some("/ja/docs/concepts/function-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/ja/docs/concepts/function-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/ja/docs/concepts/function-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Children",
                            href: "/ja/docs/concepts/function-components/children",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Pure Components",
                            href: "/ja/docs/concepts/function-components/pure-components",
                        }),
                        SidebarEntry::Category(SidebarCategory {
                            label: "Hooks",
                            link: Some("/ja/docs/concepts/function-components/hooks"),
                            items: vec![SidebarEntry::Item(SidebarItem {
                                label: "Custom Hooks",
                                href: "/ja/docs/concepts/function-components/hooks/custom-hooks",
                            })],
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Node Refs",
                            href: "/ja/docs/concepts/function-components/node-refs",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "State",
                            href: "/ja/docs/concepts/function-components/state",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Communication",
                            href: "/ja/docs/concepts/function-components/communication",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Generics",
                            href: "/ja/docs/concepts/function-components/generics",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "HTML",
                    link: Some("/ja/docs/concepts/html"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Components",
                            href: "/ja/docs/concepts/html/components",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Elements",
                            href: "/ja/docs/concepts/html/elements",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Events",
                            href: "/ja/docs/concepts/html/events",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Classes",
                            href: "/ja/docs/concepts/html/classes",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Fragments",
                            href: "/ja/docs/concepts/html/fragments",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lists",
                            href: "/ja/docs/concepts/html/lists",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Literals and Expressions",
                            href: "/ja/docs/concepts/html/literals-and-expressions",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Conditional Rendering",
                            href: "/ja/docs/concepts/html/conditional-rendering",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Agents",
                    href: "/ja/docs/concepts/agents",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Contexts",
                    href: "/ja/docs/concepts/contexts",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Router",
                    href: "/ja/docs/concepts/router",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Suspense",
                    href: "/ja/docs/concepts/suspense",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "Advanced Topics",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "How It Works",
                    href: "/ja/docs/advanced-topics/how-it-works",
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Struct Components",
                    link: Some("/ja/docs/advanced-topics/struct-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Higher Order Components",
                            href: "/ja/docs/advanced-topics/struct-components/hoc",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lifecycle",
                            href: "/ja/docs/advanced-topics/struct-components/lifecycle",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Scope",
                            href: "/ja/docs/advanced-topics/struct-components/scope",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/ja/docs/advanced-topics/struct-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/ja/docs/advanced-topics/struct-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Refs",
                            href: "/ja/docs/advanced-topics/struct-components/refs",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Children",
                    href: "/ja/docs/advanced-topics/children",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Optimizations",
                    href: "/ja/docs/advanced-topics/optimizations",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Portals",
                    href: "/ja/docs/advanced-topics/portals",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Server-Side Rendering",
                    href: "/ja/docs/advanced-topics/server-side-rendering",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Immutable",
                    href: "/ja/docs/advanced-topics/immutable",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "More",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Debugging",
                    href: "/ja/docs/more/debugging",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Deployment",
                    href: "/ja/docs/more/deployment",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "CSS",
                    href: "/ja/docs/more/css",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Testing",
                    href: "/ja/docs/more/testing",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Roadmap",
                    href: "/ja/docs/more/roadmap",
                }),
            ],
        }),
        migration_guides_sidebar(),
    ]
}
