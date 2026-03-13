use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {
    #[prop_or_default]
    pub active: AttrValue,
    #[prop_or_default]
    pub doc_version: AttrValue,
    #[prop_or_default]
    pub lang: AttrValue,
    #[prop_or_default]
    pub current_path: AttrValue,
}

const VERSION_SLUGS: &[(&str, &str)] = &[
    ("Next", ""),
    ("0.23", "0.23"),
    ("0.22", "0.22"),
    ("0.21", "0.21"),
    ("0.20", "0.20"),
];

const LANGUAGES: &[(&str, &str)] = &[
    ("English", ""),
    ("\u{65e5}\u{672c}\u{8a9e}", "ja"),
    ("\u{7b80}\u{4f53}\u{4e2d}\u{6587}", "zh-Hans"),
    ("\u{7e41}\u{9ad4}\u{4e2d}\u{6587}", "zh-Hant"),
];

fn lang_prefix(lang: &str) -> String {
    if lang.is_empty() {
        String::new()
    } else {
        format!("/{lang}")
    }
}

#[styled_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let mobile_open = use_state(|| false);
    let version_open = use_state(|| false);
    let lang_open = use_state(|| false);

    let toggle_mobile = {
        let mobile_open = mobile_open.clone();
        Callback::from(move |_: MouseEvent| {
            mobile_open.set(!*mobile_open);
        })
    };

    let toggle_version = {
        let version_open = version_open.clone();
        Callback::from(move |_: MouseEvent| {
            version_open.set(!*version_open);
        })
    };

    let toggle_lang = {
        let lang_open = lang_open.clone();
        Callback::from(move |_: MouseEvent| {
            lang_open.set(!*lang_open);
        })
    };

    use_effect_with((), |_| {
        init_docsearch();
        || {}
    });

    let has_doc_version = !props.doc_version.is_empty();

    let nav_items: &[(&str, &str, &str)] = &[
        ("Docs", "/docs/getting-started", "/docs/"),
        ("Tutorial", "/docs/tutorial", "/docs/tutorial"),
        ("Community", "/community/awesome", "/community/"),
        ("Blog", "/blog", "/blog"),
    ];

    let active_nav_label = {
        let path_matches = |prefix: &str| -> bool {
            if props.lang.is_empty() {
                props.current_path.starts_with(prefix)
            } else {
                let lang_prefix = format!("/{}{}", props.lang.as_str(), prefix);
                props.current_path.starts_with(&lang_prefix)
            }
        };
        nav_items
            .iter()
            .filter(|(_, _, prefix)| path_matches(prefix))
            .max_by_key(|(_, _, prefix)| prefix.len())
            .map(|(label, ..)| *label)
            .unwrap_or(props.active.as_str())
    };

    let current_lang_label = LANGUAGES
        .iter()
        .find(|(_, code)| *code == props.lang.as_str())
        .map(|(label, _)| *label)
        .unwrap_or("English");

    let style = css!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        height: var(--navbar-height);
        background: var(--color-bg);
        border-bottom: 1px solid var(--color-border);
        z-index: 100;
        display: flex;
        flex-direction: column;

        .inner {
            display: flex;
            align-items: center;
            justify-content: space-between;
            height: var(--navbar-height);
            padding: 0 1rem;
            max-width: 1440px;
            width: 100%;
            margin: 0 auto;
        }

        .items {
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .brand {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            color: var(--color-text);
            font-weight: 700;
            margin-right: 1rem;
            text-decoration: none;
        }

        .brand:hover {
            text-decoration: none;
        }

        .logo {
            height: 2rem;
            width: 2rem;
        }

        .title {
            font-size: 1.25rem;
        }

        .link {
            color: var(--color-text);
            padding: 0.5rem 0.75rem;
            font-size: 0.875rem;
            font-weight: 500;
            display: inline-flex;
            align-items: center;
            gap: 0.25rem;
        }

        .link:hover {
            color: var(--color-primary);
            text-decoration: none;
        }

        .link--active {
            color: var(--color-primary);
        }

        .toggle {
            display: none;
            background: none;
            border: none;
            cursor: pointer;
            color: var(--color-text);
            padding: 0.5rem;
        }

        .mobile-menu {
            display: flex;
            flex-direction: column;
            padding: 0.5rem 1rem 1rem;
            background: var(--color-bg);
            border-bottom: 1px solid var(--color-border);
            max-height: calc(100vh - 60px);
            overflow-y: auto;
        }

        .mobile-menu .link {
            padding: 0.75rem 0;
        }

        .dropdown {
            position: relative;
            margin-right: 0.5rem;
        }

        .dropdown-btn {
            display: flex;
            align-items: center;
            gap: 0.25rem;
            background: none;
            border: 1px solid var(--color-border);
            border-radius: 4px;
            padding: 0.25rem 0.5rem;
            font-size: 0.875rem;
            color: var(--color-text);
            cursor: pointer;
            font-family: inherit;
        }

        .dropdown-btn:hover {
            border-color: var(--color-primary);
        }

        .dropdown-caret {
            transition: transform 0.2s;
        }

        .dropdown--open .dropdown-caret {
            transform: rotate(180deg);
        }

        .dropdown-menu {
            position: absolute;
            top: 100%;
            left: 0;
            margin-top: 0.25rem;
            list-style: none;
            padding: 0.25rem 0;
            background: var(--color-bg);
            border: 1px solid var(--color-border);
            border-radius: 4px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
            z-index: 200;
            min-width: 80px;
        }

        .dropdown-item {
            display: block;
            padding: 0.375rem 0.75rem;
            font-size: 0.875rem;
            color: var(--color-text);
            text-decoration: none;
            white-space: nowrap;
        }

        .dropdown-item:hover {
            background: var(--color-bg-offset);
            color: var(--color-primary);
        }

        .dropdown-item--active {
            font-weight: 600;
            color: var(--color-primary);
        }

        .ext-icon {
            margin-left: 0.25rem;
        }

        @media (max-width: 700px) {
            .items .link {
                display: none;
            }
            .items .dropdown {
                display: none;
            }
            .toggle {
                display: block;
            }
        }

        .mobile-divider {
            height: 1px;
            background: var(--color-border);
            margin: 0.25rem 0;
        }

        .mobile-label {
            font-size: 0.75rem;
            font-weight: 600;
            color: var(--color-text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            padding: 0.5rem 0 0.25rem;
            display: block;
        }

        .theme-toggle {
            background: none;
            border: none;
            cursor: pointer;
            color: var(--color-text);
            padding: 0.25rem;
            display: flex;
            align-items: center;
            margin-right: 0.5rem;
        }

        .theme-toggle:hover {
            color: var(--color-primary);
        }

        .theme-toggle .sun {
            display: var(--theme-sun-display);
        }

        .theme-toggle .moon {
            display: var(--theme-moon-display);
        }

        .theme-toggle .system-icon {
            display: var(--theme-system-display);
        }

        .search {
            margin-right: 0.5rem;
        }
    "#
    );

    html! {
        <nav class={style}>
            <div class="inner">
                <div class="items">
                    <a class="brand" href="/">
                        <img class="logo" src="/img/logo.svg" alt="Yew" />
                        <strong class="title">{"Yew"}</strong>
                    </a>
                    if has_doc_version {
                        <div class={classes!("dropdown", (*version_open).then_some("dropdown--open"))}>
                            <button class="dropdown-btn" onclick={toggle_version}>
                                {&props.doc_version}
                                <svg class="dropdown-caret" viewBox="0 0 24 24" width="12" height="12">
                                    <path fill="currentColor" d="M7 10l5 5 5-5z"/>
                                </svg>
                            </button>
                            if *version_open {
                                <ul class="dropdown-menu">
                                    { for VERSION_SLUGS.iter().map(|(label, slug)| {
                                        let is_active = *label == props.doc_version.as_str();
                                        let href = compute_version_url(
                                            props.current_path.as_str(),
                                            props.lang.as_str(),
                                            props.doc_version.as_str(),
                                            slug,
                                        );
                                        html! {
                                            <li>
                                                <a
                                                    class={classes!("dropdown-item", is_active.then_some("dropdown-item--active"))}
                                                    href={href}
                                                >
                                                    {label}
                                                </a>
                                            </li>
                                        }
                                    })}
                                </ul>
                            }
                        </div>
                        <div class={classes!("dropdown", (*lang_open).then_some("dropdown--open"))}>
                            <button class="dropdown-btn" onclick={toggle_lang}>
                                {current_lang_label}
                                <svg class="dropdown-caret" viewBox="0 0 24 24" width="12" height="12">
                                    <path fill="currentColor" d="M7 10l5 5 5-5z"/>
                                </svg>
                            </button>
                            if *lang_open {
                                <ul class="dropdown-menu">
                                    { for LANGUAGES.iter().map(|(label, code)| {
                                        let is_active = *code == props.lang.as_str();
                                        let href = compute_lang_url(
                                            props.current_path.as_str(),
                                            props.lang.as_str(),
                                            props.doc_version.as_str(),
                                            code,
                                        );
                                        html! {
                                            <li>
                                                <a
                                                    class={classes!("dropdown-item", is_active.then_some("dropdown-item--active"))}
                                                    href={href}
                                                >
                                                    {label}
                                                </a>
                                            </li>
                                        }
                                    })}
                                </ul>
                            }
                        </div>
                    }
                    { for nav_items.iter().map(|(label, href, _)| {
                        let active_class = if *label == active_nav_label {
                            "link link--active"
                        } else {
                            "link"
                        };
                        html! {
                            <a class={active_class} href={*href}>{label}</a>
                        }
                    })}
                    <a class="link" href="https://play.yew.rs" target="_blank" rel="noopener noreferrer">
                        {"Playground"}
                        <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                            <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                        </svg>
                    </a>
                    <a class="link" href="https://docs.rs/yew" target="_blank" rel="noopener noreferrer">
                        {"API"}
                        <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                            <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                        </svg>
                    </a>
                </div>
                <div class="items items--right">
                    <div class="search" id="docsearch" />
                    <button class="theme-toggle" onclick={Callback::from(|_: MouseEvent| toggle_theme())} aria-label="Toggle theme">
                        <svg class="sun" viewBox="0 0 24 24" width="20" height="20">
                            <path fill="currentColor" d="M12 18a6 6 0 1 1 0-12 6 6 0 0 1 0 12zm0-2a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM11 1h2v3h-2V1zm0 19h2v3h-2v-3zM3.515 4.929l1.414-1.414L7.05 5.636 5.636 7.05 3.515 4.93zM16.95 18.364l1.414-1.414 2.121 2.121-1.414 1.414-2.121-2.121zm2.121-14.85l1.414 1.415-2.121 2.121-1.414-1.414 2.121-2.121zM5.636 16.95l1.414 1.414-2.121 2.121-1.414-1.414 2.121-2.121zM23 11v2h-3v-2h3zM4 11v2H1v-2h3z"/>
                        </svg>
                        <svg class="moon" viewBox="0 0 24 24" width="20" height="20">
                            <path fill="currentColor" d="M10 7a7 7 0 0 0 12 4.9v.1c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2h.1A6.979 6.979 0 0 0 10 7zm-6 5a8 8 0 0 0 15.062 3.762A9 9 0 0 1 8.238 4.938 7.999 7.999 0 0 0 4 12z"/>
                        </svg>
                        <svg class="system-icon" viewBox="0 0 24 24" width="20" height="20">
                            <path fill="currentColor" d="M4 6h16v10H4V6zm-2 0a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6zm4 14h12v2H6v-2z"/>
                        </svg>
                    </button>
                    <a class="link" href="https://github.com/yewstack/yew" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                        <svg viewBox="0 0 24 24" width="24" height="24">
                            <path fill="currentColor" d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
                        </svg>
                    </a>
                    <button
                        class={classes!("toggle", (*mobile_open).then_some("toggle--active"))}
                        onclick={toggle_mobile}
                        aria-label="Toggle navigation"
                    >
                        <svg viewBox="0 0 30 30" width="30" height="30">
                            <path stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"/>
                        </svg>
                    </button>
                </div>
            </div>
            if *mobile_open {
                <div class="mobile-menu">
                    { for nav_items.iter().map(|(label, href, _)| {
                        html! {
                            <a class="link" href={*href}>{label}</a>
                        }
                    })}
                    <a class="link" href="https://play.yew.rs" target="_blank" rel="noopener noreferrer">
                        {"Playground"}
                    </a>
                    <a class="link" href="https://docs.rs/yew" target="_blank" rel="noopener noreferrer">
                        {"API"}
                    </a>
                    if has_doc_version {
                        <div class="mobile-divider" />
                        <span class="mobile-label">{"Version"}</span>
                        { for VERSION_SLUGS.iter().map(|(label, slug)| {
                            let is_active = *label == props.doc_version.as_str();
                            let href = compute_version_url(
                                props.current_path.as_str(),
                                props.lang.as_str(),
                                props.doc_version.as_str(),
                                slug,
                            );
                            html! {
                                <a class={classes!("link", is_active.then_some("link--active"))} href={href}>{label}</a>
                            }
                        })}
                        <div class="mobile-divider" />
                        <span class="mobile-label">{"Language"}</span>
                        { for LANGUAGES.iter().map(|(label, code)| {
                            let is_active = *code == props.lang.as_str();
                            let href = compute_lang_url(
                                props.current_path.as_str(),
                                props.lang.as_str(),
                                props.doc_version.as_str(),
                                code,
                            );
                            html! {
                                <a class={classes!("link", is_active.then_some("link--active"))} href={href}>{label}</a>
                            }
                        })}
                    }
                </div>
            }
        </nav>
    }
}

