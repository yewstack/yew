use std::collections::HashSet;
use std::rc::Rc;

use implicit_clone::unsync::IArray;
use implicit_clone::ImplicitClone;
use yew::prelude::*;
use yew_site_proc::comp;

#[derive(Clone, PartialEq, ImplicitClone)]
pub struct SidebarItem {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone, PartialEq, ImplicitClone)]
pub struct SidebarCategory {
    pub label: &'static str,
    pub link: Option<&'static str>,
    pub items: Vec<SidebarEntry>,
}

#[derive(Clone, PartialEq, ImplicitClone)]
pub enum SidebarEntry {
    Item(SidebarItem),
    Category(SidebarCategory),
}

fn strip_locale(href: &str) -> &str {
    for prefix in ["/ja/", "/zh-Hans/", "/zh-Hant/"] {
        if let Some(rest) = href.strip_prefix(prefix) {
            return rest.strip_prefix('/').unwrap_or(rest);
        }
    }
    href.strip_prefix('/').unwrap_or(href)
}

fn paths_match(a: &str, b: &str) -> bool {
    strip_locale(a) == strip_locale(b)
}

fn collect_active_categories(
    entries: &[SidebarEntry],
    active_path: &str,
    path: &mut Vec<&'static str>,
    result: &mut HashSet<&'static str>,
) -> bool {
    for entry in entries {
        match entry {
            SidebarEntry::Item(item) => {
                if paths_match(item.href, active_path) {
                    for &label in path.iter() {
                        result.insert(label);
                    }
                    return true;
                }
            }
            SidebarEntry::Category(cat) => {
                path.push(cat.label);
                if cat.link.is_some_and(|l| paths_match(l, active_path)) {
                    for &label in path.iter() {
                        result.insert(label);
                    }
                    path.pop();
                    return true;
                }
                if collect_active_categories(&cat.items, active_path, path, result) {
                    path.pop();
                    return true;
                }
                path.pop();
            }
        }
    }
    false
}

#[derive(Clone, PartialEq)]
struct OpenCategories {
    set: Rc<HashSet<&'static str>>,
    toggle: Callback<&'static str>,
}

