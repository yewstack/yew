use std::collections::HashSet;

use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SidebarItem {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct SidebarCategory {
    pub label: &'static str,
    pub link: Option<&'static str>,
    pub items: Vec<SidebarEntry>,
}

#[derive(Clone, PartialEq)]
pub enum SidebarEntry {
    Item(SidebarItem),
    Category(SidebarCategory),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SidebarProps {
    pub entries: Vec<SidebarEntry>,
    #[prop_or_default]
    pub active_path: AttrValue,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub all_open: bool,
    #[prop_or_default]
    pub lang: AttrValue,
    #[prop_or_default]
    pub doc_version: AttrValue,
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
                if item.href == active_path {
                    for &label in path.iter() {
                        result.insert(label);
                    }
                    return true;
                }
            }
            SidebarEntry::Category(cat) => {
                path.push(cat.label);
                if cat.link == Some(active_path) {
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

#[styled_component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    let active_path = props.active_path.clone();

    let all_open = props.all_open;
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
            collect_all_labels(&props.entries, &mut result);
            result
        } else {
            let mut result = HashSet::new();
            let mut path = Vec::new();
            collect_active_categories(&props.entries, active_path.as_str(), &mut path, &mut result);
            result
        }
    };

    let open_categories = yew_hooks::use_set(initially_open);

    let nav_ctx = use_context::<crate::NavigationContext>();

    let aria_label = if props.title.is_empty() {
        "Docs sidebar"
    } else {
        "Blog recent posts navigation"
    };

    html! {
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
                if !props.title.is_empty() {
                    <div class={css!(font-size: 0.875rem; font-weight: 700; padding: 0.375rem 0.75rem; margin-bottom: 0.25rem; color: var(--color-text);)}>{&props.title}</div>
                }
                <ul class={css!(list-style: none; padding: 0; margin: 0;)}>
                    for entry in props.entries.iter() { {render_entry(entry, &active_path, &open_categories, props.lang.as_str(), props.doc_version.as_str(), &nav_ctx)} }
                </ul>
            </nav>
        </aside>
    }
}

fn make_nav_onclick(
    nav_ctx: &Option<crate::NavigationContext>,
    href: &str,
) -> Option<Callback<MouseEvent>> {
    let nav = nav_ctx.as_ref()?;
    let navigate = nav.navigate.clone();
    let href = AttrValue::from(href.to_owned());
    Some(Callback::from(move |e: MouseEvent| {
        navigate.emit((e, href.clone()));
    }))
}

fn render_entry(
    entry: &SidebarEntry,
    active_path: &str,
    open_categories: &yew_hooks::UseSetHandle<&'static str>,
    lang: &str,
    doc_version: &str,
    nav_ctx: &Option<crate::NavigationContext>,
) -> Html {
    use stylist::css;

    use super::layout::rewrite_doc_href;

    match entry {
        SidebarEntry::Item(item) => {
            let is_active = active_path == item.href;
            let href = rewrite_doc_href(item.href, lang, doc_version);
            let onclick = make_nav_onclick(nav_ctx, &href);
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
            let is_open = open_categories.current().contains(cat.label);
            let toggle = {
                let open_categories = open_categories.clone();
                let label = cat.label;
                Callback::from(move |_: MouseEvent| {
                    if open_categories.current().contains(label) {
                        open_categories.remove(&label);
                    } else {
                        open_categories.insert(label);
                    }
                })
            };
            let cat_href = cat.link.map(|h| rewrite_doc_href(h, lang, doc_version));
            let cat_onclick = cat_href
                .as_deref()
                .and_then(|h| make_nav_onclick(nav_ctx, h));
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
                            for e in cat.items.iter() { {render_entry(e, active_path, open_categories, lang, doc_version, nav_ctx)} }
                        </ul>
                    }
                </li>
            }
        }
    }
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
