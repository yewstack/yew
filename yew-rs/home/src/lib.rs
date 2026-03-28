pub mod not_found;
pub mod search;

use stylist::yew::styled_component;
use yew::prelude::*;
use yew_site_lib::Layout;

pub struct Feature {
    header: &'static str,
    body: &'static str,
    path: &'static str,
}

pub struct HomeStrings {
    pub subtitle: &'static str,
    pub get_started: &'static str,
    pub playground: &'static str,
    pub features_title: &'static str,
    pub learn_more: &'static str,
    pub features: [Feature; 3],
}

pub const STRINGS_EN: HomeStrings = HomeStrings {
    subtitle: "A framework for creating reliable and efficient web applications.",
    get_started: "Get Started",
    playground: "Playground",
    features_title: "Features",
    learn_more: "Learn more",
    features: [
        Feature {
            header: "Component Based",
            body: "Features a component-based framework which makes it easy to create interactive \
                   UIs. Developers who have experience with frameworks like React and Elm should \
                   feel quite at home when using Yew.",
            path: "/docs/concepts/function-components",
        },
        Feature {
            header: "HTML macro",
            body: "Features a macro for declaring interactive HTML with Rust expressions. \
                   Developers who have experience using JSX in React should feel quite at home \
                   when using Yew.",
            path: "/docs/concepts/html",
        },
        Feature {
            header: "Server Side Rendering",
            body: "Features server side rendering for all the SEO and enhancements of \
                   server-rendered app while keeping the feel of an SPA.",
            path: "/docs/advanced-topics/server-side-rendering",
        },
    ],
};

pub const STRINGS_JA: HomeStrings = HomeStrings {
    subtitle: "信頼性が高く効率的な Web アプリケーションを構築するためのフレームワーク。",
    get_started: "はじめる",
    playground: "Playground",
    features_title: "特徴",
    learn_more: "詳しく見る",
    features: [
        Feature {
            header: "コンポーネントベース",
            body: "インタラクティブな UI \
                   を簡単に作成できるコンポーネントベースのフレームワークです。React や Elm \
                   などのフレームワークの経験がある開発者は、Yew をすぐに使いこなせるでしょう。",
            path: "/docs/concepts/function-components",
        },
        Feature {
            header: "HTML マクロ",
            body: "Rust の式を使ってインタラクティブな HTML \
                   を宣言的に記述できるマクロを備えています。React で JSX \
                   を使った経験がある開発者は、Yew をすぐに使いこなせるでしょう。",
            path: "/docs/concepts/html",
        },
        Feature {
            header: "サーバーサイドレンダリング",
            body: "SPA の操作感を維持しながら、サーバーレンダリングによる SEO \
                   やパフォーマンスの向上を実現します。",
            path: "/docs/advanced-topics/server-side-rendering",
        },
    ],
};

pub const STRINGS_ZH_HANS: HomeStrings = HomeStrings {
    subtitle: "用于构建可靠且高效的 Web 应用程序的框架。",
    get_started: "快速开始",
    playground: "Playground",
    features_title: "特性",
    learn_more: "了解更多",
    features: [
        Feature {
            header: "基于组件",
            body: "提供基于组件的框架，轻松创建交互式 UI。有 React 或 Elm 等框架经验的开发者会对 \
                   Yew 感到非常熟悉。",
            path: "/docs/concepts/function-components",
        },
        Feature {
            header: "HTML 宏",
            body: "提供使用 Rust 表达式声明交互式 HTML 的宏。有 React JSX 经验的开发者会对 Yew \
                   感到非常熟悉。",
            path: "/docs/concepts/html",
        },
        Feature {
            header: "服务端渲染",
            body: "支持服务端渲染，在保持 SPA 体验的同时获得 SEO 优化和服务端渲染的性能提升。",
            path: "/docs/advanced-topics/server-side-rendering",
        },
    ],
};