fn compute_version_url(
    current_path: &str,
    current_lang: &str,
    current_version: &str,
    target_slug: &str,
) -> String {
    let prefix = lang_prefix(current_lang);

    let without_lang = if current_lang.is_empty() {
        current_path
    } else {
        let lp = lang_prefix(current_lang);
        current_path.strip_prefix(&lp).unwrap_or(current_path)
    };

    let without_docs = without_lang
        .strip_prefix("/docs/")
        .unwrap_or("getting-started");

    let current_slug = VERSION_SLUGS
        .iter()
        .find(|(label, _)| *label == current_version)
        .map(|(_, slug)| *slug)
        .unwrap_or("");

    let page_path = if current_slug.is_empty() {
        without_docs
    } else {
        without_docs
            .strip_prefix(current_slug)
            .and_then(|s| s.strip_prefix('/'))
            .unwrap_or(without_docs)
    };

    if target_slug.is_empty() {
        format!("{prefix}/docs/{page_path}")
    } else {
        format!("{prefix}/docs/{target_slug}/{page_path}")
    }
}

#[cfg(feature = "csr")]
fn toggle_theme() {
    let document = gloo::utils::document();
    if let Some(html) = document.document_element() {
        let choice = html.get_attribute("data-theme-choice").unwrap_or_default();
        let (new_choice, new_theme) = match choice.as_str() {
            "light" => ("dark", "dark"),
            "dark" => ("system", if is_dark_preferred() { "dark" } else { "light" }),
            _ => ("light", "light"),
        };
        let _ = html.set_attribute("data-theme", new_theme);
        let _ = html.set_attribute("data-theme-choice", new_choice);
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            let _ = storage.set_item("theme", new_choice);
        }
    }
}