#[comp]
pub fn Sidebar(
    entries: IArray<SidebarEntry>,
    #[prop_or_default] active_path: AttrValue,
    #[prop_or_default] title: AttrValue,
    #[prop_or_default] all_open: bool,
    #[prop_or_default] lang: AttrValue,
    #[prop_or_default] doc_version: AttrValue,
) {
    let initially_open = {
        if all_open {
            fn collect_all_labels(entries: &[SidebarEntry], result: &mut HashSet<&'static str>) {
                for entry in entries {
                    if let SidebarEntry::Category(cat) = entry {
                        result.insert(cat.label);
                        collect_all_labels(&cat.items, result);
                    }
                }
            }
            let mut result = HashSet::new();
            collect_all_labels(&entries, &mut result);
            result
        } else {
            let mut result = HashSet::new();
            let mut path = Vec::new();
            collect_active_categories(&entries, active_path.as_str(), &mut path, &mut result);
            result
        }
    };

    let open_state = use_state(|| Rc::new(initially_open));

    {
        let open_state = open_state.clone();
        let entries = entries.clone();
        let active_path = active_path.clone();
        use_effect_with(active_path.clone(), move |_| {
            let mut needed = HashSet::new();
            let mut path = Vec::new();
            collect_active_categories(&entries, active_path.as_str(), &mut path, &mut needed);
            if !needed.is_empty() && !needed.is_subset(&**open_state) {
                let mut next = (**open_state).clone();
                next.extend(needed);
                open_state.set(Rc::new(next));
            }
        });
    }

    let open_ctx = {
        let state_for_toggle = open_state.clone();
        let toggle = Callback::from(move |label: &'static str| {
            let mut next = (*state_for_toggle).as_ref().clone();
            if next.contains(label) {
                next.remove(label);
            } else {
                next.insert(label);
            }
            state_for_toggle.set(Rc::new(next));
        });
        OpenCategories {
            set: (*open_state).clone(),
            toggle,
        }
    };

    let aria_label = if title.is_empty() {
        "Docs sidebar"
    } else {
        "Blog recent posts navigation"
    };

    html! {
        <ContextProvider<OpenCategories> context={open_ctx}>
            <aside class={css!(r#"
                width: var(--sidebar-width);
                flex-shrink: 0;
                border-right: 1px solid var(--color-border);
                overflow-y: auto;
                position: sticky;
                top: var(--navbar-height);
                height: calc(100vh - var(--navbar-height));
                padding: 0.5rem 0;
                @media (max-width: 700px) {
                    & {
                        width: 100%;
                        position: static;
                        height: auto;
                        border-right: none;
                    }
                }
            "#)}>
                <nav class={css!(padding: 0 0.5rem;)} aria-label={aria_label}>
                    if !title.is_empty() {
                        <div class={css!(font-size: 0.875rem; font-weight: 700; padding: 0.375rem 0.75rem; margin-bottom: 0.25rem; color: var(--color-text);)}>{&title}</div>
                    }
                    <ul class={css!(list-style: none; padding: 0; margin: 0;)}>
                        for entry in entries.iter() {
                            <EntryView
                                entry={entry.clone()}
                                active_path={&active_path}
                                lang={&lang}
                                doc_version={&doc_version}
                            />
                        }
                    </ul>
                </nav>
            </aside>
        </ContextProvider<OpenCategories>>
    }
}

#[comp]
fn EntryView(entry: SidebarEntry, active_path: AttrValue, lang: AttrValue, doc_version: AttrValue) {
    use super::layout::rewrite_doc_href;

    let nav_ctx = use_context::<crate::NavigationContext>();
    let open_ctx = use_context::<OpenCategories>();

    match &entry {
        SidebarEntry::Item(item) => {
            let is_active = paths_match(active_path.as_str(), item.href);
            let href = rewrite_doc_href(item.href, lang.as_str(), doc_version.as_str());
            let onclick = crate::nav_onclick(&nav_ctx, &href);
            let link_color = if is_active {
                "var(--color-primary)"
            } else {
                "var(--color-text-secondary)"
            };
            let link_bg = if is_active {
                "var(--color-bg-secondary)"
            } else {
                "transparent"
            };
            let link_fw = if is_active { "600" } else { "normal" };
            html! {
                <li class={css!(margin: 1px 0;)}>
                    <a
                        class={css!(display: block; padding: 0.375rem 0.75rem; color: ${link_color}; font-size: 0.875rem; border-radius: 4px; text-decoration: none; background: ${link_bg}; font-weight: ${link_fw}; &:hover { color: var(--color-primary); background: var(--color-bg-secondary); text-decoration: none; })}
                        href={href}
                        {onclick}
                    >
                        {item.label}
                    </a>
                </li>
            }
        }
        SidebarEntry::Category(cat) => {
            let is_open = open_ctx
                .as_ref()
                .map(|ctx| ctx.set.contains(cat.label))
                .unwrap_or(false);
            let toggle = {
                let open_ctx = open_ctx.clone();
                let label = cat.label;
                Callback::from(move |_: MouseEvent| {
                    if let Some(ctx) = &open_ctx {
                        ctx.toggle.emit(label);
                    }
                })
            };
            let cat_href = cat
                .link
                .map(|h| rewrite_doc_href(h, lang.as_str(), doc_version.as_str()));
            let cat_onclick = cat_href
                .as_deref()
                .and_then(|h| crate::nav_onclick(&nav_ctx, h));
            let caret_rot = if is_open { "none" } else { "rotate(-90deg)" };
            html! {
                <li class={css!(margin: 1px 0;)}>
                    <div
                        class={css!(display: flex; align-items: center; justify-content: space-between; cursor: pointer;)}
                        onclick={toggle}
                        role="button"
                        aria-expanded={is_open.to_string()}
                        aria-label={format!("{} category '{}'", if is_open { "Collapse" } else { "Expand" }, cat.label)}
                    >
                        if let Some(href) = &cat_href {
                            <a class={css!(display: block; flex: 1; padding: 0.375rem 0.75rem; font-size: 0.875rem; font-weight: 600; color: var(--color-text); text-decoration: none; &:hover { color: var(--color-primary); text-decoration: none; })} href={href.clone()} onclick={cat_onclick}>
                                {cat.label}
                            </a>
                        } else {
                            <span class={css!(display: block; flex: 1; padding: 0.375rem 0.75rem; font-size: 0.875rem; font-weight: 600; color: var(--color-text);)}>{cat.label}</span>
                        }
                        <span class={css!(display: flex; align-items: center; justify-content: center; padding: 0.25rem; color: var(--color-text-secondary); flex-shrink: 0;)}>
                            <svg class={css!(transition: transform 0.2s; transform: ${caret_rot};)} viewBox="0 0 24 24" width="16" height="16">
                                <path fill="currentColor" d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6z"/>
                            </svg>
                        </span>
                    </div>
                    if is_open {
                        <ul class={css!(list-style: none; padding: 0; margin: 0; padding-left: 0.75rem;)}>
                            for e in cat.items.iter() {
                                <EntryView
                                    entry={e.clone()}
                                    active_path={&active_path}
                                    lang={&lang}
                                    doc_version={&doc_version}
                                />
                            }
                        </ul>
                    }
                </li>
            }
        }
    }
}

pub fn docs_sidebar() -> IArray<SidebarEntry> {
    vec![
        SidebarEntry::Category(SidebarCategory {
            label: "Getting Started",
            link: Some("/docs/getting-started"),
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Build a Sample App",
                    href: "/docs/getting-started/build-a-sample-app",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Examples",
                    href: "/docs/getting-started/examples",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Editor Setup",
                    href: "/docs/getting-started/editor-setup",
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
                            href: "/docs/concepts/basic-web-technologies/html",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "CSS",
                            href: "/docs/concepts/basic-web-technologies/css",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "JavaScript",
                            href: "/docs/concepts/basic-web-technologies/js",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "wasm-bindgen",
                            href: "/docs/concepts/basic-web-technologies/wasm-bindgen",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "web-sys",
                            href: "/docs/concepts/basic-web-technologies/web-sys",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Components",
                    link: Some("/docs/concepts/function-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/docs/concepts/function-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/docs/concepts/function-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Children",
                            href: "/docs/concepts/function-components/children",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Pure Components",
                            href: "/docs/concepts/function-components/pure-components",
                        }),
                        SidebarEntry::Category(SidebarCategory {
                            label: "Hooks",
                            link: Some("/docs/concepts/function-components/hooks"),
                            items: vec![SidebarEntry::Item(SidebarItem {
                                label: "Custom Hooks",
                                href: "/docs/concepts/function-components/hooks/custom-hooks",
                            })],
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Node Refs",
                            href: "/docs/concepts/function-components/node-refs",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "State",
                            href: "/docs/concepts/function-components/state",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Communication",
                            href: "/docs/concepts/function-components/communication",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Generics",
                            href: "/docs/concepts/function-components/generics",
                        }),
                    ],
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "HTML",
                    link: Some("/docs/concepts/html"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Components",
                            href: "/docs/concepts/html/components",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Elements",
                            href: "/docs/concepts/html/elements",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Events",
                            href: "/docs/concepts/html/events",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Classes",
                            href: "/docs/concepts/html/classes",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Fragments",
                            href: "/docs/concepts/html/fragments",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lists",
                            href: "/docs/concepts/html/lists",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Literals and Expressions",
                            href: "/docs/concepts/html/literals-and-expressions",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Conditional Rendering",
                            href: "/docs/concepts/html/conditional-rendering",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Agents",
                    href: "/docs/concepts/agents",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Contexts",
                    href: "/docs/concepts/contexts",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Router",
                    href: "/docs/concepts/router",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Suspense",
                    href: "/docs/concepts/suspense",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "Advanced Topics",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "How It Works",
                    href: "/docs/advanced-topics/how-it-works",
                }),
                SidebarEntry::Category(SidebarCategory {
                    label: "Struct Components",
                    link: Some("/docs/advanced-topics/struct-components"),
                    items: vec![
                        SidebarEntry::Item(SidebarItem {
                            label: "Higher Order Components",
                            href: "/docs/advanced-topics/struct-components/hoc",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Lifecycle",
                            href: "/docs/advanced-topics/struct-components/lifecycle",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Scope",
                            href: "/docs/advanced-topics/struct-components/scope",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Callbacks",
                            href: "/docs/advanced-topics/struct-components/callbacks",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Properties",
                            href: "/docs/advanced-topics/struct-components/properties",
                        }),
                        SidebarEntry::Item(SidebarItem {
                            label: "Refs",
                            href: "/docs/advanced-topics/struct-components/refs",
                        }),
                    ],
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Children",
                    href: "/docs/advanced-topics/children",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Optimizations",
                    href: "/docs/advanced-topics/optimizations",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Portals",
                    href: "/docs/advanced-topics/portals",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Server-Side Rendering",
                    href: "/docs/advanced-topics/server-side-rendering",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Immutable",
                    href: "/docs/advanced-topics/immutable",
                }),
            ],
        }),
        SidebarEntry::Category(SidebarCategory {
            label: "More",
            link: None,
            items: vec![
                SidebarEntry::Item(SidebarItem {
                    label: "Debugging",
                    href: "/docs/more/debugging",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Deployment",
                    href: "/docs/more/deployment",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "CSS",
                    href: "/docs/more/css",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Testing",
                    href: "/docs/more/testing",
                }),
                SidebarEntry::Item(SidebarItem {
                    label: "Roadmap",
                    href: "/docs/more/roadmap",
                }),
            ],
        }),
        migration_guides_sidebar(),
    ]
    .into()
}