pub const STRINGS_ZH_HANT: HomeStrings = HomeStrings {
    subtitle: "用於建構可靠且高效的 Web 應用程式的框架。",
    get_started: "快速開始",
    playground: "Playground",
    features_title: "特性",
    learn_more: "了解更多",
    features: [
        Feature {
            header: "基於元件",
            body: "提供基於元件的框架，輕鬆建立互動式 UI。有 React 或 Elm 等框架經驗的開發者會對 \
                   Yew 感到非常熟悉。",
            path: "/docs/concepts/function-components",
        },
        Feature {
            header: "HTML 巨集",
            body: "提供使用 Rust 運算式宣告互動式 HTML 的巨集。有 React JSX 經驗的開發者會對 Yew \
                   感到非常熟悉。",
            path: "/docs/concepts/html",
        },
        Feature {
            header: "伺服器端渲染",
            body: "支援伺服器端渲染，在保持 SPA 體驗的同時獲得 SEO \
                   最佳化和伺服器端渲染的效能提升。",
            path: "/docs/advanced-topics/server-side-rendering",
        },
    ],
};

pub fn strings_for_locale(locale: &str) -> &'static HomeStrings {
    match locale {
        "ja" => &STRINGS_JA,
        "zh-Hans" => &STRINGS_ZH_HANS,
        "zh-Hant" => &STRINGS_ZH_HANT,
        _ => &STRINGS_EN,
    }
}

fn lang_prefix(locale: &str) -> String {
    if locale.is_empty() || locale == "en" {
        String::new()
    } else {
        format!("/{locale}")
    }
}

fn version_infix(version_slug: &str) -> String {
    if version_slug.is_empty() {
        String::new()
    } else {
        format!("/{version_slug}")
    }
}

fn docs_href(locale: &str, version_slug: &str, path: &str) -> String {
    let lp = lang_prefix(locale);
    let vi = version_infix(version_slug);
    let rest = path.strip_prefix("/docs").unwrap_or(path);
    format!("{lp}/docs{vi}{rest}")
}

#[derive(Properties, PartialEq)]
struct HomeProps {
    #[prop_or_default]
    pub locale: AttrValue,
    #[prop_or_default]
    pub version_slug: AttrValue,
}

