use stylist::yew::styled_component;
#[cfg(feature = "csr")]
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::sidebar::{flatten_sidebar, Sidebar, SidebarEntry};
use crate::content::TocEntry;
use crate::styles::GlobalStyles;

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

fn version_banner(doc_version: &AttrValue, lang: &AttrValue) -> Html {
    use yew::virtual_dom::VNode;
    if doc_version.is_empty() || doc_version.as_str() == crate::LATEST_STABLE {
        return VNode::default();
    }
    let lang_p = if lang.is_empty() {
        String::new()
    } else {
        format!("/{lang}")
    };
    let latest_path = format!("{lang_p}/docs/getting-started");
    let latest_label = format!("latest version ({})", crate::LATEST_STABLE);
    if doc_version.as_str() == "Next" {
        html! {
            <div class="version-banner" role="alert">
                <div>{"This is unreleased documentation for Yew "}<b>{"Next"}</b>{" version."}</div>
                <div class="banner-secondary">
                    {"For up-to-date documentation, see the "}
                    <b><a href={latest_path}>{latest_label}</a></b>
                    {"."}
                </div>
            </div>
        }
    } else {
        html! {
            <div class="version-banner" role="alert">
                <div>{"This is documentation for Yew "}<b>{doc_version.to_string()}</b>{", which is no longer actively maintained."}</div>
                <div class="banner-secondary">
                    {"For up-to-date documentation, see the "}
                    <b><a href={latest_path}>{latest_label}</a></b>
                    {"."}
                </div>
            </div>
        }
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
    let after_lang = if lang.is_empty() {
        href
    } else {
        let prefix = format!("/{lang}");
        href.strip_prefix(&prefix).unwrap_or(href)
    };

    if let Some(rest) = after_lang.strip_prefix("/docs/") {
        let lang_p = if lang.is_empty() {
            String::new()
        } else {
            format!("/{lang}")
        };
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

fn edit_page_url(active_path: &str, lang: &str) -> String {
    const BASE: &str = "https://github.com/yewstack/yew/blob/master/yew-rs";

    let crate_dir = if lang.is_empty() {
        "docs".to_string()
    } else {
        format!("docs-{}", lang.to_lowercase())
    };

    let bare = if lang.is_empty() {
        active_path
    } else {
        let pfx = format!("/{lang}");
        active_path.strip_prefix(&pfx).unwrap_or(active_path)
    };

    let Some(page_path) = bare.strip_prefix("/docs/") else {
        return String::new();
    };
    let file_path: String = page_path
        .split('/')
        .map(|seg| seg.replace('-', "_"))
        .collect::<Vec<_>>()
        .join("/");

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
    let has_sidebar = props.sidebar.is_some();

    let mobile_sidebar_open = use_state(|| false);
    let toggle_mobile_sidebar = {
        let mobile_sidebar_open = mobile_sidebar_open.clone();
        Callback::from(move |_: MouseEvent| {
            mobile_sidebar_open.set(!*mobile_sidebar_open);
        })
    };

    #[cfg(feature = "csr")]
    let copied = use_state(|| false);

    #[cfg(feature = "csr")]
    let on_copy_md = {
        let md = props.markdown.clone();
        let copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            let md = md.clone();
            let copied = copied.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&md)).await;
                    copied.set(true);
                    let copied2 = copied.clone();
                    gloo::timers::callback::Timeout::new(2000, move || {
                        copied2.set(false);
                    })
                    .forget();
                }
            });
        })
    };

    let content_ref = use_node_ref();

    {
        let content_ref = content_ref.clone();
        use_effect_with((), move |_| {
            scroll_to_hash(&content_ref);
            || {}
        });
    }

    let style = css!(
        r#"
        display: flex;
        flex-direction: column;
        min-height: 100vh;

        .main {
            flex: 1;
            display: flex;
            flex-direction: column;
            margin-top: var(--navbar-height);
            max-width: 1440px;
            margin-left: auto;
            margin-right: auto;
            width: 100%;
        }

        .main--with-sidebar {
            flex-direction: row;
        }

        .content {
            flex: 1;
            max-width: var(--content-max-width);
            padding: 2rem;
            width: 100%;
            position: relative;
            margin: 0 auto;
        }

        .content--full-width {
            max-width: none;
            padding: 0;
        }

        .main--with-sidebar .content {
            margin: 0;
        }

        .page-title {
            margin-top: 0;
            margin-bottom: 1.5rem;
        }

        .blog-post-header {
            margin-bottom: 1.5rem;
        }

        .blog-post-date {
            display: block;
            font-size: 0.875rem;
            color: var(--color-text-secondary);
            margin-bottom: 0.75rem;
        }

        .blog-post-author {
            display: flex;
            align-items: center;
            gap: 0.625rem;
        }

        .blog-post-avatar {
            width: 48px;
            height: 48px;
            border-radius: 50%;
        }

        .blog-post-author-info {
            display: flex;
            flex-direction: column;
        }

        .blog-post-author-name {
            font-weight: 600;
            color: var(--color-text);
            text-decoration: none;
        }

        .blog-post-author-name:hover {
            color: var(--color-primary);
        }

        .blog-post-author-title {
            font-size: 0.8125rem;
            color: var(--color-text-secondary);
        }

        .breadcrumbs {
            margin-bottom: 0.5rem;
        }

        .breadcrumbs-list {
            display: flex;
            align-items: center;
            list-style: none;
            padding: 0;
            margin: 0;
            flex-wrap: wrap;
            font-size: 0.875rem;
        }

        .breadcrumbs-item {
            display: flex;
            align-items: center;
        }

        .breadcrumbs-item + .breadcrumbs-item::before {
            content: "/";
            margin: 0 0.5rem;
            color: var(--color-text-secondary);
            opacity: 0.5;
        }

        .breadcrumbs-link {
            color: var(--color-text-secondary);
            text-decoration: none;
        }

        a.breadcrumbs-link:hover {
            color: var(--color-primary);
        }

        .breadcrumbs-item--active .breadcrumbs-link {
            color: var(--color-text);
            font-weight: 600;
        }

        .breadcrumbs-home-icon {
            width: 18px;
            height: 18px;
            display: block;
        }

        .version-badge {
            display: inline-block;
            font-size: 0.75rem;
            font-weight: 600;
            padding: 0.125rem 0.5rem;
            border-radius: 4px;
            background: var(--color-bg-secondary);
            border: 1px solid var(--color-border);
            color: var(--color-text-secondary);
            margin-bottom: 1rem;
        }

        .version-banner {
            padding: 0.75rem 1rem;
            border: 1px solid #e6a700;
            border-radius: 6px;
            background: #fff8e1;
            color: #5a4600;
            margin-bottom: 1rem;
            font-size: 0.9rem;
            line-height: 1.5;
        }

        .version-banner a {
            color: #1a73e8;
            font-weight: 600;
        }

        .version-banner .banner-secondary {
            margin-top: 0.5rem;
        }

        .pagination {
            display: flex;
            justify-content: space-between;
            margin-top: 2rem;
            padding-top: 1.5rem;
            border-top: 1px solid var(--color-border);
        }

        .pagination-spacer {
            flex: 1;
        }

        .pagination-link {
            display: flex;
            flex-direction: column;
            padding: 0.75rem 1rem;
            border: 1px solid var(--color-border);
            border-radius: 4px;
            text-decoration: none;
            color: var(--color-text);
            max-width: 45%;
            transition: border-color 0.2s;
        }

        .pagination-link:hover {
            border-color: var(--color-primary);
        }

        .pagination-link--next {
            text-align: right;
            margin-left: auto;
        }

        .pagination-sublabel {
            font-size: 0.75rem;
            color: var(--color-text-secondary);
            margin-bottom: 0.25rem;
        }

        .pagination-label {
            font-size: 0.9375rem;
            font-weight: 600;
            color: var(--color-primary);
        }

        .edit-page {
            margin-top: 2rem;
        }

        .edit-page-link {
            display: inline-flex;
            align-items: center;
            gap: 0.375rem;
            color: var(--color-text-secondary);
            font-size: 0.875rem;
            text-decoration: none;
            transition: color 0.2s;
        }

        .edit-page-link:hover {
            color: var(--color-primary);
        }

        .edit-page-icon {
            flex-shrink: 0;
        }

        .toc-column {
            width: 250px;
            flex-shrink: 0;
        }

        .toc-container {
            position: sticky;
            top: calc(var(--navbar-height) + 1rem);
            max-height: calc(100vh - var(--navbar-height) - 2rem);
            overflow-y: auto;
            padding: 0 0.75rem;
            font-size: 0.8125rem;
            border-left: 1px solid var(--color-border);
        }

        .copy-md-btn {
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
        }

        .copy-md-btn:hover {
            border-color: var(--color-primary);
            color: var(--color-primary);
        }

        .copy-md-btn svg {
            flex-shrink: 0;
        }

        .anchor {
            scroll-margin-top: calc(var(--navbar-height) + 1rem);
        }

        .hash-link {
            opacity: 0;
            transition: opacity 0.2s;
            padding-left: 0.5rem;
            color: var(--color-primary);
            text-decoration: none;
            font-weight: normal;
        }

        .anchor:hover .hash-link,
        .hash-link:focus {
            opacity: 1;
            text-decoration: none;
        }

        .toc-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .toc-link {
            display: block;
            padding: 0.25rem 0;
            color: var(--color-text-secondary);
            text-decoration: none;
            line-height: 1.3;
            transition: color 0.2s;
        }

        .toc-link:hover,
        .toc-link--active {
            color: var(--color-primary);
        }

        .toc-link--active {
            font-weight: 600;
        }

        @media (max-width: 996px) {
            .toc-column {
                display: none;
            }
        }

        .mobile-sidebar-toggle {
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
        }

        .mobile-sidebar-toggle:hover {
            border-color: var(--color-primary);
            color: var(--color-primary);
        }

        .sidebar-wrapper {
            display: contents;
        }

        @media (max-width: 700px) {
            .mobile-sidebar-toggle {
                display: inline-flex;
            }

            .sidebar-wrapper {
                display: none;
            }

            .sidebar-wrapper--open {
                display: block;
                border-bottom: 1px solid var(--color-border);
                margin-bottom: 0.5rem;
            }

            .main--with-sidebar {
                flex-direction: column;
            }

            .copy-md-btn {
                display: none;
            }
        }
    "#
    );

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
        edit_page_url(props.active_sidebar_path.as_str(), props.lang.as_str())
    } else {
        String::new()
    };

    #[cfg(feature = "csr")]
    let copy_md_button = if !props.full_width && !props.markdown.is_empty() {
        html! {
            <button class="copy-md-btn" onclick={on_copy_md} title="Copy page content as Markdown">
                <svg viewBox="0 0 24 24" width="14" height="14">
                    <path fill="currentColor" d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                </svg>
                if *copied { {"Copied!"} } else { {"Copy as Markdown"} }
            </button>
        }
    } else {
        html! {}
    };

    #[cfg(not(feature = "csr"))]
    let copy_md_button = if !props.full_width && !props.markdown.is_empty() {
        html! {
            <button class="copy-md-btn" title="Copy page content as Markdown">
                <svg viewBox="0 0 24 24" width="14" height="14">
                    <path fill="currentColor" d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                </svg>
                {"Copy as Markdown"}
            </button>
        }
    } else {
        html! {}
    };

    html! {
        <div class={style}>
            <GlobalStyles />
            <Navbar
                active={props.active_nav.clone()}
                doc_version={props.doc_version.clone()}
                lang={props.lang.clone()}
                current_path={props.active_sidebar_path.clone()}
            />
            <div class={classes!("main", has_sidebar.then_some("main--with-sidebar"))}>
                if let Some(entries) = &props.sidebar {
                    <div class={classes!("sidebar-wrapper", (*mobile_sidebar_open).then_some("sidebar-wrapper--open"))}>
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
                <main ref={content_ref.clone()} class={classes!("content", "doc-content", props.full_width.then_some("content--full-width"))}>
                    if has_sidebar {
                        <button class="mobile-sidebar-toggle" onclick={toggle_mobile_sidebar}>
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
                        {version_banner(&props.doc_version, &props.lang)}
                    }
                    if let Some(trail) = &breadcrumbs {
                        <nav class="breadcrumbs" aria-label="Breadcrumbs">
                            <ul class="breadcrumbs-list">
                                <li class="breadcrumbs-item">
                                    <a class="breadcrumbs-link" href="/" aria-label="Home page">
                                        <svg viewBox="0 0 24 24" class="breadcrumbs-home-icon">
                                            <path d="M10 19v-5h4v5c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-7h1.7c.46 0 .68-.57.33-.87L12.67 3.6c-.38-.34-.96-.34-1.34 0l-8.36 7.53c-.34.3-.13.87.33.87H5v7c0 .55.45 1 1 1h3c.55 0 1-.45 1-1z" fill="currentColor"/>
                                        </svg>
                                    </a>
                                </li>
                                { for trail.iter().enumerate().map(|(i, (label, href))| {
                                    let is_last = i == trail.len() - 1;
                                    if is_last {
                                        html! {
                                            <li class="breadcrumbs-item breadcrumbs-item--active">
                                                <span class="breadcrumbs-link">{label}</span>
                                            </li>
                                        }
                                    } else if let Some(h) = href {
                                        let rewritten = rewrite_doc_href(h, props.lang.as_str(), props.doc_version.as_str());
                                        html! {
                                            <li class="breadcrumbs-item">
                                                <a class="breadcrumbs-link" href={rewritten}>{label}</a>
                                            </li>
                                        }
                                    } else {
                                        html! {
                                            <li class="breadcrumbs-item">
                                                <span class="breadcrumbs-link">{label}</span>
                                            </li>
                                        }
                                    }
                                })}
                            </ul>
                        </nav>
                    }
                    if has_sidebar && !props.doc_version.is_empty() {
                        <span class="version-badge">
                            {"Version: "}{&props.doc_version}
                        </span>
                    }
                    {copy_md_button}
                    if !props.title.is_empty() {
                        <h1 class="page-title">{&props.title}</h1>
                    }
                    {props.children.clone()}
                    if !edit_url.is_empty() {
                        <div class="edit-page">
                            <a class="edit-page-link" href={edit_url.clone()} target="_blank" rel="noopener noreferrer">
                                <svg viewBox="0 0 24 24" width="16" height="16" class="edit-page-icon">
                                    <path fill="currentColor" d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34a.9959.9959 0 0 0-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                                </svg>
                                {"Edit this page"}
                            </a>
                        </div>
                    }
                    if let Some((prev, next)) = &pagination {
                        <nav class="pagination" aria-label="Docs pages">
                            if let Some((label, href)) = prev {
                                <a class="pagination-link pagination-link--prev" href={href.clone()}>
                                    <span class="pagination-sublabel">{"Previous"}</span>
                                    <span class="pagination-label">{label}</span>
                                </a>
                            } else {
                                <span class="pagination-spacer" />
                            }
                            if let Some((label, href)) = next {
                                <a class="pagination-link pagination-link--next" href={href.clone()}>
                                    <span class="pagination-sublabel">{"Next"}</span>
                                    <span class="pagination-label">{label}</span>
                                </a>
                            }
                        </nav>
                    }
                </main>
                if has_sidebar && !props.toc.is_empty() {
                    <Toc entries={props.toc.clone()} content_ref={content_ref.clone()} />
                }
            </div>
            <Footer />
        </div>
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
            gloo::timers::callback::Timeout::new(100, move || {
                el.scroll_into_view();
            })
            .forget();
        }
    }
}