#[cfg(feature = "csr")]
fn is_dark_preferred() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme:dark)").ok().flatten())
        .map(|mq| mq.matches())
        .unwrap_or(false)
}

#[cfg(not(feature = "csr"))]
fn toggle_theme() {}

#[cfg(feature = "csr")]
fn init_docsearch() {
    use wasm_bindgen::prelude::*;

    let document = gloo::utils::document();
    let script = document
        .create_element("script")
        .expect("failed to create script element");
    script
        .set_attribute("src", "https://cdn.jsdelivr.net/npm/@docsearch/js@3")
        .expect("failed to set src");
    let onload = Closure::once_into_js(|| {
        let _ = js_sys::eval(
            "docsearch({appId:'F8S2ICRD2T',apiKey:'6a9cd0bf0d86b8d643b5e609e7755248',indexName:'\
             yew-rs',container:'#docsearch'})",
        );
    });
    script
        .add_event_listener_with_callback("load", onload.unchecked_ref())
        .expect("failed to add load listener");
    document
        .head()
        .expect("no head element")
        .append_child(&script)
        .expect("failed to append script");
}

#[cfg(not(feature = "csr"))]
fn init_docsearch() {}

fn compute_lang_url(
    current_path: &str,
    current_lang: &str,
    current_version: &str,
    target_lang: &str,
) -> String {
    let target_prefix = lang_prefix(target_lang);

    let without_lang = if current_lang.is_empty() {
        current_path
    } else {
        let lp = lang_prefix(current_lang);
        current_path.strip_prefix(&lp).unwrap_or(current_path)
    };

    let page_path = without_lang
        .strip_prefix("/docs/")
        .unwrap_or("getting-started");

    let current_slug = VERSION_SLUGS
        .iter()
        .find(|(label, _)| *label == current_version)
        .map(|(_, slug)| *slug)
        .unwrap_or("");

    if current_slug.is_empty() {
        format!("{target_prefix}/docs/{page_path}")
    } else {
        format!("{target_prefix}/docs/{current_slug}/{page_path}")
    }
}