#[styled_component]
fn Hero(props: &HomeProps) -> Html {
    let strings = strings_for_locale(&props.locale);
    let nav_ctx = use_context::<yew_site_lib::NavigationContext>();

    let get_started_href = docs_href(&props.locale, &props.version_slug, "/getting-started");

    html! {
        <div class={css!(padding: 4rem 2rem; text-align: center; @media (max-width: 700px) { padding: 2rem 1rem; })}>
            <div class={css!(display: flex; align-items: center; justify-content: center; gap: calc(var(--h1-font-size) / 2); padding: var(--h1-font-size); @media (max-width: 700px) { flex-direction: column; })}>
                <img class={css!(r#"width: calc(var(--h1-font-size) * 5); height: calc(var(--h1-font-size) * 5); @media (max-width: 700px) { width: calc(var(--h1-font-size) * 3); height: calc(var(--h1-font-size) * 3); }"#)} src="/img/logo.svg" alt="Yew" />
                <h1 class={css!(r#"font-size: calc(var(--h1-font-size) * 3); margin: 0; @media (max-width: 700px) { font-size: calc(var(--h1-font-size) * 1.5); }"#)}>{"Yew"}</h1>
            </div>
            <p class={css!(font-size: 1.25rem; color: var(--color-text-secondary); padding: 1rem; max-width: 600px; margin: 0 auto;)}>
                {strings.subtitle}
            </p>
            <div class={css!(display: flex; gap: 1rem; justify-content: center; margin-top: 2rem; flex-wrap: wrap; @media (max-width: 700px) { flex-direction: column; align-items: center; })}>
                <a class={css!(display: inline-flex; align-items: center; gap: 0.5rem; padding: 1rem 2rem; border-radius: 6px; font-weight: 600; font-size: 1.125rem; cursor: pointer; transition: background 0.2s, color 0.2s, border-color 0.2s; text-decoration: none; background: transparent; color: var(--color-primary); border: 2px solid var(--color-primary); &:hover { background: var(--color-primary); color: #fff; text-decoration: none; })} href={get_started_href.clone()} onclick={{
                    nav_ctx.as_ref().map(|ctx| {
                        let navigate = ctx.navigate.clone();
                        let href = AttrValue::from(get_started_href);
                        Callback::from(move |e: MouseEvent| { navigate.emit((e, href.clone())); })
                    })
                }}>
                    {strings.get_started}
                </a>
                <a class={css!(display: inline-flex; align-items: center; gap: 0.5rem; padding: 1rem 2rem; border-radius: 6px; font-weight: 600; font-size: 1.125rem; cursor: pointer; transition: background 0.2s, color 0.2s, border-color 0.2s; text-decoration: none; background: var(--color-bg-secondary); color: var(--color-text); border: 2px solid var(--color-border); &:hover { background: var(--color-border); text-decoration: none; })} href="https://play.yew.rs" target="_blank" rel="noopener noreferrer">
                    {strings.playground}
                    <svg viewBox="0 0 24 24" width="16" height="16">
                        <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}

#[styled_component]
fn Features(props: &HomeProps) -> Html {
    let strings = strings_for_locale(&props.locale);
    let nav_ctx = use_context::<yew_site_lib::NavigationContext>();

    html! {
        <article class={css!(r#"padding: 3rem 2rem; max-width: 1200px; margin: 0 auto; & h2 { font-size: calc(1.5rem * 1.5); margin-bottom: 1.5rem; }"#)}>
            <h2>{strings.features_title}</h2>
            <section class={css!(display: grid; grid-template-columns: 1fr; gap: 2rem; padding: 1rem 0; @media (min-width: 700px) { grid-template-columns: repeat(3, 1fr); })}>
                for f in strings.features.iter() {
                    <div class={css!(background: var(--color-bg); border: 1px solid var(--color-border); border-radius: 8px; overflow: hidden; transition: box-shadow 0.2s; &:hover { box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1); })}>
                        <div class={css!(padding: 1.25rem 1.25rem 0;)}>
                            <h3 class={css!(margin: 0;)}>{f.header}</h3>
                        </div>
                        <div class={css!(padding: 0.75rem 1.25rem; color: var(--color-text-secondary);)}>
                            <p class={css!(margin: 0;)}>{f.body}</p>
                        </div>
                        <div class={css!(padding: 0 1.25rem 1.25rem;)}>
                            <a class={css!(display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; border-radius: 6px; font-weight: 600; font-size: 1rem; cursor: pointer; transition: background 0.2s, color 0.2s, border-color 0.2s; text-decoration: none; background: var(--color-bg-secondary); color: var(--color-text); border: 2px solid var(--color-border); &:hover { background: var(--color-border); text-decoration: none; })} href={docs_href(&props.locale, &props.version_slug, f.path)} onclick={{
                                nav_ctx.as_ref().map(|ctx| {
                                    let navigate = ctx.navigate.clone();
                                    let href = AttrValue::from(docs_href(&props.locale, &props.version_slug, f.path));
                                    Callback::from(move |e: MouseEvent| { navigate.emit((e, href.clone())); })
                                })
                            }}>
                                {strings.learn_more}
                            </a>
                        </div>
                    </div>
                }
            </section>
        </article>
    }
}

const VERSION_LABELS: &[(&str, &str)] = yew_site_lib::VERSIONS;

fn home_url(locale: &str, version_slug: &str) -> String {
    let lp = lang_prefix(locale);
    let vi = version_infix(version_slug);
    if vi.is_empty() {
        format!("{lp}/")
    } else {
        format!("{lp}{vi}/")
    }
}

pub fn home_html(locale: &'static str, version_slug: &'static str) -> Html {
    html! {
        <>
            <Hero locale={locale} version_slug={version_slug} />
            <Features locale={locale} version_slug={version_slug} />
        </>
    }
}

fn home_page(locale: &'static str, version_slug: &'static str) -> Html {
    let strings = strings_for_locale(locale);
    let lang = if locale == "en" { "" } else { locale };
    let doc_version = VERSION_LABELS
        .iter()
        .find(|(_, slug)| *slug == version_slug)
        .map(|(label, _)| *label)
        .unwrap_or("Next");
    let current_path = home_url(locale, version_slug);
    html! {
        <Layout
            title=""
            description={strings.subtitle}
            full_width=true
            lang={lang}
            doc_version={doc_version}
            active_sidebar_path={current_path}
        >
            <Hero locale={locale} version_slug={version_slug} />
            <Features locale={locale} version_slug={version_slug} />
        </Layout>
    }
}

macro_rules! home_component {
    ($name:ident, $locale:expr, $version_slug:expr) => {
        #[component]
        pub fn $name() -> Html {
            home_page($locale, $version_slug)
        }
    };
}

// Main home pages (latest stable, at /, /ja/, etc.)
home_component!(Page, "en", "");
home_component!(PageJa, "ja", "");
home_component!(PageZhHans, "zh-Hans", "");
home_component!(PageZhHant, "zh-Hant", "");

// Next
home_component!(PageNext, "en", "next");
home_component!(PageJaNext, "ja", "next");
home_component!(PageZhHansNext, "zh-Hans", "next");
home_component!(PageZhHantNext, "zh-Hant", "next");

// 0.22
home_component!(PageV022, "en", "0.22");
home_component!(PageJaV022, "ja", "0.22");
home_component!(PageZhHansV022, "zh-Hans", "0.22");
home_component!(PageZhHantV022, "zh-Hant", "0.22");

// 0.21
home_component!(PageV021, "en", "0.21");
home_component!(PageJaV021, "ja", "0.21");
home_component!(PageZhHansV021, "zh-Hans", "0.21");
home_component!(PageZhHantV021, "zh-Hant", "0.21");

// 0.20
home_component!(PageV020, "en", "0.20");
home_component!(PageJaV020, "ja", "0.20");
home_component!(PageZhHansV020, "zh-Hans", "0.20");
home_component!(PageZhHantV020, "zh-Hant", "0.20");

#[cfg(feature = "ssr")]
pub async fn render_search_and_404() -> Vec<(&'static str, String, String)> {
    vec![
        yew_site_lib::render_page!("/search", search::Page),
        yew_site_lib::render_page!("/404", not_found::Page),
    ]
}

#[cfg(feature = "ssr")]
pub async fn render_pages() -> Vec<(&'static str, String, String)> {
    let mut pages = render_search_and_404().await;
    pages.extend(vec![
        yew_site_lib::render_page!("/", Page),
        yew_site_lib::render_page!("/ja/", PageJa),
        yew_site_lib::render_page!("/zh-Hans/", PageZhHans),
        yew_site_lib::render_page!("/zh-Hant/", PageZhHant),
        yew_site_lib::render_page!("/next/", PageNext),
        yew_site_lib::render_page!("/ja/next/", PageJaNext),
        yew_site_lib::render_page!("/zh-Hans/next/", PageZhHansNext),
        yew_site_lib::render_page!("/zh-Hant/next/", PageZhHantNext),
        yew_site_lib::render_page!("/0.22/", PageV022),
        yew_site_lib::render_page!("/ja/0.22/", PageJaV022),
        yew_site_lib::render_page!("/zh-Hans/0.22/", PageZhHansV022),
        yew_site_lib::render_page!("/zh-Hant/0.22/", PageZhHantV022),
        yew_site_lib::render_page!("/0.21/", PageV021),
        yew_site_lib::render_page!("/ja/0.21/", PageJaV021),
        yew_site_lib::render_page!("/zh-Hans/0.21/", PageZhHansV021),
        yew_site_lib::render_page!("/zh-Hant/0.21/", PageZhHantV021),
        yew_site_lib::render_page!("/0.20/", PageV020),
        yew_site_lib::render_page!("/ja/0.20/", PageJaV020),
        yew_site_lib::render_page!("/zh-Hans/0.20/", PageZhHansV020),
        yew_site_lib::render_page!("/zh-Hant/0.20/", PageZhHantV020),
    ]);
    pages
}