pub fn migration_guides_sidebar() -> SidebarEntry {
    SidebarEntry::Category(SidebarCategory {
        label: "Migration Guides",
        link: None,
        items: vec![
            SidebarEntry::Category(SidebarCategory {
                label: "yew",
                link: None,
                items: vec![
                    SidebarEntry::Item(SidebarItem {
                        label: "0.22 to 0.23",
                        href: "/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.21 to 0.22",
                        href: "/docs/migration-guides/yew/from-0-21-0-to-0-22-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.20 to 0.21",
                        href: "/docs/migration-guides/yew/from-0-20-0-to-0-21-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.19 to 0.20",
                        href: "/docs/migration-guides/yew/from-0-19-0-to-0-20-0",
                    }),
                ],
            }),
            SidebarEntry::Category(SidebarCategory {
                label: "yew-agent",
                link: None,
                items: vec![
                    SidebarEntry::Item(SidebarItem {
                        label: "0.4 to 0.5",
                        href: "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.3 to 0.4",
                        href: "/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.1 to 0.2",
                        href: "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.0 to 0.1",
                        href: "/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0",
                    }),
                ],
            }),
            SidebarEntry::Category(SidebarCategory {
                label: "yew-router",
                link: None,
                items: vec![
                    SidebarEntry::Item(SidebarItem {
                        label: "0.19 to 0.20",
                        href: "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.16 to 0.17",
                        href: "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0",
                    }),
                    SidebarEntry::Item(SidebarItem {
                        label: "0.15 to 0.16",
                        href: "/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0",
                    }),
                ],
            }),
        ],
    })
}

pub fn flatten_sidebar(entries: &[SidebarEntry]) -> Vec<(&str, &str)> {
    let mut result = Vec::new();
    for entry in entries {
        match entry {
            SidebarEntry::Item(item) => result.push((item.label, item.href)),
            SidebarEntry::Category(cat) => {
                if let Some(href) = cat.link {
                    result.push((cat.label, href));
                }
                result.extend(flatten_sidebar(&cat.items));
            }
        }
    }
    result
}