#[cfg(not(feature = "csr"))]
fn scroll_to_hash(_content_ref: &NodeRef) {}

#[derive(Clone, PartialEq, Properties)]
struct TocProps {
    entries: Vec<TocEntry>,
    content_ref: NodeRef,
}

#[cfg(feature = "csr")]
#[component]
fn Toc(props: &TocProps) -> Html {
    let active_id = use_state(|| Option::<AttrValue>::None);

    {
        let active_id = active_id.clone();
        let entries = props.entries.clone();
        let content_ref = props.content_ref.clone();
        use_effect_with(entries.clone(), move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let content_el = content_ref.cast::<web_sys::Element>();

            let navbar_height: f64 = document
                .query_selector(".navbar")
                .ok()
                .flatten()
                .map(|el| {
                    let html: web_sys::HtmlElement = el.unchecked_into();
                    html.client_height() as f64
                })
                .unwrap_or(60.0);

            let ids: Vec<AttrValue> = entries.iter().map(|e| e.id.clone()).collect();

            let compute = {
                let ids = ids.clone();
                let active_id = active_id.clone();
                let content_el = content_el.clone();
                let window = window.clone();
                move || {
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
                }
            };

            compute();

            let compute_scroll = compute.clone();
            let scroll_listener =
                gloo::events::EventListener::new(&document, "scroll", move |_| compute_scroll());

            let resize_listener =
                gloo::events::EventListener::new(&window, "resize", move |_| compute());

            move || drop((scroll_listener, resize_listener))
        });
    }

    html! {
        <aside class="toc-column">
            <nav class="toc-container">
                <ul class="toc-list">
                    { for props.entries.iter().map(|entry| {
                        let pad = match entry.level {
                            3 => "padding-left:1rem",
                            4 => "padding-left:2rem",
                            _ => "",
                        };
                        let href = format!("#{}", entry.id);
                        let is_active = *active_id == Some(entry.id.clone());
                        html! {
                            <li style={pad}>
                                <a class={classes!("toc-link", is_active.then_some("toc-link--active"))} href={href}>{&entry.text}</a>
                            </li>
                        }
                    })}
                </ul>
            </nav>
        </aside>
    }
}

#[cfg(not(feature = "csr"))]
#[component]
fn Toc(props: &TocProps) -> Html {
    html! {
        <aside class="toc-column">
            <nav class="toc-container">
                <ul class="toc-list">
                    { for props.entries.iter().map(|entry| {
                        let pad = match entry.level {
                            3 => "padding-left:1rem",
                            4 => "padding-left:2rem",
                            _ => "",
                        };
                        let href = format!("#{}", entry.id);
                        html! {
                            <li style={pad}>
                                <a class="toc-link" href={href}>{&entry.text}</a>
                            </li>
                        }
                    })}
                </ul>
            </nav>
        </aside>
    }
}
