use stylist::yew::styled_component;
#[cfg(feature = "csr")]
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::sidebar::{flatten_sidebar, Sidebar, SidebarEntry};
use crate::content::TocEntry;
use crate::styles::GlobalStyles;
use crate::DocContext;

#[derive(Clone, PartialEq, Properties)]
pub struct LayoutProps {
    pub children: Html,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub description: AttrValue,
    #[prop_or_default]
    pub sidebar: Option<Vec<SidebarEntry>>,
    #[prop_or_default]
    pub active_sidebar_path: AttrValue,
    #[prop_or_default]
    pub active_nav: AttrValue,
    #[prop_or_default]
    pub doc_version: AttrValue,
    #[prop_or_default]
    pub lang: AttrValue,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub markdown: AttrValue,
    #[prop_or_default]
    pub sidebar_title: AttrValue,
    #[prop_or_default]
    pub sidebar_all_open: bool,
    #[prop_or_default]
    pub toc: Vec<TocEntry>,
}

#[derive(Clone, PartialEq, Properties)]
struct VersionBannerProps {
    doc_version: AttrValue,
    lang: AttrValue,
}

#[styled_component]
fn VersionBanner(props: &VersionBannerProps) -> Html {
    use yew::virtual_dom::VNode;
    if props.doc_version.is_empty() || props.doc_version.as_str() == crate::LATEST_STABLE {
        return VNode::default();
    }
    let lang_p = crate::lang_prefix(props.lang.as_str());
    let latest_path = format!("{lang_p}/docs/getting-started");
    let latest_label = format!("latest version ({})", crate::LATEST_STABLE);
    let is_next = props.doc_version.as_str() == "Next";
    html! {
        <div class={css!(
            padding: 0.75rem 1rem;
            border: 1px solid #e6a700;
            border-radius: 6px;
            background: #fff8e1;
            color: #5a4600;
            margin-bottom: 1rem;
            font-size: 0.9rem;
            line-height: 1.5;
        )} role="alert">
            if is_next {
                <div>{"This is unreleased documentation for Yew "}<b>{"Next"}</b>{" version."}</div>
            } else {
                <div>{"This is documentation for Yew "}<b>{props.doc_version.to_string()}</b>{", which is no longer actively maintained."}</div>
            }
            <div class={css!(margin-top: 0.5rem;)}>
                {"For up-to-date documentation, see the "}
                <b><a class={css!(color: #1a73e8; font-weight: 600;)} href={latest_path}>{latest_label}</a></b>
                {"."}
            </div>
        </div>
    }
}

fn version_slug(doc_version: &str) -> &str {
    crate::VERSIONS
        .iter()
        .find(|(label, _)| *label == doc_version)
        .map(|(_, slug)| *slug)
        .unwrap_or("")
}

pub fn rewrite_doc_href(href: &str, lang: &str, doc_version: &str) -> String {
    let after_lang = crate::strip_lang_prefix(href, lang);

    if let Some(rest) = after_lang.strip_prefix("/docs/") {
        let lang_p = crate::lang_prefix(lang);
        if rest.starts_with("migration-guides/") {
            format!("{lang_p}/docs/{rest}")
        } else {
            let slug = version_slug(doc_version);
            if slug.is_empty() {
                format!("{lang_p}/docs/{rest}")
            } else {
                format!("{lang_p}/docs/{slug}/{rest}")
            }
        }
    } else {
        href.to_string()
    }
}

fn edit_page_url(active_path: &str, lang: &str, doc_version: &str) -> String {
    const BASE: &str = "https://github.com/yewstack/yew/blob/master/yew-rs";
    const SECTION_INDEX_SLUGS: &[&str] = &[
        "getting-started",
        "concepts/function-components",
        "concepts/function-components/hooks",
        "concepts/html",
        "advanced-topics/struct-components",
    ];

    let mut crate_dir = "docs".to_string();
    if !lang.is_empty() {
        crate_dir.push('-');
        crate_dir.push_str(&lang.to_lowercase());
    }
    if !doc_version.is_empty() && doc_version != "Next" {
        crate_dir.push('-');
        crate_dir.push_str(&doc_version.replace('.', "-"));
    }

    let bare = crate::strip_lang_prefix(active_path, lang);

    let Some(page_path) = bare.strip_prefix("/docs/") else {
        return String::new();
    };
    let mut file_path: String = page_path
        .split('/')
        .map(|seg| seg.replace('-', "_"))
        .collect::<Vec<_>>()
        .join("/");

    if SECTION_INDEX_SLUGS.contains(&page_path) {
        file_path.push_str("/introduction");
    }

    format!("{BASE}/{crate_dir}/src/pages/{file_path}.rs")
}

fn build_breadcrumbs(
    entries: &[SidebarEntry],
    active_path: &str,
) -> Option<Vec<(&'static str, Option<&'static str>)>> {
    for entry in entries {
        match entry {
            SidebarEntry::Item(item) => {
                if item.href == active_path {
                    return Some(vec![(item.label, None)]);
                }
            }
            SidebarEntry::Category(cat) => {
                if let Some(mut trail) = build_breadcrumbs(&cat.items, active_path) {
                    trail.insert(0, (cat.label, cat.link));
                    return Some(trail);
                }
                if cat.link == Some(active_path) {
                    return Some(vec![(cat.label, None)]);
                }
            }
        }
    }
    None
}

#[styled_component]
pub fn Layout(props: &LayoutProps) -> Html {
    let has_sidebar = props.sidebar.as_ref().is_some_and(|s| !s.is_empty());
    let nav_ctx = use_context::<crate::NavigationContext>();

    let mobile_sidebar_open = use_state(|| false);

    #[cfg(feature = "csr")]
    let (copied, on_copy_md) = {
        let (c, cb) = crate::use_clipboard(props.markdown.clone());
        (c, Some(cb))
    };
    #[cfg(not(feature = "csr"))]
    let (copied, on_copy_md) = (false, None::<Callback<MouseEvent>>);

    let content_ref = use_node_ref();

    {
        let title = props.title.clone();
        use_effect_with(title.clone(), move |_| {
            #[cfg(feature = "csr")]
            {
                let display = if title.is_empty() {
                    "Yew".to_string()
                } else {
                    format!("{title} | Yew")
                };
                gloo::utils::document().set_title(&display);
            }
        });
    }

    {
        let content_ref = content_ref.clone();
        let sidebar_path = props.active_sidebar_path.clone();
        use_effect_with(sidebar_path, move |_| {
            scroll_to_hash(&content_ref);
        });
    }

    let breadcrumbs = props
        .sidebar
        .as_ref()
        .and_then(|entries| build_breadcrumbs(entries, props.active_sidebar_path.as_str()));

    let pagination = props.sidebar.as_ref().map(|entries| {
        let pages = flatten_sidebar(entries);
        let active = props.active_sidebar_path.as_str();
        let lang = props.lang.as_str();
        let ver = props.doc_version.as_str();
        let idx = pages.iter().position(|(_, href)| *href == active);
        let prev: Option<(String, String)> = idx
            .and_then(|i| if i > 0 { Some(pages[i - 1]) } else { None })
            .map(|(l, h)| (l.to_string(), rewrite_doc_href(h, lang, ver)));
        let next: Option<(String, String)> = idx
            .and_then(|i| pages.get(i + 1).copied())
            .map(|(l, h)| (l.to_string(), rewrite_doc_href(h, lang, ver)));
        (prev, next)
    });

    let edit_url = if has_sidebar {
        edit_page_url(
            props.active_sidebar_path.as_str(),
            props.lang.as_str(),
            props.doc_version.as_str(),
        )
    } else {
        String::new()
    };

    let has_toc = !props.toc.is_empty();
    let main_dir = if has_sidebar || has_toc {
        "row"
    } else {
        "column"
    };
    let content_max_w = if props.full_width {
        "none"
    } else {
        "var(--content-max-width)"
    };
    let content_pad = if props.full_width { "0" } else { "2rem" };
    let content_margin = if has_sidebar || has_toc {
        "0"
    } else {
        "0 auto"
    };

    let doc_ctx = crate::DocContext {
        lang: props.lang.clone(),
        doc_version: props.doc_version.clone(),
    };

    html! {
        <ContextProvider<DocContext> context={doc_ctx}>
        <div class={css!(display: flex; flex-direction: column; min-height: 100vh;)}>
            <GlobalStyles />
            <Navbar
                active={props.active_nav.clone()}
                doc_version={props.doc_version.clone()}
                lang={props.lang.clone()}
                current_path={props.active_sidebar_path.clone()}
            />
            <div class={css!(
                flex: 1;
                display: flex;
                flex-direction: ${main_dir};
                margin-top: var(--navbar-height);
                max-width: 1440px;
                margin-left: auto;
                margin-right: auto;
                width: 100%;
                @media (max-width: 700px) {
                    flex-direction: column;
                }
            )}>
                if let Some(entries) = props.sidebar.as_ref().filter(|s| !s.is_empty()) {
                    <div class={if *mobile_sidebar_open {
                        css!(
                            display: contents;
                            @media (max-width: 700px) {
                                display: block;
                                border-bottom: 1px solid var(--color-border);
                                margin-bottom: 0.5rem;
                            }
                        )
                    } else {
                        css!(
                            display: contents;
                            @media (max-width: 700px) {
                                display: none;
                            }
                        )
                    }}>
                        <Sidebar
                            entries={entries.clone()}
                            active_path={props.active_sidebar_path.clone()}
                            title={props.sidebar_title.clone()}
                            all_open={props.sidebar_all_open}
                            lang={props.lang.clone()}
                            doc_version={props.doc_version.clone()}
                        />
                    </div>
                }
                <main ref={content_ref.clone()} class={css!(
                    flex: 1;
                    max-width: ${content_max_w};
                    padding: ${content_pad};
                    width: 100%;
                    position: relative;
                    margin: ${content_margin};
                )}>
                    if has_sidebar {
                        <button class={css!(
                            display: none;
                            align-items: center;
                            gap: 0.5rem;
                            padding: 0.5rem 0.75rem;
                            background: var(--color-bg-secondary);
                            border: 1px solid var(--color-border);
                            border-radius: 4px;
                            font-size: 0.875rem;
                            cursor: pointer;
                            color: var(--color-text);
                            margin-bottom: 1rem;
                            margin-right: 0.75rem;
                            font-family: inherit;
                            font-weight: 500;
                            &:hover {
                                border-color: var(--color-primary);
                                color: var(--color-primary);
                            }
                            @media (max-width: 700px) {
                                display: inline-flex;
                            }
                        )} onclick={let mobile_sidebar_open = mobile_sidebar_open.clone(); move |_: MouseEvent| mobile_sidebar_open.set(!*mobile_sidebar_open)}>
                            <svg viewBox="0 0 24 24" width="16" height="16">
                                <path fill="currentColor" d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
                            </svg>
                            if *mobile_sidebar_open {
                                {"Hide menu"}
                            } else {
                                {"Show menu"}
                            }
                        </button>
                    }
                    if has_sidebar {
                        <VersionBanner doc_version={props.doc_version.clone()} lang={props.lang.clone()} />
                    }
                    if let Some(trail) = &breadcrumbs {
                        <nav class={css!(margin-bottom: 0.5rem;)} aria-label="Breadcrumbs">
                            <ul class={css!(
                                display: flex;
                                align-items: center;
                                list-style: none;
                                padding: 0;
                                margin: 0;
                                flex-wrap: wrap;
                                font-size: 0.875rem;
                            )}>
                                <li class={css!(display: flex; align-items: center;)}>
                                    <a class={css!(
                                        color: var(--color-text-secondary);
                                        text-decoration: none;
                                        &:hover { color: var(--color-primary); }
                                    )} href="/" aria-label="Home page">
                                        <svg viewBox="0 0 24 24" class={css!(width: 18px; height: 18px; display: block;)}>
                                            <path d="M10 19v-5h4v5c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-7h1.7c.46 0 .68-.57.33-.87L12.67 3.6c-.38-.34-.96-.34-1.34 0l-8.36 7.53c-.34.3-.13.87.33.87H5v7c0 .55.45 1 1 1h3c.55 0 1-.45 1-1z" fill="currentColor"/>
                                        </svg>
                                    </a>
                                </li>
                                for (i, (label, href)) in trail.iter().enumerate() {
                                    {{
                                        let is_last = i == trail.len() - 1;
                                        if is_last {
                                            html! {
                                                <li class={css!(display: flex; align-items: center;)}>
                                                    <span class={css!(
                                                        margin: 0 0.5rem;
                                                        color: var(--color-text-secondary);
                                                        opacity: 0.5;
                                                    )}>{"/"}</span>
                                                    <span class={css!(
                                                        color: var(--color-text);
                                                        font-weight: 600;
                                                    )}>{label}</span>
                                                </li>
                                            }
                                        } else if let Some(h) = href {
                                            let rewritten = rewrite_doc_href(h, props.lang.as_str(), props.doc_version.as_str());
                                            let bc_onclick = crate::nav_onclick(&nav_ctx, &rewritten);
                                            html! {
                                                <li class={css!(display: flex; align-items: center;)}>
                                                    <span class={css!(
                                                        margin: 0 0.5rem;
                                                        color: var(--color-text-secondary);
                                                        opacity: 0.5;
                                                    )}>{"/"}</span>
                                                    <a class={css!(
                                                        color: var(--color-text-secondary);
                                                        text-decoration: none;
                                                        &:hover { color: var(--color-primary); }
                                                    )} href={rewritten} onclick={bc_onclick}>{label}</a>
                                                </li>
                                            }
                                        } else {
                                            html! {
                                                <li class={css!(display: flex; align-items: center;)}>
                                                    <span class={css!(
                                                        margin: 0 0.5rem;
                                                        color: var(--color-text-secondary);
                                                        opacity: 0.5;
                                                    )}>{"/"}</span>
                                                    <span class={css!(
                                                        color: var(--color-text-secondary);
                                                        text-decoration: none;
                                                    )}>{label}</span>
                                                </li>
                                            }
                                        }
                                    }}
                                }
                            </ul>
                        </nav>
                    }
                    if has_sidebar && !props.doc_version.is_empty() {
                        <span class={css!(
                            display: inline-block;
                            font-size: 0.75rem;
                            font-weight: 600;
                            padding: 0.125rem 0.5rem;
                            border-radius: 4px;
                            background: var(--color-bg-secondary);
                            border: 1px solid var(--color-border);
                            color: var(--color-text-secondary);
                            margin-bottom: 1rem;
                        )}>
                            {"Version: "}{&props.doc_version}
                        </span>
                    }
                    if !props.full_width && !props.markdown.is_empty() {
                        <button class={css!(
                            float: right;
                            margin: 0 0 0.5rem 0.5rem;
                            display: inline-flex;
                            align-items: center;
                            gap: 0.25rem;
                            background: var(--color-bg-secondary);
                            border: 1px solid var(--color-border);
                            border-radius: 4px;
                            padding: 0.25rem 0.5rem;
                            font-size: 0.75rem;
                            font-family: inherit;
                            color: var(--color-text-secondary);
                            cursor: pointer;
                            transition: border-color 0.2s, color 0.2s;
                            &:hover {
                                border-color: var(--color-primary);
                                color: var(--color-primary);
                            }
                            @media (max-width: 700px) {
                                display: none;
                            }
                        )} onclick={on_copy_md} title="Copy page content as Markdown">
                            <svg class={css!(flex-shrink: 0;)} viewBox="0 0 24 24" width="14" height="14">
                                <path fill="currentColor" d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                            </svg>
                            if copied { {"Copied!"} } else { {"Copy as Markdown"} }
                        </button>
                    }
                    if !props.title.is_empty() {
                        <h1 class={css!(margin-top: 0; margin-bottom: 1.5rem;)}>{&props.title}</h1>
                    }
                    {props.children.clone()}
                    if !edit_url.is_empty() {
                        <div class={css!(margin-top: 2rem;)}>
                            <a class={css!(
                                display: inline-flex;
                                align-items: center;
                                gap: 0.375rem;
                                color: var(--color-text-secondary);
                                font-size: 0.875rem;
                                text-decoration: none;
                                transition: color 0.2s;
                                &:hover {
                                    color: var(--color-primary);
                                }
                            )} href={edit_url.clone()} target="_blank" rel="noopener noreferrer">
                                <svg viewBox="0 0 24 24" width="16" height="16" class={css!(flex-shrink: 0;)}>
                                    <path fill="currentColor" d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34a.9959.9959 0 0 0-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                                </svg>
                                {"Edit this page"}
                            </a>
                        </div>
                    }
                    if let Some((prev, next)) = &pagination {
                        <nav class={css!(
                            display: flex;
                            justify-content: space-between;
                            margin-top: 2rem;
                            padding-top: 1.5rem;
                            border-top: 1px solid var(--color-border);
                        )} aria-label="Docs pages">
                            if let Some((label, href)) = prev {
                                <a class={css!(
                                    display: flex;
                                    flex-direction: column;
                                    padding: 0.75rem 1rem;
                                    border: 1px solid var(--color-border);
                                    border-radius: 4px;
                                    text-decoration: none;
                                    color: var(--color-text);
                                    max-width: 45%;
                                    transition: border-color 0.2s;
                                    &:hover {
                                        border-color: var(--color-primary);
                                        text-decoration: none;
                                    }
                                )} href={href.clone()} onclick={crate::nav_onclick(&nav_ctx, href)}>
                                    <span class={css!(
                                        font-size: 0.75rem;
                                        color: var(--color-text-secondary);
                                        margin-bottom: 0.25rem;
                                    )}>{"Previous"}</span>
                                    <span class={css!(
                                        font-size: 0.9375rem;
                                        font-weight: 600;
                                        color: var(--color-primary);
                                    )}>{label}</span>
                                </a>
                            } else {
                                <span class={css!(flex: 1;)} />
                            }
                            if let Some((label, href)) = next {
                                <a class={css!(
                                    display: flex;
                                    flex-direction: column;
                                    padding: 0.75rem 1rem;
                                    border: 1px solid var(--color-border);
                                    border-radius: 4px;
                                    text-decoration: none;
                                    color: var(--color-text);
                                    max-width: 45%;
                                    transition: border-color 0.2s;
                                    text-align: right;
                                    margin-left: auto;
                                    &:hover {
                                        border-color: var(--color-primary);
                                        text-decoration: none;
                                    }
                                )} href={href.clone()} onclick={crate::nav_onclick(&nav_ctx, href)}>
                                    <span class={css!(
                                        font-size: 0.75rem;
                                        color: var(--color-text-secondary);
                                        margin-bottom: 0.25rem;
                                    )}>{"Next"}</span>
                                    <span class={css!(
                                        font-size: 0.9375rem;
                                        font-weight: 600;
                                        color: var(--color-primary);
                                    )}>{label}</span>
                                </a>
                            }
                        </nav>
                    }
                </main>
                if !props.toc.is_empty() {
                    <Toc entries={props.toc.clone()} content_ref={content_ref.clone()} />
                }
            </div>
            <Footer />
        </div>
        </ContextProvider<DocContext>>
    }
}

#[cfg(feature = "csr")]
fn scroll_to_hash(content_ref: &NodeRef) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let hash = window.location().hash().unwrap_or_default();
    if hash.len() > 1 {
        let content_el = match content_ref.cast::<web_sys::Element>() {
            Some(el) => el,
            None => return,
        };
        if let Ok(Some(el)) = content_el.query_selector(&format!("[id=\"{}\"]", &hash[1..])) {
            gloo::timers::callback::Timeout::new(50, move || {
                el.scroll_into_view();
            })
            .forget();
        }
    } else {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

#[cfg(not(feature = "csr"))]
fn scroll_to_hash(_content_ref: &NodeRef) {}

#[derive(Clone, PartialEq, Properties)]
struct TocProps {
    entries: Vec<TocEntry>,
    content_ref: NodeRef,
}

#[styled_component]
fn Toc(props: &TocProps) -> Html {
    #[cfg(feature = "csr")]
    let active_id = {
        let active_id = use_state(|| Option::<AttrValue>::None);

        let compute = {
            let active_id = active_id.clone();
            let entries = props.entries.clone();
            let content_ref = props.content_ref.clone();
            use_memo(entries, move |entries| {
                let window = web_sys::window().unwrap();
                let content_el = content_ref.cast::<web_sys::Element>();
                let navbar_height: f64 = window
                    .document()
                    .and_then(|d| d.query_selector(".navbar").ok().flatten())
                    .map(|el| {
                        let html: web_sys::HtmlElement = el.unchecked_into();
                        html.client_height() as f64
                    })
                    .unwrap_or(60.0);
                let ids: Vec<AttrValue> = entries.iter().map(|e| e.id.clone()).collect();
                std::rc::Rc::new(move || {
                    let content = match &content_el {
                        Some(el) => el,
                        None => return,
                    };
                    let mut active: Option<AttrValue> = None;
                    let mut next_visible_idx: Option<usize> = None;
                    for (i, id) in ids.iter().enumerate() {
                        if let Ok(Some(el)) =
                            content.query_selector(&format!("[id=\"{}\"]", id.as_str()))
                        {
                            let rect = el.get_bounding_client_rect();
                            if rect.top() >= navbar_height {
                                next_visible_idx = Some(i);
                                break;
                            }
                        }
                    }
                    if let Some(idx) = next_visible_idx {
                        if let Ok(Some(el)) =
                            content.query_selector(&format!("[id=\"{}\"]", ids[idx].as_str()))
                        {
                            let rect = el.get_bounding_client_rect();
                            let vh = window
                                .inner_height()
                                .ok()
                                .and_then(|v| v.as_f64())
                                .unwrap_or(800.0);
                            if rect.top() > 0.0 && rect.bottom() < vh / 2.0 {
                                active = Some(ids[idx].clone());
                            } else if idx > 0 {
                                active = Some(ids[idx - 1].clone());
                            }
                        }
                    } else if !ids.is_empty() {
                        active = Some(ids[ids.len() - 1].clone());
                    }
                    active_id.set(active);
                })
            })
        };

        {
            let compute = compute.clone();
            yew_hooks::use_effect_once(move || {
                compute();
            });
        }
        {
            let compute = compute.clone();
            yew_hooks::use_event_with_window("scroll", move |_: Event| compute());
        }
        yew_hooks::use_event_with_window("resize", move |_: Event| compute());

        active_id
    };
    #[cfg(feature = "csr")]
    let active_id: &Option<AttrValue> = &active_id;

    #[cfg(not(feature = "csr"))]
    let active_id: &Option<AttrValue> = &None;

    html! {
        <aside class={css!(
            width: 250px;
            flex-shrink: 0;
            @media (max-width: 996px) {
                display: none;
            }
        )}>
            <nav class={css!(r#"
                position: sticky;
                top: calc(var(--navbar-height) + 1rem);
                max-height: calc(100vh - var(--navbar-height) - 2rem);
                overflow-y: auto;
                padding: 0 0.75rem;
                font-size: 0.8125rem;
                border-left: 1px solid var(--color-border);
            "#)}>
                <ul class={css!(list-style: none; padding: 0; margin: 0;)}>
                    for TocEntry { id, text, level } in &props.entries {
                        <li style={match level {
                                3 => "padding-left:1rem",
                                4 => "padding-left:2rem",
                                _ => "",
                            }}>
                        <a class={{
                            let is_active = matches!(active_id, Some(i) if id.eq(i));
                            css!(
                            display: block;
                            padding: 0.25rem 0;
                            color: ${if is_active { "var(--color-primary)" } else { "var(--color-text-secondary)" }};
                            text-decoration: none;
                            line-height: 1.3;
                            transition: color 0.2s;
                            font-weight: ${if is_active { "600" } else { "normal" }};
                            &:hover {
                                color: var(--color-primary);
                            }
                        ) }} href={format!("#{}", id)}>{text}</a>
                        </li>
                    }
                </ul>
            </nav>
        </aside>
    }
}
